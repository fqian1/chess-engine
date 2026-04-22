use core::fmt;

use burn::{
    data::dataloader::batcher::Batcher,
    tensor::{Tensor, TensorData, backend::Backend},
};
use log::info;
use rand::{RngExt, rngs::SmallRng, seq::IndexedRandom};

use crate::{ChessPosition, ChessSquare, Color};

#[derive(Clone, Copy, Debug)]
pub struct NetworkInputs {
    pub boards: [f32; 64 * 14],
    pub meta:   [f32; 5],
}

impl fmt::Display for NetworkInputs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        let pieces = ['p', 'n', 'b', 'r', 'k', 'q', 'P', 'N', 'B', 'R', 'K', 'Q', 'e', 'X'];
        
        output.push_str("  a b c d e f g h\n");
        output.push_str("  ----------------\n");

        for rank in (0..8).rev() {
            output.push_str(&format!("{} ", rank + 1));
            for file in 0..8 {
                let mut char_to_print = ". ";
                let idx = rank * 8 + file;

                for k in (0..14).rev() {
                    if self.boards[k * 64 + idx] != 0.0 {
                        let c = pieces[k];
                        output.push(c);
                        output.push(' ');
                        char_to_print = "";
                        break;
                    }
                }
                output.push_str(char_to_print);
            }
            output.push('\n');
        }
        
        output.push_str("  ----------------\n");
        output.push_str(&format!("Meta: {:?}\n", self.meta));
        write!(f, "{}", output)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct NetworkLabels {
    pub policy: [f32; 64],
    pub value:  [f32; 3],
}

impl fmt::Display for NetworkLabels {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        output.push_str("______________\n");
        for i in (0..8).rev() {
            for j in 0..8 {
                let val = self.policy[i * 8 + j];
                let formatted = format!("{:.1}", val);

                output.push_str(formatted.strip_prefix("0.").unwrap_or("9"));
                output.push(' ');
            }
            output.push('\n');
        }
        output.push_str(&format!("{:?}", self.value));
        write!(f, "{}", output)
    }
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

    pub fn from_position(position: &ChessPosition, selected_sq: Option<&ChessSquare>) -> Self {
        let (chess_board, castling_rights, ep_sq) = if position.side_to_move == Color::White {
            (position.chessboard.clone(), position.castling_rights, position.en_passant)
        } else {
            (position.chessboard.flip_board(), position.castling_rights.flip_perspective(), position.en_passant.map(|x| x.square_opposite()))
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
            data[832
                + if position.side_to_move == Color::Black {
                    square.square_opposite().0 as usize
                } else {
                    square.0 as usize
                }] = 1.0;
        }

        let mut meta = [0f32; 5];
        for i in 0..4 {
            meta[i] = (castling_rights.0 >> i & 1).into();
        }
        meta[4] = position.halfmove_clock as f32 / 100.0;

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

impl fmt::Display for TrainingSample {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        output.push_str("Inputs:\n");
        output.push_str(&self.inputs.to_string());
        output.push_str(&self.targets.to_string());
        write!(f, "{}", output)
    }
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
    pub loss_ratio: f32,
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

        let board_data = TensorData::new(boards, [n, 64, 14]);
        let metas_data = TensorData::new(metas, [n, 5]);
        let pol_target = TensorData::new(targets, [n, 64]);
        let val_target = TensorData::new(values, [n, 3]);

        let boards = Tensor::from_data(board_data, device);
        let metas = Tensor::from_data(metas_data, device);
        let policy_targets = Tensor::from_data(pol_target, device);
        let value_targets = Tensor::from_data(val_target, device);

        let loss_ratio = 0.7;

        ChessBatch { boards, metas, policy_targets, value_targets, loss_ratio }
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

    pub fn push(&mut self, sample: TrainingSample) {
        if self.buffer.len() < self.capacity {
            self.buffer.push(sample);
        } else {
            self.buffer[self.pointer] = sample;
            self.pointer = (self.pointer + 1) % self.capacity;
        }
    }

    pub fn sample_batch<B: Backend>(&self, batch_size: usize, rng: &mut SmallRng, device: &B::Device) -> ChessBatch<B> {
        if self.buffer.len() <= batch_size {
            // TODO
            panic!("not enough food in buffer");
        }

        info!("sampling: {}", self.buffer[rng.random_range(0..self.buffer.len())].targets);
        let samples: Vec<TrainingSample> = self.buffer.sample(rng, batch_size).cloned().collect();
        let batcher = ChessBatcher {};
        batcher.batch(samples, device)
    }
}
