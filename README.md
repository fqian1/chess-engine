# Chess Engine

A chess client and engine built in rust. Client supports legal and pseudo-legal rule sets.

## Features

*   **FEN String Parsing**: Boards can be loaded from Forsyth-Edwards Notation (FEN) strings.
*   **Move Validation**: All standard chess moves are validated, including special moves.
*   **Special Moves**: Handles castling, en-passant, and pawn promotion.
*   **Command-Line Interface**: A simple CLI to play a game of chess.

## How to Run

1.  Clone the repository:
    ```bash
    git clone https://github.com/fqian1/chess-engine.git
    ```
2.  Navigate to the project directory:
    ```bash
    cd chess-engine
    ```
3.  Run the application:
    ```bash
    cargo run
    ```
    You can then enter moves in UCI format (e.g., `e2e4`).

### Train neural network

#### Data Collection & Preparation
- [ ] Create a data structure to represent game states (board state, played move, evaluation) for training.
- [ ] Implement a system to serialize and save game data.
- [ ] Generate a dataset by running self-play games (reinforcement learning).

#### Training
- [ ] Set up a training pipeline using `burn`.
- [ ] Implement a training loop to feed game data to the model.
- [ ] Choose and implement a loss function (e.g., cross-entropy for move prediction, mean squared error for evaluation).
- [ ] Select and configure an optimizer (e.g., Adam).

```

# model architecture
inputs:
 - 64x14 tensor: 8x8 grid, 12 1 hot planes for pieces, 1 hot plane for en passant, 1/multi-hot plane for move (0 hot when picking from_sq, 1 hot when picking to_sq, 2 hot when picking promotion piece, just populate the squares)
 - 1x5 tensor: 4x1 hot castling rights, 1 scalar for 50 move rule. no 3 fold repetition (handle with contempt + search tree)
encoder:
 - project into embedding dimension. add 2d positional encoding
outputs:
 - policy: 8x8 grid, used for from_sq, to_sq and promotion_piece (promotion file look down rank for piece). bce loss, softmax activation (legal: add mask)
 - value: scalar. tanh activation, mse loss.

hyper parameter tuning:
legal punishment ratio:
geometrically impossible move : pseudolegal move : losses
pseudo legal:
geometrically impossible : loss
depth vs size

play pseudo legal, legal with masking, no masking compare 4 permutations whats difference

rayon this shit

# pseudo code?
batch = [ChessGame; batch_size]
batch.iter_mut().for_each(|game| {
        game.zobrist_hash = game.calculate_hash()
})

loop {
        batch.iter_mut().map(|game| game.result = game.check_outcome);
        let remaining_batch = batch.iter().filter(|game| game.result.is_none);
        let tensors = remaining_batch.iter_mut().for_each(|game| {
                let board, meta = if game.side_to_move == black {
                        game.board.flip, game.meta.to_tensor
                } else { game.board, game.meta}

                let board_tensor = board.to_tensor
                let meta_tensor = meta.to_tensor + Tensor::from(half_move_clock/50)

                (board_tensor, meta_tensor)
        })
        if masking, get legal from squares mask output. gen legal from sqs, -> tensor where 0 -> f32::MIN, apply onto policy head, then softmax.
        if not masking, just skip that
        let outputs = transformer.forward(board_tensor, meta_tensor) // replace with search tree - take value from depth, but obviously next policy/moves_left
        // unbatch and turn into Vec<(policy: [f32;64], value: f32); batch_size>


        let policy_sq = match rule_set {
                legal => {
                        let mut indices: [usize; 64] = [0; 64];
                        for i in 0..64 {
                                squares[i] = i
                        }

                        indices.sort_unstable_by(|&a, &b| {value[b].total_cmp(&value[a])})

                        squares: Vec<ChessSquare> = indices.iter().map(|x| ChessSquare::from(x).unwrap())
                        squares.filter(self.legal_from_sq)
                        top_sq
                }
                pseudo-legal => {
                        let max_idx = values.iter().enumerate().fold(0, |acc, (i, x)| {
                                if x > &values[acc] { i } else { acc }
                        });
                        ChessSquare::from(max_idx)
                }
        }

        batch.iter_mut().for_each(|game| {
                if let Some(entry) = game.GameStateEntry.last_mut() {
                        entry.value = value
                }
                game.GameStateEntry.push(GameStateEntry::new(..policy_sq))
        })

        make tensors again, but include from_sq from last policy

        do forward pass again
        populate entries

        if needs promotion square, make tensors again {
                forward pass again
                extract piece:
                map policy_sq {
                        to_sq = q
                        to_sq - 8 = r // square below promotion sq
                        to_sq - 16 = b // 2 squares below
                        to_sq - 24 = n // 3 below
                        _ => lose if pseudo legal or softmax it out
                }
                // promotion happen on same file so do like this
                populate entries
        }

        remaining_batch.map(|game| game.make_move) // this should fill in rest of game.GameStateEntry
}
regret...
