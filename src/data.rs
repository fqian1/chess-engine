use burn::{
    backend, data::dataloader::batcher::Batcher, tensor::{Tensor, backend::Backend}
};
use rand::{rngs::SmallRng, seq::IndexedRandom};

use crate::{ChessPosition, ChessSquare, Color};

#[derive(Clone, Copy, Debug)]
pub struct NetworkInputs {
    pub boards: [f32; 64 * 14],
    pub meta:   [f32; 5],
}

#[derive(Clone, Copy, Debug)]
pub struct NetworkLabels
{
    pub policy: [f32; 64],
    pub value:  [f32; 3],
}

impl Default for NetworkInputs {
    fn default() -> Self {
        Self { boards: [0.0; 64 * 14], meta: [0.0; 5] }
    }
}

impl Default for NetworkLabels {
    fn default() -> Self {
        Self { policy: [0.0; 64], value: [0.0; 3] }
    }
}

impl NetworkInputs {
    pub fn new(position: &ChessPosition) -> Self {
        NetworkInputs::from_position(position, None)
    }

    pub fn from_position(position: &ChessPosition, selected_sq: Option<ChessSquare>) -> Self {
        let (chess_board, castling_rights, ep_sq) = if position.side_to_move == Color::White {
            (position.chessboard.clone(), position.castling_rights, position.en_passant)
        } else {
            (
                position.chessboard.flip_board(),
                position.castling_rights.flip_perspective(),
                position.en_passant.map(|x| x.square_opposite()),
            )
        };

        let mut data = [0f32; 64 * 14];
        chess_board.pieces[0][0].write_to_slice(&mut data[0..64]);
        chess_board.pieces[0][1].write_to_slice(&mut data[64..128]);
        chess_board.pieces[0][2].write_to_slice(&mut data[128..192]);
        chess_board.pieces[0][3].write_to_slice(&mut data[192..256]);
        chess_board.pieces[0][4].write_to_slice(&mut data[256..320]);
        chess_board.pieces[0][5].write_to_slice(&mut data[320..384]);

        chess_board.pieces[1][0].write_to_slice(&mut data[384..448]);
        chess_board.pieces[1][1].write_to_slice(&mut data[448..512]);
        chess_board.pieces[1][2].write_to_slice(&mut data[512..576]);
        chess_board.pieces[1][3].write_to_slice(&mut data[576..640]);
        chess_board.pieces[1][4].write_to_slice(&mut data[640..704]);
        chess_board.pieces[1][5].write_to_slice(&mut data[704..768]);

        if let Some(square) = ep_sq {
            data[768 + square.0 as usize] = 1.0;
        }

        if let Some(square) = selected_sq {
            data[832 + square.0 as usize] = 1.0;
        }

        let mut meta = [0f32; 5];
        for i in 0..4 {
            meta[i] = (castling_rights.0 >> i & 1).into();
        }
        meta[4] = position.halfmove_clock as f32 / 50.0;

        Self { boards: data, meta }
    }
}

impl NetworkLabels {
    pub fn as_squares(&self) -> [(ChessSquare, f32); 64] {
        std::array::from_fn(|i| {
            let sq = ChessSquare::new(i as u8).unwrap();
            let val = self.policy[i];
            (sq, val)
        })
    }
}

#[derive(Default, Clone)]
pub struct ChessBatcher {}

#[derive(Clone, Copy, Debug)]
pub struct TrainingSample {
    pub inputs:  NetworkInputs,
    pub targets: NetworkLabels,
}

impl Default for TrainingSample {
    fn default() -> Self {
        TrainingSample { inputs: NetworkInputs::default(), targets: NetworkLabels::default() }
    }
}

#[derive(Clone, Debug)]
pub struct ChessBatch<B: Backend> {
    pub boards: Tensor<B, 3>,         // Batch x 64 x 14
    pub metas: Tensor<B, 2>,          // Batch x 5
    pub policy_targets: Tensor<B, 2>, // Batch x 64
    pub value_targets: Tensor<B, 2>,  // Batch x 1
}

impl<B: Backend> Batcher<B, TrainingSample, ChessBatch<B>> for ChessBatcher {
    fn batch(&self, items: Vec<TrainingSample>, device: &B::Device) -> ChessBatch<B> {
        let n = items.len();

        let mut boards = Vec::with_capacity(n * 64 * 14);
        let mut metas = Vec::with_capacity(n * 5);
        let mut targets = Vec::with_capacity(n * 64);
        let mut values = Vec::with_capacity(n * 3);

        for item in items {
            boards.extend_from_slice(&item.inputs.boards);
            metas.extend_from_slice(&item.inputs.meta);
            targets.extend_from_slice(&item.targets.policy);
            values.extend_from_slice(&item.targets.value)
        }

        ChessBatch {
            boards: Tensor::<B, 3>::from_floats(boards.as_slice(), device).reshape([n, 64, 14]),
            metas: Tensor::<B, 2>::from_floats(metas.as_slice(), device).reshape([n, 5]),
            policy_targets: Tensor::<B, 2>::from_floats(targets.as_slice(), device).reshape([n, 64]),
            value_targets: Tensor::<B, 2>::from_floats(values.as_slice(), device).reshape([n, 3]),
        }
    }
}

pub struct ReplayBuffer {
    pub capacity: usize,
    pub pointer:  usize,
    pub buffer:   Vec<TrainingSample>,
}

impl ReplayBuffer {
    pub fn new(capacity: usize) -> Self {
        Self { capacity, pointer: 0, buffer: Vec::with_capacity(capacity) }
    }

    pub fn push(&mut self, sample: &TrainingSample) {
        if self.buffer.len() < self.capacity {
            self.buffer.push(sample.clone());
        } else {
            self.buffer[self.pointer] = sample.clone();
            self.pointer = (self.pointer + 1) % self.capacity;
        }
    }

    pub fn sample_batch<B: Backend>(&self, batch_size: usize, rng: &mut SmallRng, device: &B::Device) -> ChessBatch<B> {
        let samples: Vec<TrainingSample> = self.buffer.sample(rng, batch_size).cloned().collect();
        let batcher = ChessBatcher {};
        batcher.batch(samples, device)
    }
}
