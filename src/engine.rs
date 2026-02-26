use burn::{
    module::AutodiffModule,
    nn::{
        Embedding, EmbeddingConfig, Linear, LinearConfig,
        transformer::{TransformerEncoder, TransformerEncoderConfig, TransformerEncoderInput},
    },
    prelude::*,
    tensor::{Scalar, backend::AutodiffBackend},
};

use crate::{Bitboard, CastlingRights, ChessPiece, chess_square};
use crate::{ChessBoard, castling, chess_board, chess_game::GameStateEntry};
use crate::{ChessGame, ChessSquare, Color};

#[derive(Config, Debug)]
pub struct ChessTransformerConfig {
    pub d_model: usize,
    pub n_heads: usize,
    pub n_layers: usize,
    pub d_ff: usize,
    pub dropout: f64,
    #[config(default = 64)]
    pub chess_squares: usize,
    #[config(default = 14)]
    pub vocab_size: usize,
}

impl ChessTransformerConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> ChessTransformerModel<B> {
        let pos_encoding_x = EmbeddingConfig::new(self.chess_squares, self.d_model).init(device);
        let pos_encoding_y = EmbeddingConfig::new(self.chess_squares, self.d_model).init(device);

        let board_projection = LinearConfig::new(self.vocab_size, self.d_model).init(device);
        let meta_projection = LinearConfig::new(6, self.d_model).init(device);

        let encoder = TransformerEncoderConfig::new(self.d_model, self.d_ff, self.n_heads, self.n_layers)
            .with_dropout(self.dropout)
            .init(device);

        let policy_head = LinearConfig::new(self.d_model, 64).init(device);
        let value_head = LinearConfig::new(self.d_model, 1).init(device);
        let moves_head = LinearConfig::new(self.d_model, 10).init(device);

        ChessTransformerModel {
            board_projection,
            meta_projection,
            pos_encoding_x,
            pos_encoding_y,
            encoder,
            policy_head,
            value_head,
            moves_head,
            d_model: self.d_model,
        }
    }
}

#[derive(Module, Debug)]
pub struct ChessTransformerModel<B: Backend> {
    board_projection: Linear<B>,
    meta_projection: Linear<B>,
    pos_encoding_x: Embedding<B>,
    pos_encoding_y: Embedding<B>,
    encoder: TransformerEncoder<B>,
    policy_head: Linear<B>,
    value_head: Linear<B>,
    moves_head: Linear<B>,
    d_model: usize,
}

impl<B: Backend> ChessTransformerModel<B> {
    pub fn forward(&self, board: Tensor<B, 3>, meta: Tensor<B, 2>) -> (Tensor<B, 2>, Tensor<B, 2>, Tensor<B, 2>) {
        let batch_size = board.dims()[0];

        let mut x = self.board_projection.forward(board.clone());

        let pos_indices = Tensor::arange(0..64, &board.device()).reshape([1, 64]).repeat(&[batch_size, 1]);
        x = x + self.pos_encoding.forward(pos_indices);

        let meta_x = self.meta_projection.forward(meta).unsqueeze_dim(1);
        let x = Tensor::cat(vec![x, meta_x], 1);

        let x = self.encoder.forward(TransformerEncoderInput::new(x));

        let global_latent = x.clone().slice([0..batch_size, 64..65]).squeeze();

        let value = self.value_head.forward(global_latent.clone()).tanh();
        let moves_left = self.moves_head.forward(global_latent);

        let board_latent = x.slice([0..batch_size, 0..64]).flatten(1, 1);
        let policy = self.policy_head.forward(board_latent);

        (policy, value, moves_left)
    }
    // pub fn loss(
    //     &self,
    //     pred: (Tensor<B, 3>, Tensor<B, 2>, Tensor<B, 2>),
    //     training_data: TrainingDataEntry,
    // ) -> (Tensor<B, 3>, Tensor<B, 2>, Tensor<B, 2>) {
    //     // policy bce, value mse, [value;3] mse
    // }
    // pub fn backprop(&self, training_data: TrainingDataEntry)
}
