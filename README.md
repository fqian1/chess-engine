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

## Roadmap and TODO List

### Core Engine Improvements
- [ ] Implement check detection.
- [ ] Implement checkmate and stalemate detection.
- [ ] Implement full legal move generation functions.
- [ ] Add `perft` testing to ensure move generation is accurate.
- [ ] Implement Zobrist hashing for efficient position tracking and repetition detection.
- [ ] Parallelize play to run multiple games simultaneously.

### Train neural network

#### Data Collection & Preparation
- [ ] Create a data structure to represent game states (board state, played move, evaluation) for training.
- [ ] Implement a system to serialize and save game data.
- [ ] Generate a dataset by running self-play games (reinforcement learning).

#### Model Architecture (using Burn)
- [ ] Design a Transformer architecture for board evaluation. Self attention for each square. input 12 or 15 bitboards? half move counter, normalised between 0-1 (half move counter / 100). give it last 8 chessboard states.
- [ ] Implement the network with Burn; Input: Bitboards of game state + Some(Square coords) and Output: Square coords.
- [ ] 

#### Training
- [ ] Set up a training pipeline using `burn`.
- [ ] Implement a training loop to feed game data to the model.
- [ ] Choose and implement a loss function (e.g., cross-entropy for move prediction, mean squared error for evaluation).
- [ ] Select and configure an optimizer (e.g., Adam).
- [ ] Parallelize the training process.

```
# model architecture
inputs:
 - 8x8x14 tensor: 8x8 grid, 12 1 hot planes for pieces, 1 hot plane for en passant, 0/1/2 hot plane for move (0 hot when picking from_sq, 1 hot when picking to_sq, 2 hot when picking promotion piece, just populate the squares)
 - 1x5 tensor: 4x1 hot castling rights, 1 scalar for 50 move rule. no 3 fold repetition (handle with contempt + search tree)
outputs:
 - policy: 8x8 grid, used for from_sq, to_sq and promotion_piece. binary cross entropy loss function, softmax activation (add mask before for pseudo legal)
 - value: scalar. tanh activation, mse loss.
 - moves_left: 10x1 scalar, where each scalar represents bucket 1-10, 11-20, 21-30 moves left etc. allows to represent sharp positions (e.g. both small and big buckets)

# pseudo code?
MoveData{policy(Bitboard), value(f32), moves_left(u32)} // bitboard is u64
GameData{Chessboard([Bitboard; 12]), en_passant(square), }

GameTimeLine {Vec<MoveData>, Vec<GameData>, result: Option<f32>}

Batch = [GameTimeLine; batch_size]
Games = [ChessGame; batch_size]

inference loop: while some game still playing {
        if GameTimeLine.result.is_none { // do this to dont check unecessarily
                result = chessgame.get_outcome // -1, 0 or -1
        }

        if game.side_to_move is different {
                game.chessboard.invert_rows, invert castling
        }

        features = [(board_tensor, meta_tensor); batchsize] from Game // get batched features, dont forget to flip it if side to move is black

        batch_tensor = forward(features)

        targets = [(policy1, value, moves_left); batch_size] from batch_tensor // extract from_sq, value and moves_left convert from tensor to chessgame representation

        push targets to BatchFrom[0] // push targets to training data structs.

        features.board_tensor += targets.policy1 // add from_sq to board_tensor

        batch_tensor = forward(features) // second pass to get the to_square

        targets = [(policy2, value, move_left); batch_size] from batch_tensor // extract to_sq, value and moves left from tensor to chessgame representaiton

        push target to Batch2[0] if white else Batch2[1]

        if to_sq.rank == 8 {
                populate from_sq plane with to_sq
                run inference get policy_sq,
                extract piece:
                map policy_sq {
                        to_sq = q
                        to_sq - 8 = r // square below promotion sq
                        to_sq - 16 = b // 2 squares below
                        to_sq - 24 = n // 3 below
                        _ => lose if pseudo legal or softmax it out
                }
                // promotion happen on same file so do like this
        }

        game.make_move(move{from_sq, to_sq, Some()})
}

for GameTimeLine[0],GameTimeLine[1] in BatchTo {
        for each MoveData in GameTimeLine {
                Decrement moves_left (start at GameTimeLine.count)

                calculate average between win/loss and value in subsequent entry // TD loss

                set policy_sq to 1, -0.1, -1 depending on result
        }
}
repeat from BatchFrom

training loop {
        for each entry in from_training_data {
                flatten the mega batch
                mse bce mse tanh whatever
                backprop
        }
}
```

