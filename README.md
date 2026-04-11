# Chess Engine

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

This is my undergrad fyp:
A chess client and mcts powered engine in about ~2500 lines.

## Features

*   **FEN String Parsing**: load chess game from fen strings
*   **Move Generation**: generate pseudolegal moves for a given position then filter for legality
*   **Command-Line Interface**: A simple CLI to train your own model and then run inference on it

## How to build from source:

1. Ensure the rust toolchain is installed:
   Linux/MacOS: ```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```
   Windows: Find the installer here: https://rustup.rs/#
   ```

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
    cargo run -- -batch_size 1 -legal -masked -n 20 -e 20 -i 3 -path "./tmp"
    ```
Run with the -h flag to see options

Distributed under the MIT License. See LICENSE for more information.

1. Hypothesis and Experiments
-------------------------------------------------------------------------------
A. Ruleset Convergence (Pseudo-Legal vs. Legal)
   - Control: Train on strict legal ruleset.
   - Test: Train on pseudo-legal ruleset (explicit king capture = win).
   - Hypothesis: Pseudo-legal training naturally converges to legal play (checkmate/pins emerge as emergent behaviors to secure/prevent king capture).

B. Logit Masking vs. Punishment (Mechanics "Grokking")
   - Control: Mask illegal moves before softmax (standard practice; network is blind to rules).
   - Test: No masking before softmax. Punish illegal/impossible moves via loss function.
   - Hypothesis: Unmasked training learns slower and plays tentatively, but forces the network to internally map ("grok") the physical mechanics of the board.

2. Novel Network Architecture: 2 Pass Encoder
-------------------------------------------------------------------------------
- Inputs: Board state (12 bitboards) + En Passant square (1 bitboard) + Selected square (1 bitboard) & Castling rights (4x1 f32) + 50-move rule scalar (1 f32).
- Outputs: Policy head (64-square distribution: [f32; 64]) + Value head (W/D/L buckets: [f32; 3]).
- Execution Flow:
   - Pass 1: Select origin square (piece to move).
   - Pass 2: Select destination square (feed Pass 1 output back into input). (Promotions rolled out automatically by Mcts).
- MCTS Implications: Doubles tree depth, but reduces model action space.
- Both heads use softmax activation, kl divergence loss.

3. Bespoke Client and Engine Implementation
-------------------------------------------------------------------------------
- Board Representation: Bitboards.
- Move Generation: Pseudo-legal and legal generators.
- Bipartite MCTS for piece -> destination.
- Language/Stack: rust, burn, rand, rayon.
