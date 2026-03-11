use burn::{
    data::dataloader::batcher::Batcher,
    tensor::{Int, Tensor, backend::Backend},
};
use rand::{rngs::SmallRng, seq::IndexedRandom};

#[derive(Default, Clone)]
pub struct ChessBatcher {}

#[derive(Clone, Debug)]
pub struct TrainingSample {
    pub board: [f32; 64 * 14],    // 64 * 14 boards
    pub meta: [f32; 5],           // 4 castling rights 1 hot + f32 move counter
    pub target_policy: [f32; 64], // distribution over squares
    pub target_value: i32,        // distribution over w/d/l (but target is one hot so i32)
}

impl Default for TrainingSample {
    fn default() -> Self {
        TrainingSample { board: [0.0; 64 * 14], meta: [0.0; 5], target_policy: [0.0; 64], target_value: 0 }
    }
}

#[derive(Clone, Debug)]
pub struct ChessBatch<B: Backend> {
    pub boards: Tensor<B, 3>,             // Batch x 64 x 14
    pub metas: Tensor<B, 2>,              // Batch x 5
    pub policy_targets: Tensor<B, 2>,     // Batch x 64
    pub value_targets: Tensor<B, 1, Int>, // Batch x 1
}

impl<B: Backend> Batcher<B, TrainingSample, ChessBatch<B>> for ChessBatcher {
    fn batch(&self, items: Vec<TrainingSample>, device: &B::Device) -> ChessBatch<B> {
        let n = items.len();

        let mut boards = Vec::with_capacity(n * 64 * 14);
        let mut metas = Vec::with_capacity(n * 5);
        let mut targets = Vec::with_capacity(n * 64);
        let mut values = Vec::with_capacity(n);

        for item in items {
            boards.extend_from_slice(&item.board);
            metas.extend_from_slice(&item.meta);
            targets.extend_from_slice(&item.target_policy);
            values.push(item.target_value)
        }

        ChessBatch {
            boards: Tensor::<B, 3>::from_floats(boards.as_slice(), device).reshape([n, 64, 14]),
            metas: Tensor::<B, 2>::from_floats(metas.as_slice(), device).reshape([n, 5]),
            policy_targets: Tensor::<B, 2>::from_floats(targets.as_slice(), device).reshape([n, 64]),
            value_targets: Tensor::<B, 1, Int>::from_ints(values.as_slice(), device).reshape([n]),
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

    // for inferencing!!
    pub fn to_tensor<B: Backend>(&self, device: &B::Device) -> (Tensor<B, 3>, Tensor<B, 2>) {
        let n = self.buffer.len();

        let mut boards = Vec::with_capacity(n * 64 * 14);
        let mut metas = Vec::with_capacity(n * 5);

        for item in &self.buffer {
            boards.extend_from_slice(&item.board);
            metas.extend_from_slice(&item.meta);
        }

        let t1 = Tensor::<B, 3>::from_floats(boards.as_slice(), device).reshape([n, 64, 14]);
        let t2 = Tensor::<B, 2>::from_floats(metas.as_slice(), device).reshape([n, 5]);
        (t1, t2)
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
        let samples: Vec<TrainingSample> = self.buffer.sample(rng, batch_size).cloned().collect();
        let batcher = ChessBatcher {};
        batcher.batch(samples, device)
    }
}
