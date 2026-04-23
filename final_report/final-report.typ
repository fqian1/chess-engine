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
Modern machine learning often relies on scaling data and compute over opaque architectures. However, these models are often decoupled from environment transition dynamics, aided by *Logit Masking* - a heuristic that artificially zeroes out the probability of illegal actions prior to softmax activation. While computationally efficient, this deprives the network of feedback for invalid predictions, bypassing the models need to internally represent the mechanics of play. Furthermore, standard RL agents are constrained against making suboptimal or invalid state transitions through environmental constraints. This prevents observation of how internal action space pruning to match a more constrained environment before developing strategy influences learning. Chess provides an optimal substrate for this investigation: it presents highly ambiguous intermediate states bound by objective ground truths and a mathematically perfect, yet computationally intractable, solution space.

== Project Description
Existing literature rarely assesses the impact of these environmental guardrails on the learning dynamics of transformer-based agents. This project investgates two orthogonal axes of environmental constraint: Action Space Constraints and Logit Masking.

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
- Quantitatively evaluate the convergence rates, illegal move proposal frequencies, and the emergence of rule-abiding play across the unconstrained configurations.
- Correlate the emergence of legal play with the attention distributions of the 2 pass encoder to quantify spatial reasoning.

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
== Chess as a MDP
Chess is a deterministic, zero-sum, perfect-information game. It is formally modeled as a Markov Decision Process (MDP) defined by a tuple $(S, A, T, R)$, where $S$ is the state space ($|S| approx 10^{40}$), $A$ is the action space, $T: S times A -> S$ is the deterministic transition function, and $R: S times A -> {-1, 0, 1}$ is the reward function @Shannon01031950.

Standard RL implementations enforce a legal action space $A_L(s) subset A$ where $A_L$ contains only moves that comply with the rules of chess (e.g., preventing self-check) @fide2023laws. This project introduces a relaxed pseudo-legal action space $A_P(s)$ where $A_L(s) subset A_P(s) subset A$, permitting all geometrically valid piece movements and shifting the terminal condition from the abstract checkmate to the explicit king capture $(K_"cap")$.

Typical agents produce a single policy distribution representing every possible piece movement on all 64 squares ($|A| = 4672$). A standard chess position has approximately 30 legal moves ($|A_L(s)| approx 30$) @Shannon01031950, vastly smaller than the unrestricted action space. Factoring the joint probability $P(a | s)$ into $P("from" | s) times P("to" | s, "from")$ reduces output layer parameters significantly (e.g., from $"d_model" times 4672$ to $"d_model" times 64$).

For any state $s$, an optimal policy $pi^*$ trained in $A_P$ must implicitly learn to prune the action space, such that:
$ pi^*(a | s) -> 0 quad "for all" a in (A_P(s) backslash A_L(s)) $
In this paradigm, concepts such as pins and checkmates are not hardcoded environmental constraints, but emergent survival heuristics within the value landscape.

== Chess played by Machines
=== AlphaZero, MuZero
The paradigm of self-play RL in chess was defined by AlphaZero @silver2017masteringchessshogiselfplay, which combined Monte Carlo Tree Search (MCTS) with deep convolutional neural networks (CNNs) for policy and value evaluation. However, AlphaZero relies entirely on a hardcoded environment to generate $A_L(s)$, bypassing the need for the network to internalize the rules of the game.

Subsequent architectures, such as MuZero @Schrittwieser_2020, removed the need for a known transition dynamics model by learning a latent environment simulator. While MuZero demonstrates the ability to adhere to environment constraints implicitly within its hidden state transitions, its policy head still simulates within the legal action space during MCTS rollouts. The literature lacks insight into how the removal of environmental guardrails or action masking impacts the sample efficiency and representational robustness of the underlying network.

=== Transformers in Chess
Recent advancements have demonstrated the efficacy of Transformer architectures in chess, shifting away from the spatial inductive biases of CNNs. #cite(label("monroe2024masteringchesstransformermodel")) demonstrated that Transformers can achieve Grandmaster-level performance without explicit search, relying entirely on attention mechanisms to evaluate board states. Furthermore, #cite(label("jenner2024evidencelearnedlookaheadchessplaying")) provided evidence of learned look-ahead within chess-playing neural networks, showing that attention heads naturally encode future board states.

Standard chess engines flatten the action space into a single discrete distribution. Decoupling piece selection $P("from" | s)$ and destination selection $P("to" | s, "from")$ forces attention mechanisms to explicitly map geometric constraints. This factored approach provides an observable window into distinct failure modes: attention failure (identifying movable pieces) and kinematic failure (how pieces move).

== Invalid Action Masking
In policy gradient algorithms, it is standard practice to prevent the selection of illegal actions via Invalid Action Masking (Logit Masking). Masking applies a transformation to the policy logits $z$ prior to the softmax activation:

$ P(a_i | s) = exp(z_i + M_i) / (sum_j exp(z_j + M_j)) $

where $M_i = -infinity$ if $a_i limits(in.not)A_L(s)$, and $0$ otherwise.

While often employed to accelerate convergence and simplify design, #cite(label("DBLP:journals/corr/abs-2006-14171")) demonstrated that invalid action masking significantly improves sample efficiency and asymptotic performance in environments with large, state-dependent action spaces by collapsing the exploration space over the manifold space. @9231687 similarly emphasized the importance of action space transformations within policy gradient algorithms to create learnable environments, especially for continuous action spaces. However, masking prevents punishment from invalid actions from propagating back through the network, effectively disabling feedback for invalid decisions. This is particularly critical for MCTS algorithms, where simulations into invalid state transitions degrade search efficiency and poison the policy head.

== Learning Dynamics
The requirement for an unmasked network to learn the rules of chess from scratch introduces complex learning dynamics. #cite(label("power2022grokking")) identified the phenomenon of "grokking," where neural networks trained on small algorithmic datasets exhibit delayed generalization long after overfitting the training data. This phenomenon relies on weight decay pressure, signifying the importance of weight regularization in network robustness. Chess is similarly an algorithmic dataset embedded within a strategic MDP, showcasing the potential for analogous dynamics.

#cite(label("10.1145/1553374.1553380")) formalized curriculum learning, finding that machine learning models converge more effectively if training data is organized in a sequence of increasing complexity. The authors demonstrate this strategy effectively functions as a regularization method, guiding the model to superior local minima often inaccessible through data shuffling. Theoretically, a pseudo-legal environment serves as an implicit curriculum, where an agent first receives rewards through piece kinematics and king capture before developing legal survival heuristics.

#cite(label("DBLP:journals/corr/SharmaSRR17")) introduced a framework that exploits the compositional nature of discrete action spaces by factoring them into independent basis-like components. This enables cross-action learning, allowing the agent to generalize actions that share common factors.

== Summary and Research Gap
While the efficacy of action masking is well-documented in general RL #cite(label("DBLP:journals/corr/abs-2006-14171")), and Transformers have proven capable of modeling chess @jenner2024evidencelearnedlookaheadchessplaying, the intersection of these domains remains unexplored. Existing literature does not isolate the impact of environmental guardrails on the representational quality of Transformer-based MCTS agents.

In the context of unmasked, pseudo-legal chess training, the network faces a dual-optimization problem: it must learn the physical constraints of the board (the rules) and the strategic evaluation of states (the game). It is hypothesized that the network will exhibit a grokking-like phase transition: an initial period of high illegal-move proposal (random walk), followed by a sharp convergence toward $A_L(s)$ as the physical constraints are internalized, preceding the development of high-level strategy.

This project addresses this gap by training four distinct configurations across two orthogonal axes: Action Space Constraints (Legal vs. Pseudo-Legal) and Logit Masking (Masked vs. Unmasked). By utilizing a factored two-pass autoregressive encoder, this research provides a novel, quantitative analysis of how rule internalization and action space pruning manifest within the attention layers of a chess-playing agent.

// ========================================== // CHAPTER 3: DESIGN // ===========================================
= System Design and Methodology
== System Architecture Overview
The system is designed as a closed-loop reinforcement learning pipeline, decoupling the environment transition dynamics from the neural network's internal representations. The architecture consists of three primary components: a custom deterministic environment simulator supporting dual rulesets, a two-pass autoregressive Transformer encoder, and a bipartite Monte Carlo Tree Search (MCTS) algorithm. The system operates via asynchronous self-play, generating trajectories that are aggregated into a global replay buffer to optimize the network via gradient descent.
== Environment Formulation and State Representation
=== Dual Ruleset MDPs
+ *Legal Environment ($E_l$)*: Adheres to standard FIDE rules. The action space ($A_l(s)$) is strictly contained to moves that do not leave the king in check. The terminal reward $R(s, a)$ is evaluated upon Checkmate, Stalemate or standard draw conditions.
+ *Pseudo-Legal Environment ($E_p$)*: Relaxes the action space ($A_p(s)$), permitting all geometrically valid piece movements regardless of king safety. The terminal conditions is shifted to the explicit king capture ($K_"cap"$). FIDE draw conditions are maintained to prevent infinite rollouts and support correct transition into legal play (e.g., stalemate by no legal moves which would otherwise lead to a win in pseudo-legal play).
=== State Canonicalization and Tensor Formulation
To enforce symmetric learning and halve the state space, all board states are canonicalized to the perspective of the side-to-move. If it is Black's turn, the board and castling rights are geometrically flipped.
The state $s$ is mapped to the spatial tensor representation $X in RR^(64 times 14)$ and a scalar meta-tensor $M in RR^5$.
- *Spatial Tensor*: Comprises 12 piece bitboards (6 piece types $times$ 2 colors), 1 en-passant target plane, and 1 selected-square plane (utilized in the second pass of the network).
- *Meta Tensor*: Encodes the 4 castling rights (one-hot) and the half-move clock (scaled by $1/100$).
Standard Chess architectures flatten the action space into a single discrete distribution. To explicitly measure spatial reasoning and interpretability, this system factors the joint probability of a move $P(a|s)$ into an autoregressive sequence: $P(a|s) = P("from"|s) times P("to"|s, "from")$.

The network executes 2 forward passes using shared weights:
+ *Pass 1 (Piece Selection)*: The network evaluates the board state with the selected square plane zeroed out, outputting a policy distribution over the 64 squares to select the origin square (_from_). The value head will evaluate the board state.
+ *Pass 2 (Destination Selection)*: The origin square is encoded into the 14th plane of the spatial tensor. The network re-evaluates the state, outputting a policy distribution over the 64 squares to select the destination (_to_). The value head will evaluate the viability of the selected piece.

=== Network Topology
The model uses a Transformer Encoder architecture without spatial inductive biases (e.g., convolutions). The spatial tensor $X$ is linearly projected to $d_"model"$, and $2D$ learned positional embeddings are added to retain geometric context. The meta-tensor $M$ is linearly projected and concatenated as a pseudo-[CLS] token, resulting in a sequence length of 65.

The model is scaled symmetrically:
- $n_"heads" = 8$
- $n_"layers" = 8$
- $d_"model" = 8 * 64$
- $d_"ff" = 8 * 4 * 64$

=== Output Heads
The final latent representation is routed into two distinct heads:
+ *Policy Head*: A linear projection of the 64 spatial tokens to $RR^64$, representing unnormalised logits for square seletion.
+ *Value Head*: A linear projection of the [CLS] token to $RR^3$, passed through softmax acti

=== Bipartite Monte Carlo Tree Search (MCTS)
To accomodate the two-pass autoregressive policy, a bipartite MCTS is implemented. The search tree alternates between two node types: 
- *Selection Nodes ($N_"select"$)*: Represents a board state. Edges represent selection of an origin square.
- *Action Nodes ($N_"move"$)*: Represents a board state plus selected origin square. Edges represent the destination square, transitioning the environment to an $N_"select"$ node.

=== Selection and PUCT Formulation
During traversals, edges are selected using a modified Predictor + Upper Confidence Bound applied to Trees (PUCT) algorithm. To mitigate draw-seeking behaviour, a contempt factor is integrated into the exploitation term:

$U(s, a) = (W - L - 0.05 D)("Exploitation") + c_"puct"P(s, a) sqrt(sum_b N(s, b))/(1+N(s, a))("Exploration")$

where W, D and L are the mean value estimates for the edge, P(s, a), is the prior probability from the policy head, and $N$ is the visit count.

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
The network is optimised using the AdamW optimiser with $beta_1 = 0.9$, $beta_2 = 0.99$ and weight decay = $10^(-4)$. Noam learning rate scheduler is used with a factor of 0.1 and 4000 warmup steps. The loss function is a linear combination of policy loss and value loss.

For a given state $s$, let $pi$ be the MCTS visit count distribution, $p$ be the network's policy prediction, $z$ be the true game outcome, and $v$ be the network's value prediction. The loss function is defined as:
$L = (1 + lambda) (- limits(sum)_a pi(a|s) log p(a|s)) plus (1 - lambda) (- limits(sum)_i z_i log v_i)$
where $lambda$ is the average probability mass on illegal squares. In unmasked configurations, $p(a|s)$ includes probability mass on illegal moves, which are penalised as $pi(a|s) = 0$ for all invalid actions.

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
The project is available on github: https://github.com/fqian1/chess-engine
Dependencies are locked with a Cargo.lock file, and the toolchain and environment is versioned with Nix in the flake.nix.
== The Chess Client
=== Primitives
- ChessSquare
- ChessMove
- Color
- CaslingRights
- Zobrist Hashing
=== Bitboard
The board is represented using 12 primary bitboards (unsigned 64-bit integers), packing the state into 96 bytes. Additional convenience bitboards (occupancy masks) are maintained to accelerate move generation. Bitboards enable high-performance move generation via bitwise intrinsics. In deep learning, these are expanded into $64 times 14$ tensors.
- Consts
=== ChessBoard 
```rust 
struct ChessBoard {
    pieces: [[Bitboard; 6]; 2],
    white_occupancy: Bitboard,
    black_occupancy: Bitboard,
    all_pieces: Bitboard,
}
```
==== Masks
```rust
const WHITE_SQUARES: Bitboard;
const BLACK_SQUARES: Bitboard;
const KNIGHT_ATTACKS: [Bitboard; 64];
const KING_ATTACKS: [Bitboard; 64];
const PAWN_ATTACKS_WHITE: [Bitboard; 64];
const PAWN_ATTACKS_BLACK: [Bitboard; 64];
pub const ROOK_ATTACKS: [[Bitboard; 4]; 64];
const BISHOP_ATTACKS: [[Bitboard; 4]; 64];
const BISHOP_ATTACKS_ALL: [Bitboard; 64];
const ROOK_ATTACKS_ALL: [Bitboard; 64];
const BETWEEN: [[Option<Bitboard>; 64]; 64];
```

=== ChessPosition
An instance of a chess position.
```rust 
struct ChessPosition {
    chessboard: ChessBoard,
    side_to_move: Color,
    castling_rights: CastlingRights,
    en_passant: Option<ChessSquare>,
    halfmove_clock: u32,
    zobrist_hash: u64,
    pseudolegal_moves: ArrayVec<ChessMove, 128>,
}
```

==== Move Generation
Ray casting is optimized using constant directional masks and pre-computed "between" masks, allowing for $O(1)$ validation of sliding piece moves.

=== ChessGame
```rust
struct ChessGame {
    castling_rights: CastlingRights,
    chessboard: ChessBoard,
    en_passant: Option<ChessSquare>,
    fullmove_counter: u32,
    game_history: alloc::vec::Vec<GameStateEntry>,
    halfmove_clock: u32,
    outcome: Outcome,
    rule_set: RuleSet,
    side_to_move: chess_piece::Color,
    zobrist_hash: u64,
}
```
== The Model
=== Transformer
=== Data
3 fold repetition is handled by the environment rather than being encoded as an input.
the replay buffer is initialised with size 524288, meaning the model learns from the past approx 500000 moves made. with a batch size of 256, 2048 moves are made before saturating the replay buffer. assuming an average game length of 96, the model learns from approx the last 21 games played at a time, balancing forgetting with data freshness.

== The Monte Carlo Tree Search
=== Nodes
- Select
- Move
=== Edges
=== Arenas
=== Engine

== Main()

// ========================================== // CHAPTER 5: Results // ==========================================
= Results and Evaluation
== Experimental Setup
To ensure fairness, all variants were initialized with identical seeds, trained for the same number of epochs, and
evaluated at identical MCTS search depths.

// mention policy gradient poisoning due to mcts

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
- *Compute Constraints:* Hardware limitations restricted the total number of training epochs, preventing observation of
deep late-stage grokking.
- *Move Generation:* Future iterations should replace standard bitboard raycasting with Magic Bitboards for optimal
performance.
- *Endgame Tablebases (EGTB):* Injecting perfect knowledge at varying stages of the training pipeline remains a highly viable area for future research to bootstrap the value head and sharpen endgame play.
- *Kinematic bootstrapping:* Boot strap the policy head with uniform distributions of valid moves, before playing for value.

// ========================================== // CHAPTER 6: Ethics // ==========================================
= Ethics



== Using Typst

// Table Example
#figure(
  table(
    columns: (1fr, auto),
    align: (left, center),
    // Logic: Vertical line after column 0 (index 0).
    // Horizontal lines are handled by table.hline, which overrides this stroke.
    stroke: (x, y) => if x == 1 { (left: 0.5pt) } else { none },

    table.hline(),
    table.header([Operation], [Speed]),
    table.hline(),
    [Add, Mul, Mul-Add], [8],
    [Reciprocal], [2],
    [Divide], [0.88],
    [Divide Intrinsic], [1.6],
    table.hline(stroke: 0.5pt),
    [Recip. Square Root], [2],
    [Square Root], [1],
    table.hline(stroke: 0.5pt),
    [Logarithm], [2],
    [Exponent], [1],
    table.hline(stroke: 0.5pt),
    [Sin, Cos Intrinsics], [2],
    [Sin, Cos, Tan], [Slow],
    table.hline(),
  ),
  caption: [An example table, showing speed in operations per cycle per multiprocessor],
)

=== Adding figures

Figures are added using the `#figure` function. Typst automatically handles the numbering and placement of your images,
as shown in the example that produces Figure 1.1. Common formats like PNG, JPEG, and SVG are supported natively.

=== Adding tables

Tables are defined directly in the source code using the `#table` function. This provides a highly flexible way to grid
your data. An example of a styled table is given in Table 1.1.

=== Adding equations

A primary advantage of Typst is its intuitive mathematical notation. Equations can be written within `$ ... $`
delimiters. It handles numbered equations easily, as in the recursive formula:

// Using op("Pr") for upright Pr
$
  alpha(iota, x_2) = sum_(x_1, d_(iota-1)) alpha(iota - 1, x_1) op("Pr") { bold(r)_(n(iota-1)+x_1, n iota + x_2),
    bold(t)_(iota-1) }
$

Inline math is also supported, for example to specify $1 <= iota < N$. Typst's syntax remains readable even for complex
multiline expressions:
testing testings

// Figure Example with subfigures
#figure(
  grid(
    columns: 1fr,
    row-gutter: 1em,
    align(center)[
      // Placeholder for diagram
      #rect(width: 60%, height: 3cm, stroke: 1pt)[*Diagrammatic representation*]
      \ (a) Diagrammatic representation
    ],
    align(center)[
      // Placeholder for photo
      #rect(width: 40%, height: 5cm, stroke: 1pt)[*Photo of Cluster*]
      \ (b) Photo of the Tempest cluster
    ]
  ),
  caption: [An example figure, with two parts],
)

// Multiline equation with large brackets
$
  op("Pr") { bold(r)_(0, n iota + x_2), sigma_(n iota) = x_2 } = sum_(x_1, d_(iota-1)) lr(
    [
      op("Pr") { bold(r)_(0, n(iota-1)+x_1), sigma_(n(iota-1)) = x_1 } \
      times op("Pr") { bold(r)_(n(iota-1)+x_1, n iota + x_2), bold(t)_(iota-1) }
    ], size: #200%
  )
$

// ========================================== // BIBLIOGRAPHY // ==========================================
#bibliography("refs.bib", style: "harvard-cite-them-right")
