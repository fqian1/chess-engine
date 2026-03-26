use burn::{
    Tensor,
    config::Config,
    optim::{Adam, AdamConfig, GradientsParams, Optimizer, adaptor::OptimizerAdaptor},
    prelude::Backend,
    tensor::{Bool, TensorData, activation::softmax, backend::AutodiffBackend},
};
use rand::rngs::SmallRng;

use crate::{
    ChessGame, ChessTransformer, MctsConfig, ReplayBuffer,
    data::{ChessBatch, NetworkInputs, NetworkLabels, TrainingSample},
    model::ChessTransformerConfig,
};

fn create_artifact_dir(artifact_dir: &str) {
    std::fs::remove_dir_all(artifact_dir).ok();
    std::fs::create_dir_all(artifact_dir).ok();
}

#[derive(Config, Debug)]
pub struct TrainingConfig {
    pub model: ChessTransformerConfig,
    pub masked: bool,
    pub legal: bool,
    pub optimizer: AdamConfig,
    #[config(default = 10)]
    pub num_epochs: usize,
    #[config(default = 1024)]
    pub batch_size: usize,
    #[config(default = 4)]
    pub num_workers: usize,
    #[config(default = 1234)]
    pub seed: u64,
    #[config(default = 1.0e-4)]
    pub learning_rate: f64,
}

pub fn model_make_outputs<B: Backend>(
    model: ChessTransformer<B>,
    inputs: &Vec<NetworkInputs>,
    config: &TrainingConfig,
    masks: Option<Vec<bool>>,
    device: &B::Device,
) -> Vec<NetworkLabels> {
    let batch_size = config.batch_size;
    let (board, meta) = inputs_to_tensor(inputs, device);
    let (mut policy, value) = model.forward(board, meta);
    if let Some(bools) = masks {
        let data = TensorData::new(bools, [batch_size, 64]);
        let mask = Tensor::<B, 2, Bool>::from_data(data, device);
        policy = policy.clone().mask_fill(mask, -1e9);
    }
    policy = softmax(policy, 1);
    // batch x 64
    let policy: Tensor<B, 1> = policy.flatten(0, 1);
    let policy = policy.into_data().to_vec::<f32>().unwrap();
    let values: Tensor<B, 1> = value.flatten(0, 1);
    let values = values.into_data().to_vec::<f32>().unwrap();

    let mut out = vec![NetworkLabels::default(); batch_size];

    for i in 0..batch_size {
        let mut policy = [0.0; 64];
        let mut value = [0.0; 3];
        for j in 0..64 {
            policy[j] = policy[j + i * 64];
            value[0] = values[j];
            value[1] = values[j + 1];
            value[2] = values[j + 2];
        }
        out[i] = NetworkLabels { policy, value };
    }

    out
}

pub fn inputs_to_tensor<B: Backend>(buffer: &Vec<NetworkInputs>, device: &B::Device) -> (Tensor<B, 3>, Tensor<B, 2>) {
    let n = buffer.len();

    let mut boards = Vec::with_capacity(n * 64 * 14);
    let mut metas = Vec::with_capacity(n * 5);

    for item in buffer {
        boards.extend_from_slice(&item.boards);
        metas.extend_from_slice(&item.meta);
    }

    let t1 = Tensor::<B, 3>::from_floats(boards.as_slice(), device).reshape([n, 64, 14]);
    let t2 = Tensor::<B, 2>::from_floats(metas.as_slice(), device).reshape([n, 5]);
    (t1, t2)
}

pub fn generate_self_play_data<B: AutodiffBackend>(
    artifact_dir: &str,
    mcts_config: &MctsConfig,
    training_config: &TrainingConfig,
    replay_buffer: &mut ReplayBuffer,
    device: &B::Device,
) -> Vec<TrainingSample> {
    B::seed(device, training_config.seed);
    let model: ChessTransformer<B> = training_config.model.init(device);
    let optimizer = training_config.optimizer.init::<B, ChessTransformer<B>>();

    let mut games = vec![ChessGame::default(); training_config.batch_size];
    let mut samples: Vec<TrainingSample> = Vec::new();
    let mut action_node_arena: Vec<MctsNodeAction> = Vec::new();
    let mut state_node_arena: Vec<MctsNodeState> = Vec::new();
    let mut network_outputs: Vec<NetworkLabels> = Vec::with_capacity(training_config.batch_size);

    loop {
        run_mcts(&games, model, mcts_config, training_config, device);
        // TODO
    }
    samples
}

//         samples.iter_mut().enumerate().for_each(|(i, sample)| {
//             sample.board = current_game_state[i].boards;
//             sample.meta = current_game_state[i].meta;
//             //sample.target_policy = policy_from_mcst
//             //sample.target_value = value_from_mcst
//         });
//         replay_buffer.buffer.append(&mut samples);
//     }
// }

pub fn train<B: AutodiffBackend>(
    model: ChessTransformer<B>,
    optimizer: &mut OptimizerAdaptor<Adam, ChessTransformer<B>, B>, // what... tf?
    config: TrainingConfig,
    games: &ReplayBuffer,
    device: &B::Device,
    rng: &mut SmallRng,
) {
    let datas: ChessBatch<B> = games.sample_batch(config.batch_size, rng, device);
    let output = model.forward_classification(datas);
    let loss = output.loss;
    let grads = loss.backward();
    let grads = GradientsParams::from_grads(grads, &model);
    optimizer.step(config.learning_rate, model, grads);
}
