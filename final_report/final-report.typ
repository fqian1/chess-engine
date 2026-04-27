// ========================================== // DOCUMENT SETUP // ==========================================
#set page(
  paper: "a4",
  margin: (x: 3cm, y: 3cm),
  numbering: "1",
)

#set text(
  font: "New Computer Modern",
  size: 11pt,
  lang: "en",
)

#set par(
  justify: true,
  leading: 0.65em,
)

#import "@preview/lovelace:0.3.1": *

// ========================================== // CUSTOM FUNCTIONS // ==========================================

// Function to format Chapter headings
#show heading.where(level: 1): it => {
  pagebreak(weak: true)
  v(10%)

  if it.numbering != none {
    text(size: 24pt, weight: "bold")[
      Chapter #counter(heading).display()
    ]
    parbreak()
    v(0.5cm)
  }
  text(size: 28pt, weight: "bold")[
    #it.body
  ]
  v(1.5cm)
}

// Function for unnumbered front-matter headings
#let front-heading(title) = {
  pagebreak()
  v(10%)
  text(size: 28pt, weight: "bold")[#title]
  v(1.5cm)
}

// ========================================== // PAGE 1: TITLE PAGE // ==========================================
#page(numbering: none)[
  #align(center)[
    #v(2fr)
    #text(
      size: 20pt,
    )[Does the enforcement of legal play through logit masking and environment constraints accelerate agent convergence?]

    #v(2cm)
    by
    #v(2cm)

    #text(size: 16pt)[Francois Qian] \
    URN: 6702759/7

    #v(3cm)

    A dissertation submitted in partial fulfilment of the \
    requirements for the award of

    #v(1.5cm)

    #text(size: 16pt)[BACHELOR OF SCIENCE IN COMPUTER SCIENCE]

    #v(1.5cm)

    May 2026

    #v(2cm)

    Department of Computer Science \
    University of Surrey \
    Guildford GU2 7XH

    #v(2fr)

    // Use a block to ensure left alignment stays relative to the center
    #align(left)[Supervised by: Nishanth Sastry]
  ]
]

// ========================================== // PAGE 2: DECLARATION // ==========================================
#page(numbering: none)[
  // Removed v(1fr) to match PDF top alignment
  #v(2cm)
  I declare that this dissertation is my own work and that the work of others is acknowledged and indicated by explicit
  references.

  #v(1cm)
  Francois Qian \
  May 2026
  #v(1fr)
]

// ========================================== // PAGE 3: COPYRIGHT // ==========================================
#page(numbering: none)[
  #align(center + horizon)[
    #sym.copyright Copyright Francois Qian, May 2026
  ]
]

// ========================================== // PAGE 4: ABSTRACT // ==========================================
#set page(numbering: "1")
#counter(page).update(3)

#front-heading("Abstract")

//TODO! do this at the end

1. The convergence of self-play in a pseudo-legal environment toward strict legal play.
2. The impact of logit masking on learning dynamics, specifically regarding the "grokking" of physical board mechanics.

In the development of neural agents for combinatorial games, a fundamental tension exists between explicit policy pruning and emergent rule internalization. This thesis investigates two primary paradigms: *Logit Masking* and *Punishment Propagation*.

- *Hypothesis:* While logit masking provides superior convergence speed, unmasked training (Punishment Propagation) forces the network to "grok" the underlying physical mechanics of the environment, potentially yielding more robust latent representations.


// ========================================== // PAGE 5: ACKNOWLEDGEMENTS // ==========================================
#front-heading("Acknowledgements")

I would like to thank my supervisor, Nishanth Sastry, for overseeing the project.

// ========================================== // PAGE 6: CONTENTS // ==========================================
#show outline.entry.where(
  level: 1,
): it => {
  v(12pt, weak: true)
  strong(it)
}

#outline(
  title: front-heading("Contents"),
  indent: auto,
  depth: 3,
)

// ========================================== // PAGE 7: LIST OF FIGURES // ==========================================
#outline(
  title: front-heading("List of Figures"),
  target: figure.where(kind: image),
)

// ========================================== // PAGE 8: LIST OF TABLES // ==========================================
#outline(
  title: front-heading("List of Tables"),
  target: figure.where(kind: table),
)

// ========================================== // PAGE 9: GLOSSARY // ==========================================
#front-heading("Glossary")

#grid(
  columns: (1.5cm, 1fr),
  row-gutter: 1.5em,
  column-gutter: 1em,

  [$Q(s, a)$], [The total reward accumulated from state],
  [$C(s)$], [CPUCT],
  [$S_i$], [Upper confidence bound],
  [$overline(x_i)$], [The Mean Value of the current node],
  [$c$], [The Exploitation parameter],
  [$N$], [The current node's Visit Count],
  [$n_i$], [The Visit Count of the child],
)

// ========================================== // PAGE 10: ABBREVIATIONS // ==========================================
#front-heading("Abbreviations")

#grid(
  columns: (2cm, 1fr),
  row-gutter: 1.5em,

  [MCTS], [Monte Carlo Tree Search],
  [UCB1], [Upper Confidence Bound 1],
  [PUCT], [Predictor + Upper Confidence Bound applied to Trees],
  [CPUCT], [Exploration Predictor Constant],
)

// ========================================== // MAIN CONTENT START // ==========================================
#set heading(numbering: "1.1")

// ========================================== // CHAPTER 1: INTRODUCTION // ==========================================
= Introduction
== Project Background
Modern machine learning often relies on scaling data and compute over opaque architectures. However, these models are frequently decoupled from environment transition dynamics via *Logit Masking*—a heuristic that artificially zeroes out the probability of illegal actions prior to softmax activation. While computationally efficient, this deprives the network of negative feedback for invalid predictions, bypassing the model's need to internally represent the mechanics of play. Furthermore, standard reinforcement learning (RL) agents are prevented from making invalid state transitions by hardcoded environmental constraints. This precludes the observation of how internal action space pruning influences learning before strategic evaluation develops. Chess provides an optimal substrate for this investigation: it presents highly ambiguous intermediate states bound by objective ground truths and a mathematically perfect, yet computationally intractable, solution space.

== Project Description
Existing literature rarely assesses the impact of these environmental guardrails on the learning dynamics of transformer-based agents. This project investigates two orthogonal axes of environmental constraint: Action Space Constraints and Logit Masking.

To facilitate explicit measurement of model interpretability, a custom two-pass autoregressive transformer encoder and a bipartite Monte Carlo Tree Search (MCTS) are introduced. This architecture decouples piece selection from destination selection, providing a granular window into the model's spatial reasoning.

The experimental configurations are defined across two axes:
+ *Axis 1: Rule Set Convergence (Action Space Constraints)*
  - *Control:* Training on a strict legal ruleset.
  - *Test:* Training on a pseudo-legal ruleset where the terminal state is the simpler, denser king capture $(K_"cap")$ over the highly sparse, complex ($A_l = nothing$). This variant of Chess fundamentally alters the MDP reward function to $R(s, a)$ yielding a potentially smoother reward landscape.
  - *Hypothesis:* The network first bootstraps learning the rules and goals of the environment. Ideal play in the pseudo-legal space $A_P$ may converge to standard Chess where concepts such as pins and checkmates emerge as survival heuristics to prevent or secure king capture. Conversely, the removal of environmental guardrails may degrade MCTS value estimation, preventing policy bootstrapping.

+ *Axis 2: Logit Masking*
  - *Control:* Masked logits (illegal moves are mathematically eliminated prior to selection).
  - *Test:* Unmasked logits (illegal moves are permitted by the network but penalized by the environment/loss function).
  - *Hypothesis:* Unmasked training will incur an initial sample-efficiency penalty by introducing the dual-optimization problem of learning game mechanics alongside strategy, but may lead to more robust internal representations reflecting potentially stronger play long term.

== Aims and Objectives
The primary aim of this project is to train multiple transformer-based agents via self-play reinforcement learning across varying rule sets and action spaces, quantitatively evaluating their performance, learning trajectories, and emergent behaviors.

Specific objectives include:
- Implement a highly optimized, dual-ruleset (legal and pseudo-legal) Chess library to facilitate high-throughput RL self-play.
- Architect and implement a two-pass autoregressive transformer and bipartite MCTS to explicitly model piece-then-square action selection.
- Train four distinct model configurations via self-play: (Legal, Masked), (Legal, Unmasked), (Pseudo-Legal, Masked), and (Pseudo-Legal, Unmasked).
- Quantitatively evaluate convergence rates, illegal move proposal frequencies, and the emergence of rule-abiding play across the unconstrained configurations.
- Correlate the emergence of legal play with the attention distributions of the two-pass encoder to quantify spatial reasoning.

== Thesis Structure
The remainder of this dissertation is structured as follows:
- *Chapter 2* reviews the literature surrounding reinforcement learning, transformer architectures, and action masking.
- *Chapter 3* details the system design, including the bipartite MCTS and two-pass encoder architecture.
- *Chapter 4* outlines the implementation details of the Rust engine and training pipeline.
- *Chapter 5* presents the results and evaluation of the four experimental configurations.
- *Chapter 6* concludes the project and discusses avenues for future work.

// ========================================== // CHAPTER 2: LITERATURE REVIEW // ==========================================
// Grokking: Generalization beyond overfitting on small algorithmic datasets [power2022grokking]
// XXII. Programming a computer for playing chess (Shannon) [Shannon01031950]
// An overview of the action space for deep reinforcement learning [zhu2021overview]
// Mastering Chess and Shogi by Self-Play with a General Reinforcement Learning Algorithm (Alphazero) [silver2017masteringchessshogiselfplay]
// Mastering Atari, Go, chess and shogi by planning with a learned model (Muzero) [Schrittwieser_2020]
// A Closer Look at Invalid Action Masking in Policy Gradient Algorithms (huang) [DBLP:journals/corr/abs-2006-14171]
// Mastering Chess with a Transformer Model (2024) [monroe2026chessformer]
// Evidence of Learned Look-Ahead in a Chess-Playing Neural Network (2024) [jenner2024evidencelearnedlookaheadchessplaying]
// Acquisition of Chess Knowledge in AlphaZero (2021) [DBLP:journals/corr/abs-2111-09259]
// Giraffe: Using Deep Reinforcement Learning to Play Chess (2015) [DBLP:journals/corr/Lai15a]
// Curriculum learning [10.1145/1553374.1553380]
// Action Space Shaping in Deep Reinforcement Learning (Kanervisto et al) [9231687]
// Factored Action Space Representations for Deep Reinforcement learning [DBLP:journals/corr/SharmaSRR17]

= Literature Review
== Chess as an MDP
Chess is a deterministic, zero-sum, perfect-information game. It is formally modeled as a Markov Decision Process (MDP) defined by a tuple $(S, A, T, R)$, where $S$ is the state space ($|S| approx 10^{40}$), $A$ is the action space, $T: S times A -> S$ is the deterministic transition function, and $R: S times A -> {-1, 0, 1}$ is the reward function @Shannon01031950.

Standard RL implementations enforce a legal action space $A_L(s) subset A$ where $A_L$ contains only moves that comply with the rules of chess (e.g., preventing self-check) @fide2023laws. This project introduces a relaxed pseudo-legal action space $A_P(s)$ where $A_L(s) subset A_P(s) subset A$, permitting all geometrically valid piece movements and shifting the terminal condition from the abstract checkmate to the explicit king capture $(K_"cap")$.

Typical agents produce a single policy distribution representing every possible piece movement on all 64 squares ($|A| = 4672$). Factoring the joint probability $P(a | s)$ into $P("from" | s) times P("to" | s, "from")$ reduces output layer parameters significantly (e.g., from $"d_model" times 4672$ to $"d_model" times 64$).

For any state $s$, an optimal policy $pi^*$ trained in $A_P$ must implicitly learn to prune the action space, such that:
$ pi^*(a | s) -> 0 quad "for all" a in (A_P(s) backslash A_L(s)) $
In this paradigm, concepts such as pins and checkmates are not hardcoded environmental constraints, but emergent survival heuristics within the value landscape.

== Chess Played by Machines
=== AlphaZero and MuZero
The paradigm of self-play RL in chess was defined by AlphaZero @silver2017masteringchessshogiselfplay, which combined Monte Carlo Tree Search (MCTS) with deep convolutional neural networks (CNNs) for policy and value evaluation. However, AlphaZero relies entirely on a hardcoded environment to generate $A_L(s)$, bypassing the need for the network to internalize the rules of the game.

Subsequent architectures, such as MuZero @Schrittwieser_2020, removed the need for a known transition dynamics model by learning a latent environment simulator. While MuZero demonstrates the ability to adhere to environment constraints implicitly within its hidden state transitions, its policy head still simulates within the legal action space during MCTS rollouts. The literature lacks insight into how the removal of environmental guardrails or action masking impacts the sample efficiency and representational robustness of the underlying network.

=== Transformers in Chess
Recent advancements demonstrate the efficacy of Transformer architectures in chess, shifting away from the spatial inductive biases of CNNs. #cite(label("monroe2024masteringchesstransformermodel")) demonstrated that Transformers can achieve Grandmaster-level performance without explicit search, relying entirely on attention mechanisms to evaluate board states. Furthermore, #cite(label("jenner2024evidencelearnedlookaheadchessplaying")) provided evidence of learned look-ahead within chess-playing neural networks, showing that attention heads naturally encode future board states.

Standard chess engines flatten the action space into a single discrete distribution. Decoupling piece selection $P("from" | s)$ and destination selection $P("to" | s, "from")$ forces attention mechanisms to explicitly map geometric constraints. This factored approach provides an observable window into distinct failure modes: attention failure (identifying movable pieces) and kinematic failure (how pieces move).

== Invalid Action Masking
In policy gradient algorithms, it is standard practice to prevent the selection of illegal actions via Invalid Action Masking (Logit Masking). Masking applies a transformation to the policy logits $z$ prior to the softmax activation:

$ P(a_i | s) = exp(z_i + M_i) / (sum_j exp(z_j + M_j)) $

where $M_i = -infinity$ if $a_i limits(in.not)A_L(s)$, and $0$ otherwise.

While employed to accelerate convergence, #cite(label("DBLP:journals/corr/abs-2006-14171")) demonstrated that invalid action masking significantly improves sample efficiency by collapsing the exploration space. However, masking prevents punishment from invalid actions from propagating back through the network. This is critical for MCTS algorithms, where simulations into invalid state transitions degrade search efficiency and poison the policy head.

== Learning Dynamics
The requirement for an unmasked network to learn the rules of chess from scratch introduces complex learning dynamics. #cite(label("power2022grokking")) identified the phenomenon of "grokking," where neural networks trained on small algorithmic datasets exhibit delayed generalization long after overfitting the training data. The phenomenon arose from the result of weight decay pressure, showcasing the importance of weight regularization. Chess is an algorithmic dataset embedded within a strategic MDP, showcasing the potential for analogous phase transitions.

#cite(label("10.1145/1553374.1553380")) formalized curriculum learning, finding that machine learning models converge more effectively if training data is organized in a sequence of increasing complexity. The authors demonstrate this strategy effectively functions as a regularization method, guiding the model to superior local minima often inaccessible through data shuffling. Theoretically, a pseudo-legal environment serves as an implicit curriculum, where an agent first receives rewards through piece kinematics and king capture before developing legal survival heuristics.

#cite(label("DBLP:journals/corr/SharmaSRR17")) introduced a framework that exploits the compositional nature of discrete action spaces by factoring them into independent basis-like components. This enables cross-action learning, allowing the agent to generalize actions that share common factors.

== Summary and Research Gap
While the efficacy of action masking is well-documented in general RL #cite(label("DBLP:journals/corr/abs-2006-14171")), and Transformers have proven capable of modeling chess @jenner2024evidencelearnedlookaheadchessplaying, the intersection of these domains remains unexplored. Existing literature does not isolate the impact of environmental guardrails on the representational quality of Transformer-based MCTS agents.

While the efficacy of action masking is well-documented, and Transformers have proven capable of modeling chess, the intersection of these domains remains unexplored. In the context of unmasked, pseudo-legal chess training, the network faces a dual-optimization problem: it must learn the physical constraints of the board (the rules) and the strategic evaluation of states (the game). We hypothesize that the network will exhibit a grokking-like phase transition: an initial period of high illegal-move proposals, followed by a sharp convergence toward $A_L(s)$ as physical constraints are internalized, preceding the development of high-level strategy.

In the context of unmasked, pseudo-legal chess training, the network faces a dual-optimization problem: it must learn the physical constraints of the board (the rules) and the strategic evaluation of states (the game). It is hypothesized that the network will exhibit a grokking-like phase transition: an initial period of high illegal-move proposals (random walk), followed by a sharp convergence toward $A_L(s)$ as physical constraints are internalized, preceding the development of high-level strategy.

This project addresses this gap by training four distinct configurations across two orthogonal axes: Action Space Constraints (Legal vs. Pseudo-Legal) and Logit Masking (Masked vs. Unmasked). By utilizing a factored two-pass autoregressive encoder, this research provides a novel, quantitative analysis of how rule internalization and action space pruning manifest within the attention layers of a chess-playing agent.

// ========================================== // CHAPTER 3: DESIGN // ===========================================
= System Design and Methodology
== System Architecture Overview
The system is designed as a closed-loop reinforcement learning pipeline, decoupling the environment transition dynamics from the neural network's internal representations. The architecture consists of three primary components: a custom deterministic environment simulator supporting dual rulesets, a two-pass autoregressive Transformer encoder, and a bipartite Monte Carlo Tree Search (MCTS) algorithm. The system operates via asynchronous self-play, generating trajectories that are aggregated into a global replay buffer to optimize the network via gradient descent.
== Environment Formulation and State Representation
=== Dual Ruleset MDPs
+ *Legal Environment ($E_l$)*: Adheres to standard FIDE rules. The action space ($A_l(s)$) is strictly contained to moves that do not leave the king in check. The terminal reward $R(s, a)$ is evaluated upon Checkmate, Stalemate or standard draw conditions.
+ *Pseudo-Legal Environment ($E_p$)*: Relaxes the action space ($A_p(s)$), permitting all geometrically valid piece movements regardless of king safety. The terminal conditions is shifted to the explicit king capture ($K_"cap"$). FIDE draw conditions are maintained to prevent infinite rollouts and support correct transition into legal play.
=== State Canonicalization and Tensor Formulation
To enforce symmetric learning and halve the state space, all board states are canonicalized to the perspective of the side-to-move. If it is Black's turn, the board and castling rights are geometrically flipped.
The state $s$ is mapped to the spatial tensor representation $X in RR^(64 times 14)$ and a scalar meta-tensor $M in RR^5$.
- *Spatial Tensor*: Comprises 12 piece bitboards (6 piece types $times$ 2 colors), 1 en-passant target plane, and 1 selected-square plane (utilized in the second pass of the network).
- *Meta Tensor*: Encodes the 4 castling rights (one-hot) and the half-move clock (scaled by $1/100$).

To explicitly measure spatial reasoning, the system factors the joint probability of a move $P(a|s)$ into an autoregressive sequence: $P(a|s) = P("from"|s) times P("to"|s,"from")$. The network executes two forward passes using shared weights:
+ *Pass 1 (Piece Selection)*: The network evaluates the board state with the selected square plane zeroed out, outputting a policy distribution over the 64 squares to select the origin square (_from_). The value head will evaluate the board state.
+ *Pass 2 (Destination Selection)*: The origin square is encoded into the 14th plane of the spatial tensor. The network re-evaluates the state, outputting a policy distribution over the 64 squares to select the destination (_to_). The value head will evaluate the position given a piece to move, effectively evaluating the viability of the selected piece.

_Note on Promotions_: Because the factored $64 -> 64$ architecture lacks a third dimension for promotion piece selection, promotions are handled dynamically during MCTS expansion. If a pawn reaches the 8th rank, the destination edge is expanded into four distinct sub-edges (Q, R, B, N), dividing the prior probability equally.

=== Network Topology
The model uses a Transformer Encoder architecture without spatial inductive biases (e.g., convolutions). The spatial tensor $X$ is linearly projected to $d_"model"$, and $2D$ learned positional embeddings are added to retain geometric context. The meta-tensor $M$ is linearly projected and concatenated as a pseudo-[CLS] token, resulting in a sequence length of 65.

The model is scaled symmetrically:
- $n_"heads" = 8$, $n_"layers" = 8$, $d_"model" = 8 * 64$, $d_"ff" = 8 * 4 * 64$

=== Output Heads
The final latent representation is routed into two distinct heads:
+ *Policy Head*: A linear projection of the 64 spatial tokens to $RR^64$, representing unnormalised logits for square seletion.
+ *Value Head*: A linear projection of the [CLS] token to $RR^3$, passed through softmax to predict Win/Draw/Loss probabilities. In Pass 1, this evaluates the raw state; in Pass 2, it evaluates the state conditioned on the highlighted origin square.

=== Bipartite Monte Carlo Tree Search (MCTS)
To accomodate the two-pass autoregressive policy, a bipartite MCTS is implemented. The search tree alternates between two node types:
- *Selection Nodes ($N_"select"$)*: Represents a board state. Edges represent selection of an origin square.
- *Action Nodes ($N_"move"$)*: Represents a board state plus selected origin square. Edges represent the destination square, transitioning the environment to an $N_"select"$ node.

=== Selection and PUCT Formulation
During traversals, edges are selected using a modified Predictor + Upper Confidence Bound applied to Trees (PUCT) algorithm. To mitigate draw-seeking behaviour, a contempt factor is baked into the empirical action value $Q(s, a)$.

- $Q(s, a) = W - L - 0.05D$
- $U(s, a) = c_"puct"P(s, a) sqrt(sum_b N(s, b))/(1+N(s, a))("Exploration")$
- $"PUCT"(s, a) = Q(s, a) + U(s, a)$

where $W, D, L$ are the mean value estimates for the edge, $P(s, a)$ is the prior probability, and $N$ is the visit count.
=== Exploration Mechanisms
To ensure robust exploration of the state space:
+ *Dirichlet Noise*: Applied exclusively to the root node prior probabilities: $P(s, a) = (1 - epsilon)P(s, a) plus epsilon eta$, where $eta ~ "Dir"(alpha)$ with $alpha = 0.3$ and $epsilon = 0.25$.
+ *Temperature Scaling*: Move selection at the node is sampled proportionally to $N(s, a)^(1/t)$. The temperature $t$ scales dynamically based on ply count and remaining material, transitioning from exploratory in the opening to deterministic in endgames.

=== Simulation Integrity and Masking
During MCTS rollouts, invalid state transitions poison the value estimations of the tree. To maintain simulation integrity, action masking is applied during the search phase across all configurations. However, in the "Unmasked" experimental configurations, the network's raw logits are not masked during the calculation of the loss function, forcing the network to internalize the penalty for assigning probability mass to illegal moves.

== Reinforcement Learning Pipeline
=== Self-Play and Replay Buffer
The system generates data via highly parallelized self-play. Trajectories are stored in a First-In-First-Out (FIFO) replay buffer with a capacity of 524,288 samples. To prevent infinite games, rollouts are forcibly terminated as draws after 400 plies, or if the network's internal value estimation for a draw exceeds 0.95, scaled to 0.75 in the endgame.

== Optimisation and Loss Function
The network is optimised using the AdamW optimiser with $beta_1 = 0.9$, $beta_2 = 0.99$ and weight decay = $10^(-4)$. Noam learning rate scheduler is used with a factor of 0.1 and 4000 warmup steps.

For a given state $s$, let $pi$ be the MCTS visit count distribution, $p$ be the network's policy prediction, $z$ be the true game outcome, and $v$ be the network's value prediction. The loss function is defined as:
$L = (1 + lambda) (- limits(sum)_a pi(a|s) log p(a|s)) plus (1 - lambda) (- limits(sum)_i z_i log v_i)$

This acts as a strict mathematical curriculum. When the network proposes many illegal moves $lambda approx 1$, the policy loss approaches $2$ and the value loss approaches $0$. The network is forced to learn piece kinematics before state evaluation. As the model internalises piece kinematics, the weights balance to $1:1$. This approach to curriculum learning is unique to the unmasked configuration, as $lambda$ is dependent on illegal move weights ($lambda$ is always zero in masked configurations).

== Experimental Design and Evaluation Metrics
=== The Configuration Matrix
To isolate the effects of environmental guardrails, four distinct models are trained from scratch using identical hyperparameters ($256$ batch size, $256$ MCTS simulations per move). The configurations form a $2 times 2$ matrix:
+ *Legal $plus$ Masked (Control)*: Standard AlphaZero paradigm.
+ *Legal $plus$ Unmasked*: Network must learn piece kinematics via loss penalties before strategy.
+ *Pseudo-Legal $plus$ Masked*: Network plays the $K_"cap"$ variant, and must learn FIDE rules before strategy.
+ *Pseudo-Legal $plus$ Unmasked*: Network must learn piece kinematics and FIDE rules entirely from scratch.

=== Quantitative Metrics
The learning dynamics of each configuration are evaluated using the following metrics, logged continuously during training:
- *Average Illegal Probability*: The total probability mass assigned by the raw policy head to invalid actions, quantifying the network's internalization of the ruleset.
- *Average Game Length*: Measures in plies, indicating the transition from random walks to strategic, prolonged play.
- *Win/Draw Ratios*: Tracking the emergence of decisive outcomes versus stalemates or repetitions.
- *Value Loss Convergence*: Measures the accuracy of the network's state evaluation over time.

// ========================================== // CHAPTER 4: Implementation // ==========================================
= Implementation Details
== Software Stack and Overview
The system is implemented in Rust, utilising its strict aliasing rules to safely parallelise MCTS rollouts without runtime data-race checks. The neural network is implemented using the Burn deep learning framework to enable backend-agnostic tensor operations (WGPU/CUDA). The complete source code is version controlled via a Nix flake.

The main execution flow is as follows:
#pseudocode-list(booktabs: true, title: smallcaps("The Loop"))[
  *Algorithm:* Self-Play Reinforcement Learning Loop

  + *Initialization*
    + Load Model weights and initialize Optimizer.
    + Initialize Replay Buffer and $B$ parallel Chess games.

  + *Self-Play Phase (Data Generation)*
    + *For each* step in training iteration:
      + *MCTS Simulation:*
        + *For* $N$ simulations:
          + Traverse trees to terminal leaf nodes.
          + Expand and evaluate leaves in batch using Model.
      + *Action Selection:*
        + Generate policy and value targets from tree statistics.
        + Execute best move in Environment.
        + Apply Dirichlet noise to root for exploration.
        + *If* Game meets draw threshold or move limit: Terminate and reset.
      + *Store:* Push resulting samples into Replay Buffer.

  + *Optimization Phase (Learning)*
    + *If* Replay Buffer contains sufficient samples:
      + *For* each gradient epoch:
        + Sample batch from Buffer.
        + Update Model parameters via Optimizer.
        + Step Learning Rate Scheduler.

  + *Checkpointing*
    + Log metrics (loss, game length, win/draw rates) to CSV.
    + *If* iteration interval reached: Save Model snapshot to disk.
]

== Chess Engine Core
=== Bitboard layout and memory footprint:
The full board state is packed primarily into 15 64-bit Unsigned integers `struct Bitboard(u64)`, representing the spatial occupancy of each piece type and colour, plus three occupancy bitboards for white, black and all. This reduces the memory footprint to 120 bytes, ensuring optimal CPU cache locality. This enables the use of hardware-level intrinsics `trailing_zeros` and `leading_zeroes` to find the LSB or MSB of a bitboard.

=== Compile‑time mask generation:
Masks are precomputed drectional rays (e.g., `ROOK_ATTACKS` split into cardinal directions) evaluated at compile time using `const fn`. A between mask is also created.

=== Move generation with blocker isolation:
The engine isolates the correct directional mask to use at runtime, and uses `pop_lsb` or `pop_msb` to lookup the blocking square and get the correct `BETWEEN` mask to iterate over when adding viable squares the sliding piece can move to (Brian Kernighan's algorithm.

=== Stack‑allocated moves:
Move generation utilizes stack-allocated `ArrayVec<ChessMove, 128>`. While the theoretical maximum branching factor in chess is 218, empirical average branching is $approx 30$. A 128-element capacity prevents heap fragmentation and allocator bottlenecks while safely encompassing $>99.9\%$ of reachable states. Moves are stored in ChessPosition itself, to easily decompose into available squares to select pieces from, and available destinations given a selected piece for both select and move nodes.

=== Zobrist hashing:
To detect three-fold repetitions and cache MCTS evaluations, the engine implements Zobrist Hashing. A static table of pseudo-random 64-bit integers is generated and initialised via OnceLock. The hash of any state is the `XOR` sum of the keys corresponding to the pieces, castling rights, and en-passant files. Currently, the implementation suffers from $O(P)$ time complexity calculation from scratch where $P$ is the number of pieces on the board, as opposed to $O(1)$ for incremental updates. This trade off was made for ease of implementation.

== Bipartite Monte Carlo Tree Search
Standard MCTS implementations for chess utilize a unipartite node structure where edges represent complete moves (e.g., $"e2e4"$). To explicitly model the autoregressive two-pass network, a bipartite MCTS was engineered. The tree strictly alternates between `PieceSelect` nodes (state $s$) and `PieceMove` nodes (state $s$ + origin square $u$). This guarantees the search topology perfectly mirrors the factored action space, allowing the network's intermediate spatial reasoning to be cached and traversed

== Node and edge types:
To handle the two-pass architecture efficiently, the network processes both `PieceSelect` and `PieceMove` inputs identically. The only distinction is the presence of a single active bit in the 14th plane (the selected square) during the second pass.
```rust
enum MctsNode {
    PieceSelect { data: NodeData },
    PieceMove { data: NodeData, from_sq: ChessSquare },
}
```
This bipartite structure explicitly models the two-pass autoregressive nature of the network. A `PieceSelect` node transitions via an edge (the origin square) to a `PieceMove` node, which transitions via an edge (the destination square) to the next `PieceSelect` node. This guarantees that the MCTS topology perfectly mirrors the factored action space $P("from"|s) times P("to"|s, "from")$.

== Arena allocators:
Each MCTS is initialised from a `ChessGame` with 3 pre-allocated Vecs serving as arenas for Nodes, Edges and Positions. Due to the bipartite nature of the search tree, consecutive Select and Move nodes have the same chess position, differing only in selected piece. As such, a separate position arena is used to halve memory consumption. As each game instance has their own MCTS, operations can be easily parallelised without mutexes.

== Node expansion:
add leaf node... terminal state 3 fold repetition and stuff, look the pseudo code
#pseudocode-list(booktabs: true, title: smallcaps("Leaf Node Addition and Evaluation"))[
  *Algorithm:* Leaf Node Addition and Evaluation

  + *Input:* Edge index $e$ to be expanded
  + *Path Tracking:*
    + Initialize $H$ (path hashes) with current Root position hash.
    + *For each* node in the current selection path:
      + *If* node is `PieceSelect`: Add position Zobrist hash to $H$.

  + *Validation:*
    + *If* $e$ already points to a child node: *Return* None.

  + *State Transition Logic:*
    + Let $P$ be the parent node of $e$.
    + *If* $P$ is a `PieceSelect` node:
      + Create `PieceMove` node (storing selected square).
      + Link $e$ to the new node and *Return*.

    + *Else if* $P$ is a `PieceMove` node:
      + *State Generation:*
        + Retrieve board from $P$; apply move defined by $e$.
        + Compute new Zobrist hash $h_"new"$.
      + *Repetition Detection:*
        + Count occurrences of $h_(n e w)$ in global history and local path $H$.
        + *If* count $\ge 2$: Set state to `Finished(Draw)`.
        + *Else*: Set state to `check_game_state()`.

      + *Arena Integration:*
        + *If* $h_(n e w)$ exists in `position_arena`: Reuse index.
        + *Else*: Push new position to arena and use new index.

      + *Terminal Evaluation:*
        + Create `PieceSelect` node.
        + *If* state is `Finished`:
          + Mark node as *Terminal*.
          + Map reward vector $[W, D, L]$ relative to current side-to-move.

      + Link $e$ to final node and *Return*.
]

=== Value Propagation:
```rust
pub fn backprop(&mut self, value: [f32; 3], color: Color) {
    self.path.iter().for_each(|&idx| {
        let edge = &mut self.edge_arena[idx];

        let parent = &self.node_arena[edge.parent_node_idx];
        let position = &self.position_arena[parent.get_data().chess_position_idx];
        let side_to_move = position.side_to_move;

        let value = if color != side_to_move { [value[2], value[1], value[0]] } else { value };

        edge.total_value[0] += value[0];
        edge.total_value[1] += value[1];
        edge.total_value[2] += value[2];
        edge.visits += 1;
        edge.mean_value =
            [edge.total_value[0] / edge.visits as f32, edge.total_value[1] / edge.visits as f32, edge.total_value[2] / edge.visits as f32];
        self.node_arena[edge.parent_node_idx].get_data_mut().visits += 1;
    });
}
```

=== Promotion splitting:
The engine detects when a promotion occurs, and creates 4 edges dividing the prior equally between them, before creating the leaf nodes and positions with promotion piece. The network considers promotion as a single move, rather than 4 distinct moves.

#pseudocode-list(booktabs: true, title: smallcaps("Bipartite MCTS Expansion and State Transition"))[
  *Algorithm:* Promotion handling
  + *If* node == PieceMove
    + *If* side_to_move is `Black`; flip square
    + *If* move is promotion; Push edges with promotion piece to Edge Arena dividing prior among edges equally
      + *Else* Push edge without promotion piece
  + *Else* add Edge
]

=== PUCT and edge selection:
the custom Q = W − L − 0.05D, and the PUCT formula with exploration constant. Include the select_best_edge function or a snippet.

=== Mask application and renormalisation:
During node expansion, the same mask used to zero out logits in masked configurations is reused (or used for the first time in unmasked configurations) to decide which edges to add to the node, to prevent invalid state transitions.
In unmasked configurations, policies are renormalised over valid squares to maintain mathematical integrity in PUCT calculations.
```rust
if !config.masked {
    policy.iter_mut().for_each(|e| e.1 = e.1 / (1.0 - rate) as f32);
}
```
where rate is total prior in invalid squares.

In masked configurations, logits are set to -1e9. 
```rust
if let Some(masks) = masks {
    let mask_data = TensorData::new(masks, [batch_size, 64]);
    let mask = Tensor::<B, 2, Bool>::from_data(mask_data, device);
    let mask = mask.bool_not();

    policies = policies.clone().mask_fill(mask, -1e9);
}
```
in unmasked configurations, output distribution is renormalised over valid squares.
```rust
let rate: f64 = mask.iter().zip(policy.iter()).map(|(legal, policy)| if !legal { policy.1 as f64 } else { 0.0 }).sum();
if !config.masked {
    policy.iter_mut().for_each(|e| e.1 = e.1 / (1.0 - rate) as f32);
}
```

=== PUCT and edge selection
#pseudocode-list(booktabs: true, title: smallcaps("Edge Selection"))[
  + *Function* select_best_edge(node_idx):
    + node $arrow$ self.node_arena[node_idx]
    + *if* node.child_edge_range is None *then*
      + *return* None
    + *end*
    + (start, end) $arrow$ node.child_edge_range
    + $N$ $arrow$ node.visits
    + max_score $arrow -infinity$
    + best_edge $arrow$ None
    + *for* each edge_idx *from* start *to* end:
      + edge $arrow$ self.edge_arena[edge_idx]
      + $(w, d, l) arrow$ edge.mean_value
      + exploitation $arrow w - l - 0.05 dot d$
      + exploration $arrow$ edge.confidence $dot$ config.c_puct $dot$ frac(sqrt(N) + 10^(-8), 1 + edge.visits)
      + total_score $arrow$ exploitation + exploration
      + *if* total_score $>$ max_score *then*
        + max_score $arrow$ total_score
        + best_edge $arrow$ edge_idx
      + *end*
    + *end*
    + *return* best_edge
]

=== Parallel search:
Self-play throughput is maximized by parallelizing MCTS rollouts across the batch size. Using the `rayon` crate, the system executes tree traversals concurrently (`mctss.par_iter_mut()`). Because the Arena Allocator isolates each MCTS instance in contiguous memory, data races are structurally impossible, and cache invalidation is minimized.

== Neural Network and Tensor Formulation
=== State canonicalisation:
The board is canonicalized to the perspective of the side-to-move when making tensors. This halves the required state-space exploration for the network.
```rust
NetworkInputs::from_position(position: &ChessPosition, selected_sq: Option<&ChessSquare>) -> Self {
    let (chess_board, castling_rights, ep_sq) = if position.side_to_move == Color::White {
        (position.chessboard.clone(), position.castling_rights, position.en_passant)
    } else {
        (position.chessboard.flip_board(), position.castling_rights.flip_perspective(), position.en_passant.map(|x| x.square_opposite()))
    };
    ...
}
```

=== Input tensor layout:
```rust
pub fn inputs_to_tensor<B: Backend>(buffer: &Vec<NetworkInputs>, device: &B::Device) -> (Tensor<B, 3>, Tensor<B, 2>) {
    let n = buffer.len();

    let mut boards = Vec::with_capacity(n * 64 * 14);
    let mut metas = Vec::with_capacity(n * 5);

    for item in buffer {
        boards.extend_from_slice(&item.boards);
        metas.extend_from_slice(&item.meta);
    }

    let shape = [n, 64, 14];
    let board_data = TensorData::new(boards, shape);
    let t1 = Tensor::from_data(board_data, device);

    let shape = [n, 5];
    let meta_data = TensorData::new(metas, shape);
    let t2 = Tensor::from_data(meta_data, device);

    (t1, t2)
}
```
The spatial tensor $X in RR^(64 times 14)$ reserves the 14th plane for the selected piece. During a `PieceSelect` node evaluation, this plane is strictly zeroed. 

=== Two‑pass forward:
The network executes two forward passes using shared weights. The policy head outputs $P("from"|s)$. During a `PieceMove` node evaluation, the index of the selected square is set to $1.0$ in the 14th plane. The network re-evaluates the state, outputting $P("to"|s, "from")$. This mechanism forces the attention layers to dynamically shift focus from global piece evaluation to localized kinematic projection based solely on the presence of the 14th plane.

=== Transformer architecture:
list the layers (piece_encoder, meta_encoder, positional embedding, transformer encoder, policy/value heads). Include the forward method that concatenates the meta‑token as a [CLS] token. Show the 2D positional encoding code.

```rust
// 2D Positional Encoding
let x_emb = self.pos_embedding_x.forward(self.coordinates.clone()).unsqueeze_dim(1);
let y_emb = self.pos_embedding_y.forward(self.coordinates.clone()).unsqueeze_dim(2);
let pos_flat = (x_emb + y_emb).reshape([1, 64, self.d_model]);
x = x + pos_flat;

// Meta-token concatenation (Pseudo-[CLS] token)
let meta_x = self.meta_encoder.forward(meta).unsqueeze_dim(1);
let x = Tensor::cat(vec![x, meta_x], 1);
```

=== Output processing:
the policy head outputs 64 logits → softmax; value head outputs 3 logits → softmax. Mention the model_make_outputs utility that optionally masks logits and returns NetworkLabels.

== Training Pipeline
=== Self‑play loop (play()):
give a high‑level walkthrough of the outer loop. Use a flowchart or numbered steps: initialise MCTS and games → for each step in iteration → traverse MCTS → expand batch using model → select best move → generate training sample → push to replay buffer → every steps_per_iter iterations, perform gradient steps.

=== Expand batch (expand_batch):
explain how masks are gathered, network inference is batched, edges are created, and Dirichlet noise is added at the root. Show the core code that creates edges (including promotion splitting) and updates the node’s child_edge_range.

=== Replay buffer:
describe the FIFO buffer, its size (524k samples), and the ChessBatcher that constructs tensors from sampled TrainingSamples.

The `ChessBatcher` constructs the input tensors dynamically given a slice of training samples randomly sampled from the replay buffer. The spatial tensor $X in RR^(B times 64 times 14)$ is populated by writing to flat `Vec<f32>` before being turned into tensors and reshaped.

Generated trajectories are pushed to a First-In-First-Out (FIFO) `ReplayBuffer` with a capacity of 524,288 samples. With a batch size of 256, the buffer saturates after approximately 2,048 moves. Assuming an average game length of 96 plies, the buffer contains data from the most recent $\sim 5,400$ games. This capacity was explicitly chosen to balance data freshness (preventing the network from overfitting to outdated policies) with sufficient historical context to stabilize the gradients.

=== Self‑annealing loss:
In unmasked configurations, the network must learn environmental constraints via gradient descent. This is achieved through a dynamic, self-annealing loss function. Let $lambda$ represent the average probability mass assigned to illegal moves across a batch. The loss is defined as:
$L = L_{"policy"}(1 + lambda) + L_"value" (1 - lambda)$

When the network acts randomly ($lambda approx 1$), the value loss is suppressed, forcing the optimizer to prioritize kinematic legality. As the policy converges on $A_L(s)$ ($lambda approx 0$), the weights balance, allowing strategic value evaluation to propagate. This acts as an automated curriculum, implemented directly within the `ChessTransformer::calculate_loss` method.

=== Optimiser and LR schedule:
briefly state AdamW with Noam scheduler (warmup 4000 steps) and weight decay. You can reference the configuration struct.

== Performance and Correctness
=== Correctness:
mention unit tests for move generation, perft‑like validation (i didnt do this).  describe the test strategy (e.g., comparing move counts to known perft numbers for a few FENs) than claim full verification.

=== Profiling and throughput:
report the number of MCTS simulations per second on your hardware, the average inference batch throughput, and memory usage per MCTS instance. network inference is primary bottleneck

=== Scalability:
mention that the arena‑based design allowed near‑linear scaling with the number of parallel games.

== Summary
A short paragraph recapping the key implementation contributions: the bipartite MCTS, the two‑pass network with canonicalised input, the self‑annealing loss, and the efficient Rust engine.
// ========================================== // CHAPTER 5: Results // ==========================================
= Results and Evaluation
== Experimental Setup
To ensure fairness, all variants were initialized with identical seeds, trained for the same number of epochs, and
evaluated at identical MCTS search depths.

/*
* pseudo + unmasked chart
* pseudo + masked chart
* legal + unmasked chart
* legal + masked chart
* e.g.
#figure(
  table(
    columns: (1fr, auto, auto),
    align: (left, center, center),
    stroke: (x, y) => if x > 0 { (left: 0.5pt) } else { none },
    table.hline(),
    table.header([Model Variant], [Peak ELO], [Illegal Move %]),
    table.hline(),
    [Legal Masked (Control)], [TBD], [0.0%],
    [Legal Unmasked], [TBD], [TBD%],
    [Pseudo-Legal Masked], [TBD], [0.0%],
    [Pseudo-Legal Unmasked], [TBD], [TBD%],
    table.hline(),
  ),
  caption: [Comparative performance metrics across the four experimental variants.],
)
*/

== Ruleset Convergence Analysis
== Masking and Grokking Analysis

// ========================================== // CHAPTER 6: CONCLUSION // ==========================================
= Conclusion
== Project Evaluation
== Limitations and Future Work
- *Compute Constraints:* Hardware limitations restricted the total number of training epochs, preventing observation of deep late-stage grokking.
- *Move Generation:* Future iterations should replace standard bitboard raycasting with Magic Bitboards for optimal
performance.
- *Endgame Tablebases (EGTB):* Injecting perfect knowledge at varying stages of the training pipeline remains a highly viable area for future research to bootstrap the value head and sharpen endgame play.
- *Engine Evaluations* Injecting perfect knowledge from high depth engine evaluations with forced wins similarly are viable for perject knowledge injection.
- *Kinematic Bootstrapping:* Boot strap the policy head with uniform distributions of valid moves, before playing for value.

// ========================================== // CHAPTER 6: Ethics // ==========================================
= Statement of Ethics

// ========================================== // BIBLIOGRAPHY // ==========================================
#bibliography("refs.bib", style: "harvard-cite-them-right")
