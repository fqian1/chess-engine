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

#[derive(Debug, Clone)]
pub struct MoveData {
    policy:     ChessSquare,
    value:      f32,
    moves_left: u32,
}

#[derive(Debug, Clone)]
pub struct GameData {
    game_state: GameStateEntry,
    from_sq:    Option<ChessSquare>,
    to_sq:      Option<ChessSquare>,
}

#[derive(Clone)]
pub struct GameTimeLine {
    moves:  Vec<MoveData>,
    games:  Vec<GameData>,
    result: f32,
}
// let batch = [GameTimeLine; batch_size];

impl GameData {
    pub fn to_tensor<B: Backend>(&self, device: &Device<B>) -> (Tensor<B, 2>, Tensor<B, 1>) {
        let mut data = [[0f32; 64]; 14];

        let (chess_board, castling_rights, ep_sq) = if self.game_state.side_to_move == Color::White {
            (self.game_state.chessboard.pieces, self.game_state.castling_rights, self.game_state.en_passant)
        } else {
            (
                self.game_state.chessboard.flip_board(),
                self.game_state.castling_rights.flip_perspective(),
                self.game_state.en_passant.map(|x| x.square_opposite()),
            )
        };
        let mut flat = [Bitboard::default(); 12];

        flat[..6].copy_from_slice(&chess_board[0]);
        flat[6..].copy_from_slice(&chess_board[1]);

        for i in 0..12 {
            data[i] = flat[i].to_f32();
        }

        if let Some(square) = ep_sq {
            data[12][square.0 as usize] = 1.0;
        }

        if let Some(square) = self.from_sq {
            data[13][square.0 as usize] = 1.0;
        }

        if let Some(square) = self.to_sq {
            data[13][square.0 as usize] = 1.0;
        }

        let t1 = Tensor::from_data(data, device);

        let mut data = [0.0f32; 4];
        for i in 0..4 {
            data[i] = (castling_rights.0 >> i & 1).into();
        }

        let t2 = Tensor::from_data(data, device);
        (t1, t2)
    }
}

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
        let pos_encoding = EmbeddingConfig::new(self.chess_squares, self.d_model).init(device);

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
            pos_encoding,
            encoder,
            policy_head,
            value_head,
            moves_head,
            d_model: self.d_model,
        }
    }
}

// 2 pass encoder architecture:
// inputs:
// one hot (8x8x14): chessboard + from_sq + ep_square. this gets positional encodings.
// tensor 1x6: castling rights one hot, 50 move scalar, repition count scalar.
// outputs:
// 8x8 (bce), pick a square (to or from square)
// scalar (mse)
// 1x3 tensor for promotion (queen auto promote)
// 1x10 tensor moves left (buckets)
// resnet muzero thing?
// 2 pass encoder: first pass generate from square, populate from_sq bitboard, second pass generate
// to square, create distribution over 64*2 possible/impossible moves. or just evaluate top 10
// from, to squares or something or keep searching until a valid move made.

#[derive(Module, Debug)]
pub struct ChessTransformerModel<B: Backend> {
    board_projection: Linear<B>,
    meta_projection: Linear<B>,
    pos_encoding: Embedding<B>,
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
    pub fn loss(
        &self,
        pred: (Tensor<B, 3>, Tensor<B, 2>, Tensor<B, 2>),
        training_data: TrainingDataEntry,
    ) -> (Tensor<B, 3>, Tensor<B, 2>, Tensor<B, 2>) {
        // policy bce, value mse, [value;3] mse
    }
    // pub fn backprop(&self, training_data: TrainingDataEntry)
}

// loop:
// convert chessgame to tensor
// forward pass
// store state + calc outcome, delta value, moves left in training data
// batch tensor with training data
// backprop
