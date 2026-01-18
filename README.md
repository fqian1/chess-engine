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
- [ ] Design a neural network architecture for board evaluation.
- [ ] Implement the network with Burn; Input: Bitboards of game state + Some(Square coords) and Output: Square coords.

#### Training
- [ ] Set up a training pipeline using `burn`.
- [ ] Implement a training loop to feed game data to the model.
- [ ] Choose and implement a loss function (e.g., cross-entropy for move prediction, mean squared error for evaluation).
- [ ] Select and configure an optimizer (e.g., Adam).
- [ ] Parallelize the training process.

