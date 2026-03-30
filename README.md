# Chess Engine

A chess client and engine in about ~2000 lines. Client supports legal and pseudo-legal rule sets.

## Features

*   **FEN String Parsing**: Boards can be loaded from Forsyth-Edwards Notation (FEN) strings.
*   **Move Generation**: All standard chess moves can be generated for a given position and rule set.
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

This is my undergrad cs fyp: custom chess client and chess engine

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
   - Pass 2: Select destination square (feed Pass 1 output back into input). (Promotions handled client side in the MCTS).
- MCTS Implications: Doubles tree depth, but reduces branching factor and model complexity.
- Both heads use softmax activation, kl divergence loss.

3. Endgame Table Base Injection Scheduling
-------------------------------------------------------------------------------
Test three distinct EGTB perfect knowledge injection strategies.
   - Schedule 1 (Bootstrap): Inject at the beginning - perfect knowledge bootstraps value head, comes with risk of forgetting/overfitting.
   - Schedule 2 (Balanced): Inject evenly throughout the entire training pipeline.
   - Schedule 3 (Late-Stage): Inject near the end of training - aligns with chronological order of learning, comes with risk of catastrophic forgetting.

4. Bespoke Client and Engine Implementation
-------------------------------------------------------------------------------
- Board Representation: Bitboards.
- Move Generation: Pseudo-legal and legal generators.
- Language/Stack: rust, burn, rand, rayon.

5. Deliverables and Metrics
-------------------------------------------------------------------------------
- Training statistics (Loss curves, ELO progression, illegal move frequency).
- Model snapshots at defined epochs for all experimental branches.
- Comparative analysis: Masked vs. Unmasked, Legal vs. Pseudo-Legal, EGTB schedules.
