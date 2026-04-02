use burn::{
    Tensor,
    config::Config,
    optim::{Adam, AdamConfig, GradientsParams, Optimizer, adaptor::OptimizerAdaptor},
    prelude::Backend,
    tensor::{Bool, TensorData, activation::softmax, backend::AutodiffBackend},
};
use rand::{SeedableRng, rngs::SmallRng};

use crate::{
    ChessGame, ChessTransformer, Mcts, MctsConfig, ReplayBuffer,
    chess_game::Outcome,
    data::{ChessBatch, NetworkInputs, NetworkLabels},
    expand_batch,
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
    let (boards, metas) = inputs_to_tensor(inputs, device);
    let (mut policies, values) = model.forward(boards, metas);
    if let Some(masks) = masks {
        let mask_data = TensorData::new(masks, [batch_size, 64]);
        let mask = Tensor::<B, 2, Bool>::from_data(mask_data, device);
        policies = policies.clone().mask_fill(mask, -1e9);
    }
    policies = softmax(policies, 1);
    // batch_size x 64
    let policies: Tensor<B, 1> = policies.flatten(0, 1);
    let policies = policies.into_data().to_vec::<f32>().unwrap();

    // batch_size x 3
    let values: Tensor<B, 1> = values.flatten(0, 1);
    let values = values.into_data().to_vec::<f32>().unwrap();

    let mut out: Vec<NetworkLabels> = Vec::with_capacity(batch_size);

    for i in 0..batch_size {
        let policy: [f32; 64] = policies[(i * 64)..(i * 64 + 64)].try_into().unwrap();
        let value: [f32; 3] = values[(i * 3)..(i * 3 + 3)].try_into().unwrap();
        out.push(NetworkLabels { policy, value });
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

pub fn play<B: AutodiffBackend>(
    artifact_dir: &str,
    mcts_config: &MctsConfig,
    training_config: &TrainingConfig,
    replay_buffer: &mut ReplayBuffer,
    device: &B::Device,
) {
    create_artifact_dir(artifact_dir);

    B::seed(device, training_config.seed);

    let mut model: ChessTransformer<B> = training_config.model.init(device);
    let mut optimizer = training_config.optimizer.init::<B, ChessTransformer<B>>();

    let mut games = vec![ChessGame::default(); training_config.batch_size];

    let mut rng = SmallRng::seed_from_u64(training_config.seed);

    loop {
        games.iter_mut().for_each(|game| match game.check_game_state(training_config.legal) {
            Outcome::Finished(_) => *game = ChessGame::default(),
            Outcome::Unfinished => (),
        });

        let mut mctss: Vec<Mcts> = games.iter().map(|game| Mcts::from_game(&game, 1000, *mcts_config)).collect();

        // just play 50 moves or something
        for _ in 0..50 {
            // mcts roll out
            for _ in 0..mcts_config.num_simulations {
                mctss.iter_mut().for_each(|e| {
                    e.traverse();
                });
                expand_batch(&mut mctss[..], model.clone(), training_config, device);
            }

            // get best move and play it
            mctss.iter_mut().zip(games.iter_mut()).for_each(|(mcts, game)| {
                let sample = mcts.make_targets();
                replay_buffer.push(sample);
                if let Some(mov) = mcts.get_move() {
                    game.make_move(&mov);
                }
            });
        }

        train(&model, &mut optimizer, &training_config, &replay_buffer, device, &mut rng);

        // make snapshot of model
    }
}

pub fn train<B: AutodiffBackend>(
    model: &ChessTransformer<B>,
    optimizer: &mut OptimizerAdaptor<Adam, ChessTransformer<B>, B>, // what... tf?
    config: &TrainingConfig,
    games: &ReplayBuffer,
    device: &B::Device,
    rng: &mut SmallRng,
) {
    let datas: ChessBatch<B> = games.sample_batch(config.batch_size, rng, device);
    let output = model.forward_classification(datas);
    let loss = output.loss;
    let grads = loss.backward();
    let grads = GradientsParams::from_grads(grads, model);
    optimizer.step(config.learning_rate, model.clone(), grads);
}
