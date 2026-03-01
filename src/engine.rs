use burn::{Tensor, prelude::Backend};
use crate::{Bitboard, CastlingRights, ChessBoard, ChessSquare};

#[derive(Debug, Clone)]
pub struct TrainingSample {
    pub board:  [Bitboard; 64 * 14],
    pub meta:   [f32; 5],
    pub policy: [f32; 64],
    pub value:  f32,
}

impl TrainingSample {
    pub fn from_game<B: Backend>(
        &self,
        device: &B::Device,
        chess_board: &ChessBoard,
        castling_rights: &CastlingRights,
        en_passant: Option<ChessSquare>,
        half_move_counter: u32,
    ) -> TrainingSample {
        let board_f32 = chess_board.to_f32(en_passant);
        let board = Tensor::from_data(board_f32, device).reshape([64, 14]);

        let mut meta = [0.0f32; 5];
        for i in 0..4 {
            meta[i] = (castling_rights.0 >> i & 1).into();
        }
        meta[4] = half_move_counter as f32 / 50.0;
        let meta = Tensor::from_data(meta, device);
        TrainingSample { board, meta }
    }

    pub fn to_tensor<B: Backend>(
        &self,
        device: &B::Device,
        from_sq: Option<ChessSquare>,
        to_sq: Option<ChessSquare>,
    ) -> (Tensor<B, 2>, Tensor<B, 1>) {
        let (chess_board, castling_rights, ep_sq) = if self.side_to_move == Color::White {
            (self.chessboard.clone(), self.castling_rights, self.en_passant)
        } else {
            (
                self.chessboard.flip_board(),
                self.castling_rights.flip_perspective(),
                self.en_passant.map(|x| x.square_opposite()),
            )
        };
        let t1 = chess_board.to_tensor(device, ep_sq, from_sq, to_sq);

        let mut data = [0.0f32; 4];
        for i in 0..4 {
            data[i] = (castling_rights.0 >> i & 1).into();
        }

        let t2 = Tensor::from_data(data, device);
        (t1, t2)
    }
}
