# Chess Engine

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

A Rust-based chess engine that combines Monte Carlo Tree Search (MCTS) with a Transformer neural network for self-play reinforcement learning. Built with the Burn deep learning framework.

## Overview
This project implements an AlphaZero-style chess engine where a Transformer model learns to evaluate positions and suggest moves through self-play. The engine consists of:
 - A fully featured chess library with bitboards, move generators, legality checking, fen parsing.
 - A Bipartite MCTS implementation with PUCT node selection and dirichlet noise. 
 - An autoregressive Transformer neural network that predicts a piece to select or square to move to and position value.
 - A self-play pipeline to train the transformer

## Features
*   **FEN String Parsing**: load chess game from fen strings.
*   **Move Generation**: generate pseudolegal moves for a given position then filter for legality.
*   **MCTS**: Configurable MCTS rollouts.
*   **Transformer Model**: 8 heads, 8 layers, 512 embedding dimensions.
*   **Masking and Legality**: Optional masking and legality training options.
*   **Cuda and Wgpu**: Configurable backends with --features flag.
*   **Command-Line Interface**: A simple CLI to train your own model and then run inference on it.

## How to build from source:

1. Ensure the rust toolchain is installed:
   Linux/MacOS: 
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
   Windows: Find the installer here: https://rustup.rs/#

2.  Clone the repository:
    ```bash
    git clone https://github.com/fqian1/chess-engine.git
    ```

3.  Navigate to the project directory:
    ```bash
    cd chess-engine
    ```

4.  Optional: allow direnv (Nix only):
    ```bash
    direnv allow
    ```
This makes it 2% more likely the code will compile.

5.  Run the application:
    ```bash
    cargo run -- -batch_size 1 -legal -masked -n 20 -g 20 -i 3 -path "./tmp"
    ```
Run with the -h flag to see options


```
default features disable autotune and use wgpu
for big speed:
--no-default-feature --features autotune --features cuda
```

Tested on: rustc rustc 1.97.0-nightly (ca9a134e0 2026-04-26)

Project structure: 
```
./
├── bin/
│   ├── stockfish*
│   └── stockfish_source.txt
├── src/
│   ├── bin/
│   │   └── parse_data.rs
│   ├── bitboard.rs
│   ├── castling.rs
│   ├── chess_board.rs
│   ├── chess_game.rs
│   ├── chess_move.rs
│   ├── chess_piece.rs
│   ├── chess_position.rs
│   ├── chess_square.rs
│   ├── data.rs
│   ├── engine.rs
│   ├── lib.rs
│   ├── main.rs
│   ├── mcts.rs
│   ├── model.rs
│   ├── stockfish.rs
│   └── zobrist.rs
├── tests/
│   └── tests.rs
├── Cargo.lock
├── Cargo.toml
├── flake.lock
├── flake.nix
├── LICENSE
├── mate_evals.tsv
├── README.md
├── run.sh*
├── rust-analyzer.toml
└── rustfmt.toml
```

Performance Notes: 
The Engine was designed for interpretability and experimentation, training can take weeks, and isn't suited for competitive play against top engines. System Memory usage scales with VRAM usage.

# Open Source Notices:
This project incorporates the Stockfish chess engine (located in /bin), which is licensed under the GNU General Public License v3.0 (GPL v3).
- Stockfish Source Code: You can find the original source code at stockfishchess.org or github.com/official-stockfish/Stockfish.
- Modifications: The Stockfish binary included here has been compressed using UPX for size optimization, but the underlying source code remains unchanged.

This project utilizes evaluation data from the [Lichess Open Database(https://database.lichess.org/#evals)]. We thank Lichess.org for providing this resource to the public.

Distributed under the GPLv3 License. See LICENSE for more information.
