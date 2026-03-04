use burn::tensor::{Tensor, backend::Backend};

#[derive(Clone, Debug)]
pub struct TrainingSample {
    pub board: [f32; 64 * 14],
    pub meta: [f32; 5],
    pub target_policy: [f32; 64],
    pub target_value: f32,
}

pub struct Batch<B: Backend> {
    pub boards:  Tensor<B, 3>,
    pub meta:    Tensor<B, 2>,
    pub targets: Tensor<B, 2>,
    pub values:  Tensor<B, 1>,
}

pub struct ReplayBuffer {
    capacity: usize,
    buffer:   Vec<TrainingSample>,
}

impl ReplayBuffer {
    pub fn new(capacity: usize) -> Self {
        Self { capacity, buffer: Vec::with_capacity(capacity) }
    }

    pub fn push(&mut self, sample: TrainingSample) {
        if self.buffer.len() >= self.capacity {
            self.buffer.remove(0);
        }
        self.buffer.push(sample);
    }

    pub fn sample_batch<B: Backend>(&self, batch_size: usize, device: &B::Device) -> Batch<B> {
        // top N
        let start = self.buffer.len().saturating_sub(batch_size);
        let samples = &self.buffer[start..];

        let mut flat_boards = Vec::with_capacity(samples.len() * 64 * 14);
        let mut flat_meta = Vec::with_capacity(samples.len() * 5);
        let mut targets = Vec::with_capacity(samples.len() * 64);
        let mut values = Vec::with_capacity(samples.len());

        for s in samples {
            flat_boards.extend_from_slice(&s.board);
            flat_meta.extend_from_slice(&s.meta);
            targets.extend_from_slice(&s.target_policy);
            values.push(s.target_value);
        }

        Batch {
            boards:  Tensor::<B, 3>::from_floats(flat_boards.as_slice(), device).reshape([samples.len(), 64, 14]),
            meta:    Tensor::<B, 2>::from_floats(flat_meta.as_slice(), device).reshape([samples.len(), 5]),
            targets: Tensor::<B, 2>::from_floats(targets.as_slice(), device).reshape([samples.len(), 64]),
            values:  Tensor::from_floats(values.as_slice(), device),
        }
    }
}
