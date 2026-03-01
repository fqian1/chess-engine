use burn::{
    nn::{
        Embedding, EmbeddingConfig, Linear, LinearConfig,
        transformer::{TransformerEncoder, TransformerEncoderConfig, TransformerEncoderInput},
    },
    prelude::*,
};

// 2 pass encoder: select from sq, populate plane 14, select to square
#[derive(Module, Debug)]
pub struct ChessTransformer<B: Backend> {
    piece_encoder: Linear<B>, // 64 x 14 (12 piece plane + en pasant plane + selected sq plane)
    meta_encoder: Linear<B>,  // This is just castling rights and 50 move counter (4 1-hot + 1 scalar)
    pos_embedding_x: Embedding<B>,
    pos_embedding_y: Embedding<B>,
    transformer: TransformerEncoder<B>,
    policy: Linear<B>, // Just pick one square
    value: Linear<B>,
    d_model: usize,
}

#[derive(Config, Debug)]
pub struct ChessTransformerConfig {
    d_model:  usize, // Token dimensions
    n_heads:  usize,
    d_ff:     usize, // height x width
    n_layers: usize, // depth
}

impl ChessTransformerConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> ChessTransformer<B> {
        ChessTransformer {
            piece_encoder: LinearConfig::new(14, self.d_model).init(device),
            meta_encoder: LinearConfig::new(5, self.d_model).init(device),
            pos_embedding_x: EmbeddingConfig::new(8, self.d_model).init(device),
            pos_embedding_y: EmbeddingConfig::new(8, self.d_model).init(device),
            transformer: TransformerEncoderConfig::new(self.d_model, self.d_ff, self.n_heads, self.n_layers)
                .with_dropout(0.0)
                .init(device),
            policy: LinearConfig::new(self.d_model, 1).init(device),
            value: LinearConfig::new(self.d_model, 1).init(device),
            d_model: self.d_model,
        }
    }
}

impl<B: Backend> ChessTransformer<B> {
    pub fn forward(&self, board: Tensor<B, 3>, meta: Tensor<B, 2>) -> (Tensor<B, 2>, Tensor<B, 2>) {
        let [batch_size, seq_len, _] = board.dims();

        // batchsize x 64 x d_model
        let mut x = self.piece_encoder.forward(board);

        // positional encodings
        let coords = Tensor::arange(0..8, &x.device()).unsqueeze_dim(0);
        let x_emb: Tensor<B, 4> = self.pos_embedding_x.forward(coords.clone()).unsqueeze_dim(1);
        let y_emb: Tensor<B, 4> = self.pos_embedding_y.forward(coords).unsqueeze_dim(2);
        let t1 = x_emb.expand([1, 8, 8, self.d_model]);
        let t2 = y_emb.expand([1, 8, 8, self.d_model]);
        let pos2d = t1 + t2;
        let pos_flat = pos2d.reshape([1, 64, self.d_model]);
        x = x + pos_flat;

        // batchsize x d_model -> batchsize x 64 x d_model
        let meta_x = self.meta_encoder.forward(meta).unsqueeze_dim(1);
        let x = Tensor::cat(vec![x, meta_x], 1);

        let x = self.transformer.forward(TransformerEncoderInput::new(x));

        // batchsize x f32
        let value_latent = x.clone().slice([0..batch_size, 64..65]).squeeze_dim(1);
        let value = self.value.forward(value_latent).tanh();

        // batch_size x 64 x d_model -> bach_size x 64 x f32
        let board_latent = x.slice([0..batch_size, 0..64]);
        let policy = self.policy.forward(board_latent).squeeze_dim(2);

        (policy, value)
    }
}
