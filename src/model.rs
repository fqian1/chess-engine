use burn::{
    nn::{
        Embedding, EmbeddingConfig, Linear, LinearConfig,
        transformer::{TransformerEncoder, TransformerEncoderConfig, TransformerEncoderInput},
    },
    prelude::*,
    tensor::{activation::log_softmax, backend::AutodiffBackend},
    train::{ClassificationOutput, InferenceStep, TrainOutput, TrainStep},
};

use crate::data::ChessBatch;

// 2 pass encoder: select from sq, populate plane 14, select to square
#[derive(Module, Debug)]
pub struct ChessTransformer<B: Backend> {
    piece_encoder: Linear<B>, // 64 x 14 (12 piece plane + en pasant plane + selected sq plane)
    meta_encoder: Linear<B>,  // This is just castling rights and 50 move counter (4 1-hot + 1 scalar)
    coordinates: Tensor<B, 2, Int>,
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
        let coordinates = Tensor::arange(0..8, device).unsqueeze_dim(0);
        ChessTransformer {
            piece_encoder: LinearConfig::new(14, self.d_model).init(device),
            meta_encoder: LinearConfig::new(5, self.d_model).init(device),
            coordinates,
            pos_embedding_x: EmbeddingConfig::new(8, self.d_model).init(device),
            pos_embedding_y: EmbeddingConfig::new(8, self.d_model).init(device),
            transformer: TransformerEncoderConfig::new(self.d_model, self.d_ff, self.n_heads, self.n_layers)
                .with_dropout(0.0)
                .init(device),
            policy: LinearConfig::new(self.d_model, 1).init(device),
            value: LinearConfig::new(self.d_model, 3).init(device),
            d_model: self.d_model,
        }
    }
}

impl<B: AutodiffBackend> TrainStep for ChessTransformer<B> {
    type Input = ChessBatch<B>;
    type Output = ClassificationOutput<B>;
    fn step(&self, batch: ChessBatch<B>) -> TrainOutput<ClassificationOutput<B>> {
        let item = self.forward_classification(batch);
        TrainOutput::new(self, item.loss.backward(), item)
    }
}

impl<B: Backend> InferenceStep for ChessTransformer<B> {
    type Input = ChessBatch<B>;
    type Output = ClassificationOutput<B>;
    fn step(&self, batch: ChessBatch<B>) -> ClassificationOutput<B> {
        self.forward_classification(batch)
    }
}

impl<B: Backend> ChessTransformer<B> {
    // this is just straight logits no softmax yet
    pub fn forward(&self, board: Tensor<B, 3>, meta: Tensor<B, 2>) -> (Tensor<B, 2>, Tensor<B, 2>) {
        let [batch_size, _seq_len, _] = board.dims();

        // batchsize x 64 x d_model
        let mut x = self.piece_encoder.forward(board);

        // positional encodings
        let x_emb: Tensor<B, 4> = self.pos_embedding_x.forward(self.coordinates.clone()).unsqueeze_dim(1);
        let y_emb: Tensor<B, 4> = self.pos_embedding_y.forward(self.coordinates.clone()).unsqueeze_dim(2);
        let pos2d = x_emb + y_emb;
        let pos_flat = pos2d.reshape([1, 64, self.d_model]);
        x = x + pos_flat;

        // batchsize x d_model -> batchsize x 64 x d_model
        let meta_x = self.meta_encoder.forward(meta).unsqueeze_dim(1);
        let x = Tensor::cat(vec![x, meta_x], 1);

        let x = self.transformer.forward(TransformerEncoderInput::new(x));

        // batchsize x 1 x d_model -> batch_size x 3
        let value_latent = x.clone().slice([0..batch_size, 64..65]).squeeze_dim(1);
        let value = self.value.forward(value_latent);

        // batch_size x 64 x d_model -> batch_size x 64
        let board_latent = x.slice([0..batch_size, 0..64]);
        let policy = self.policy.forward(board_latent).squeeze_dim(2);

        (policy, value)
    }

    pub fn forward_classification(&self, batch: ChessBatch<B>) -> ClassificationOutput<B> {
        let [batch_size, _, _] = batch.boards.dims();
        let (policy_pred, value_pred) = self.forward(batch.boards.clone(), batch.metas);
        let loss = self.calculate_loss(
            policy_pred.clone(),
            value_pred.clone(),
            batch.policy_targets.clone(),
            batch.value_targets.clone(),
            0.8,
        );
        let target_indices = batch.policy_targets.argmax(1).reshape([batch_size]);
        ClassificationOutput::new(loss, policy_pred, target_indices)
    }

    // tunable hyperparameter: weight of policy loss vs value loss
    fn calculate_loss(
        &self,
        policy_pred: Tensor<B, 2>,
        value_pred: Tensor<B, 2>,
        target_policy: Tensor<B, 2>,
        target_value: Tensor<B, 2>,
        ratio: f32,
    ) -> Tensor<B, 1> {
        // Kl divergence
        let policy_probs = log_softmax(policy_pred, 1);
        let policy_loss = (target_policy * policy_probs).sum_dim(1).mean().neg();

        let value_probs = log_softmax(value_pred, 1);
        let value_loss = (target_value * value_probs).sum_dim(1).mean().neg();

        if ratio >= 1.0 || ratio <= 0.0 {
            return policy_loss + value_loss;
        }
        (policy_loss * ratio) + (value_loss * (1.0 - ratio))
    }
}
