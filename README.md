# Chess Engine

A chess client and engine built in rust. Client supports legal and pseudo-legal rule sets.

## Features

*   **FEN String Parsing**: Boards can be loaded from Forsyth-Edwards Notation (FEN) strings.
*   **Move Generation**: All standard chess moves can be generated for a given position and rule set.
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

### Train neural network

PROJECT: UNDERGRADUATE CS FYP - CUSTOM CHESS CLIENT & NEURAL ENGINE

1. CORE HYPOTHESES & EXPERIMENTS
-------------------------------------------------------------------------------
A. Ruleset Convergence (Pseudo-Legal vs. Legal)
   - Control: Train on strict legal ruleset.
   - Test: Train on pseudo-legal ruleset (explicit king capture = win).
   - Hypothesis: Pseudo-legal training naturally converges to legal play (checkmate/pins emerge as emergent behaviors to secure/prevent king capture).

B. Logit Masking vs. Punishment (Mechanics "Grokking")
   - Control: Mask illegal moves before softmax (standard practice; network is blind to rules).
   - Test: No masking before softmax. Punish illegal/impossible moves via loss function.
   - Hypothesis: Unmasked training learns slower and plays tentatively, but forces the network to internally map ("grok") the physical mechanics of the board.

2. NOVEL ARCHITECTURE: MULTI-PASS ENCODER
-------------------------------------------------------------------------------
- Inputs: Board state (12 bitboards) + En Passant square (1 bitboard) + Selected square(s) (1 bitboard) + Castling rights (4x1 hot) + 50-move rule scalar (1 f32).
- Outputs: Policy head (64-square distribution) + Value head (W/D/L buckets).
- Execution Flow:
   - Pass 1: Select origin square (piece to move).
   - Pass 2: Select destination square (feed Pass 1 output back into input).
   - Pass 3 (Conditional): Select promotion piece (scan down promotion file). Maybe 2 pass, expect 2 hot policy in second pass when need promotion? seeing as pawns cant move backwards, scanning down ranks wont conflict with any other move, however might be confusing to learn.
- MCTS Implications: Doubles tree depth, but drastically reduces branching factor per node (fewer candidate pieces, fewer candidate destinations).
- Both heads use softmax activation, kl divergence loss.

3. ENDGAME TABLEBASE (EGTB) INJECTION SCHEDULES
-------------------------------------------------------------------------------
Test three distinct EGTB data integration strategies to measure retention/forgetting:
   - Schedule 1 (Bootstrap): Inject at the beginning to bootstrap the value head. (Risk: Catastrophic forgetting).
   - Schedule 2 (Balanced): Inject evenly throughout the entire training pipeline.
   - Schedule 3 (Late-Stage): Inject near the end of training. (Aligns with chronological endgame learning, but risks poor integration with early-game weights).

4. ENGINE & CLIENT IMPLEMENTATION (FROM SCRATCH)
-------------------------------------------------------------------------------
- Board Representation: Bitboards.
- Move Generation: Pseudo-legal and legal generators.
- Optimizations: Bitwise intrinsics, efficient memory referencing, slice systems.
- Language/Stack: rust, burn, rand, rayon.

5. REQUIRED DELIVERABLES & METRICS
-------------------------------------------------------------------------------
- Custom chess client and engine source code.
- Training statistics (Loss curves, ELO progression, illegal move frequency).
- Model snapshots at defined epochs for all experimental branches.
- Comparative analysis: Masked vs. Unmasked, Legal vs. Pseudo-Legal, EGTB schedules.

6. AREAS OF IMPROVEMENT
-------------------------------------------------------------------------------
- Custom chess client and engine source code.
- Get board masks directly from move generator
- Implement magic bitboards

