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
  - *Hypothesis:* Unmasked training will incur an initial sample-efficiency penalty by introducing the dual-optimisation problem of learning game mechanics alongside strategy, but may lead to more robust internal representations reflecting potentially stronger play long term.

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
The requirement for an unmasked network to learn the rules of chess from scratch introduces complex learning dynamics. #cite(label("power2022grokking")) identified the phenomenon of "grokking," where neural networks trained on small algorithmic datasets exhibit delayed generalisation long after overfitting the training data. The phenomenon arose from the result of weight decay pressure, showcasing the importance of weight regularisation. Chess is an algorithmic dataset embedded within a strategic MDP, showcasing the potential for analogous phase transitions.

#cite(label("10.1145/1553374.1553380")) formalized curriculum learning, finding that machine learning models converge more effectively if training data is organized in a sequence of increasing complexity. The authors demonstrate this strategy effectively functions as a regularisation method, guiding the model to superior local minima often inaccessible through data shuffling. Theoretically, a pseudo-legal environment serves as an implicit curriculum, where an agent first receives rewards through piece kinematics and king capture before developing legal survival heuristics.

#cite(label("DBLP:journals/corr/SharmaSRR17")) introduced a framework that exploits the compositional nature of discrete action spaces by factoring them into independent basis-like components. This enables cross-action learning, allowing the agent to generalize actions that share common factors.

== Summary and Research Gap
While the efficacy of action masking is well-documented in general RL #cite(label("DBLP:journals/corr/abs-2006-14171")), and Transformers have proven capable of modeling chess @jenner2024evidencelearnedlookaheadchessplaying, the intersection of these domains remains unexplored. Existing literature does not isolate the impact of environmental guardrails on the representational quality of Transformer-based MCTS agents.

In the context of unmasked, pseudo-legal chess training, the network faces a dual-optimisation problem: it must learn the physical constraints of the board (the rules) and the strategic evaluation of states (the game). It is hypothesized that the network will exhibit a grokking-like phase transition: an initial period of high illegal-move proposals (random walk), followed by a sharp convergence toward $A_L(s)$ as physical constraints are internalized, preceding the development of high-level strategy.

This project addresses this gap by training four distinct configurations across two orthogonal axes: Action Space Constraints (Legal vs. Pseudo-Legal) and Logit Masking (Masked vs. Unmasked). By utilizing a factored two-pass autoregressive encoder, this research provides a novel, quantitative analysis of how rule internalization and action space pruning manifest within the attention layers of a chess-playing agent.

// ========================================== // CHAPTER 3: DESIGN // ===========================================
= System Design and Methodology
== System Architecture Overview
The system is designed as a closed-loop reinforcement learning pipeline, decoupling the environment transition dynamics from the neural network's internal representations. The architecture consists of three primary components: a custom deterministic environment simulator supporting dual rulesets, a two-pass autoregressive Transformer encoder, and a bipartite Monte Carlo Tree Search (MCTS) algorithm. The system operates via self-play, generating trajectories that are aggregated into a global replay buffer to optimize the network via gradient descent.

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
+ *Policy Head*: A linear projection of the 64 spatial tokens to $RR^64$, representing unnormalized logits for square seletion.
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
The system generates data via highly parallelized self-play. Trajectories are stored in a First-In-First-Out (FIFO) replay buffer with a capacity of 524,288 samples. To prevent infinite games, rollouts are forcibly terminated as draws after 400 plies, or if the root value estimation for a draw exceeds 0.95, scaled to 0.75 in the endgame.

== Optimisation and Loss Function
The network is optimized using the AdamW optimizer with $beta_1 = 0.9$, $beta_2 = 0.99$ and weight decay = $10^(-4)$. Noam learning rate scheduler is used with a factor of 0.1 and 4000 warmup steps.

For a given state $s$, let $pi$ be the MCTS visit count distribution, $p$ be the network's policy prediction, $z$ be the simulated game outcome, and $v$ be the network's value prediction. The loss function is defined as:
$L = (1 + lambda) (- limits(sum)_a pi(a|s) log p(a|s)) plus (1 - lambda) (- limits(sum)_i z_i log v_i)$

This acts as a strict mathematical curriculum. When the network proposes many illegal moves $lambda approx 1$, the policy loss approaches $2$ and the value loss approaches $0$. The network is forced to learn piece kinematics before state evaluation. As the model internalizes piece kinematics, the weights balance to $1:1$. This approach to curriculum learning is unique to the unmasked configuration, as $lambda$ is dependent on illegal move weights ($lambda$ is always zero in masked configurations).

//TODO talk about td value bootstrapping

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
This chapter details the software architecture and engineering paradigms utilized to construct the self-play reinforcement learning system. The implementation bridges the theoretical methodology with a highly optimized, parallelized execution environment. The system comprises a custom bitboard-based chess engine, a bipartite Monte Carlo Tree Search (MCTS) utilizing flat arena allocators, and a shared-weight Transformer encoder implemented via the Burn deep learning framework.

== Software Stack and Overview
The system is implemented entirely in Rust, leveraging the language's strict aliasing rules and ownership model to guarantee data-race-free parallelization. Neural network operations are executed via the Burn framework, which provides a backend-agnostic tensor computation graph (WGPU for portable execution, CUDA for hardware acceleration).

The self-play pipeline (@self-play) is designed for maximum throughput via lock-free concurrency. At the initialization of each training iteration, $B$ parallel game instances are spawned. Because each MCTS instance encapsulates its own memory arenas, the Rayon library's `par_iter_mut()` is utilized to distribute tree traversals, leaf expansions, and backpropagations across all available CPU cores without mutex contention. Synchronization only occurs at the neural network interface, where leaf nodes from all $B$ trees are aggregated into a single contiguous batch for GPU inference.

#figure(
  kind: "algorithm",
  supplement: [Algorithm],
  pseudocode-list(booktabs: true, title: smallcaps("Reinforcement Learning Loop"))[
    + *Require:* Model weights $theta$, Replay Buffer $D$ (capacity $2^19$), Batch size $B$
    + Initialize $B$ independent MCTS instances {$T_1, dots, T_B$}
    + *while* training *do*
      + *for* step = 1 *to* steps_per_iteration *do*
        + *for* i = 1 *to* simulations_per_move *do*
          + *parallel for* each tree $T$ in {$T_1, dots, T_B$} *do*
            + Traverse $T$ to find leaf node $n$
          + *end*
          + Synchronize threads
          + Batch inputs from all leaves and execute forward pass($theta$)
          + *parallel for* each tree $T$ *do*
            + Expand leaf $n$ using network outputs
            + Backpropagate value estimates to root
          + *end*
        + *end*
        + *parallel for* each tree $T$ *do*
          + Select action $a tilde pi(a|s)$ based on visit counts
          + Store transition ($s, pi, z$) in $D$
          + Advance environment state
          + *if* terminal state reached *then* reset $T$
        + *end*
      + *end*
      + *if* $|D| >$ min_samples *then*
        + *for* epoch = 1 *to* gradient_steps *do*
          + Sample mini-batch from $D$
          + Compute composite loss $L(theta)$
          + Update $theta$ via AdamW
        + *end*
      + *end*
    + *end*
  ],
) <self-play>

== Chess Engine
The environment is a custom chess engine optimized specifically for MCTS rollouts. State representation is strictly packed, and move generation relies on compile-time precomputed masks to eliminate branching and minimize CPU cache misses.

=== Bitboards and Move Generation
The board state is encoded within the `ChessBoard` struct using a `[[Bitboard; 6]; 2]` array, representing the 6 piece types across 2 colors, alongside 3 aggregate occupancy bitboards (White, Black, All). This yields a core memory footprint of 120 bytes, fitting comfortably within two standard 64-byte CPU cache lines. Hardware intrinsics, specifically `trailing_zeros` and `leading_zeros`, are utilized for $O(1)$ piece iteration and bit isolation.

To accelerate move generation, the engine employs a two-step deferred verification architecture:
+ *Pseudo-Legal Generation:* Moves are generated branchlessly using lookup tables initialized via `const fn`. Sliding piece attacks are resolved using directional masks (`ROOK_ATTACKS`, `BISHOP_ATTACKS`) and a precomputed `BETWEEN` mask table. The engine isolates the first blocking piece using Least Significant Bit (LSB) or Most Significant Bit (MSB) isolation, applying the `BETWEEN` mask to enumerate valid target squares. Moves are written to a stack-allocated `ArrayVec<ChessMove, 128>`, safely bounding the empirical maximum branching factor of chess and entirely eliminating heap allocation overhead.

+ *Strict Legality Verification:* Rather than implementing computationally expensive pin-detection logic during generation, legality is verified post-hoc. The `is_legal()` method clones the board state, applies the pseudo-legal move, and evaluates if the resulting state leaves the active King under attack via `is_square_attacked()`. This deferred approach significantly reduces branching complexity while maintaining high throughput.

=== Zobrist hashing:
State repetition detection is implemented via Zobrist hashing. A static table of 64-bit pseudo-random keys (`ZobristKeys`) is lazily initialized using Rust's `OnceLock`. The hash is computed in $O(P)$ time, where $P$ is the number of active pieces, by XORing the keys corresponding to the current board state, castling rights, en-passant file, and side-to-move. While incremental $O(1)$ hashing is theoretically optimal for sequential play, the $O(P)$ approach was selected to eliminate complex state-tracking across independent MCTS nodes. Profiling indicates this computational cost is negligible relative to network inference.

== Bipartite Monte Carlo Tree Search
Standard MCTS implementations represent a complete action as a single edge. To map the search topology to the network's two-pass autoregressive policy, the MCTS is structured as a bipartite graph alternating between `PieceSelect` and `PieceMove` nodes.

=== Arena Allocators <arenas>
To avoid the memory fragmentation and pointer-chasing overhead associated with recursive tree structures, the MCTS utilizes flat, index-based arena allocators:
- `node_arena: Vec<MctsNode>`
- `edge_arena: Vec<MctsEdge>`
- `position_arena: Vec<ChessPosition>`

Nodes reference their children and associated board states via `usize` indices. A dedicated `position_arena` is implemented because consecutive `PieceSelect` and `PieceMove` nodes share identical underlying board states (differing only in the `selected_sq` parameter). This deduplication provides a crucial memory optimization.

*Architectural Limitation:* The index-based arena design exhibits suboptimal memory scaling during deep rollouts. Because indices must remain stable throughout the lifetime of a game, fully explored or abandoned subtrees cannot be easily garbage-collected or culled without invalidating the index space. Consequently, RAM consumption scales disproportionately compared to VRAM during large batch operations, establishing system memory as the primary bottleneck for batch size scaling. Garbage collection methods were considered, but rejected to stay within scope of the project.

=== Node Expansion, Promotions and Terminal Handling
Leaf expansion (@leaf_expansion) transitions the state machine. Expanding a `PieceSelect` edge generates a `PieceMove` node without altering the underlying board state. Expanding a `PieceMove` edge applies the move, computes the new Zobrist hash, evaluates terminal states (including threefold repetition via path history), and generates a new `PieceSelect` node.

Promotions are handled natively by the bipartite structure, bypassing the need for a third output dimension in the neural network. During the expansion of a pawn moving to the 8th rank, the MCTS intercepts the spatial prior $p$ and distributes it uniformly ($p/4$) across four discrete edges (Queen, Rook, Bishop, Knight). This enables the model to handle underpromotions through search topology rather than increased output dimensionality.

#figure(
  kind: "algorithm",
  supplement: [Algorithm],
  pseudocode-list(booktabs: true, title: smallcaps[Bipartite Leaf Expansion])[
    + *Require:* Leaf edge $e$, Network prior probabilities $P$, Search path history $H$
    + Let $N_"parent"$ be the parent node of $e$
    + *if* $N_"parent"$ is of type PieceSelect *then*
      + Create $N_"child"$ of type PieceMove
      + Set $N_"child"$.selected_square = $e$.target_square
      + Link $e$ to $N_"child"$
    + *else if* $N_"parent"$ is of type PieceMove *then*
      + $s' <-$ Apply move($N_"parent"$.selected_square, $e$.target_square)
      + *if* Zobrist($s'$) appears $≥ 2$ times in $H$ *then*
        + Create $N_"child"$ as Terminal(Draw)
      + *else if* is_checkmate($s'$) *then*
        + Create $N_"child"$ as Terminal(Win/Loss)
      + *else*
        + Create $N_"child"$ of type PieceSelect representing $s'$
      + *end*
      + Link $e$ to $N_"child"$
    + *end*
    + *if* $N_"child"$ is not Terminal *then*
      + *for each* legal destination square $i$ *do*
        + *if* move is pawn promotion *then*
          + Create 4 edges (Q, R, B, N) with prior = $P[i] / 4$
        + *else*
          + Create 1 edge with prior = $P[i]$
        + *end*
        + Attach edge(s) to $N_"child"$
      + *end*
    + *end*
  ],
) <leaf_expansion>

=== Edge Selection and Value Propagation
During traversal, edge selection utilizes a PUCT variant that penalizes draws to encourage decisive variations. For a node with visit count $N$, an edge $e$ with confidence $p_e$, visits $n_e$, and mean value $(W_e, D_e, L_e)$ obtains a score edge_selection

- $Q_e = W_e - L_e - D_e$
- $U_e = p_e dot c_"puct" dot (sqrt(N) + 10^(-8)) / (1 + n_e)$

The selected edge maximizes $Q_e + U_e$. Upon leaf evaluation, the value vector $[W, D, L]$ is backpropagated. If the side-to-move alternates along the path, the vector is inverted ($[W, D, L] -> [L, D, W]$) value_prop.

The path is cleared after each simulation, ready for the next traversal.

=== Mask Integration and Prior Renormalisation
In unmasked configurations, the network outputs a probability distribution over all 64 squares, including illegal moves. To maintain a valid probability distribution for PUCT, the probability mass assigned to illegal squares ($lambda$) is calculated, and the legal priors are renormalized:
$ p'_i = frac(p_i, 1 - lambda) quad forall i in "legal moves" $
In masked configurations, illegal logits are masked to $-10^9$ prior to the softmax operation, rendering renormalization unnecessary.

The illegal probability mass $lambda$ also plays a central role in the self-annealing loss described in @self-annealing-loss.

== Neural Network Architecture
The `ChessTransformer` is a shared-weight encoder mapping a spatial tensor and a metadata vector to a policy distribution and a value estimate.

=== Tensor Formulation and Canonicalisation:
To enforce symmetric learning, all board states are canonicalized to the perspective of the side-to-move via `flip_board()`. The input comprises:
- *Spatial Tensor ($X in RR^(64 times 14)$):* 12 piece planes, 1 en-passant plane, and 1 selected-square plane. The selected-square plane is populated exclusively during the `PieceMove` pass.
- *Metadata Tensor ($M in RR^5$):* 4 binary castling rights and 1 continuous scalar representing the half-move clock.

Tensors are constructed by extending flat `Vec<f32>` slices and reshaping them via Burn's `TensorData` API, ensuring contiguous memory layouts before transfer to the GPU.

=== Transformer Encoder and Dynamic Masking
The architecture utilizes $d_"model" = 512$, $n_"layers" = 8$, and $n_"heads" = 8$. The 64 spatial vectors are linearly projected to $d_"model"$, and 2D learnable positional embeddings are broadcast across the sequence. The metadata vector $M$ is projected and concatenated as a pseudo-[CLS] token, resulting in a $65 times d_"model"$ tensor.

Following the transformer layers, the sequence bifurcates:
- Tokens $0..63$ are passed through a linear policy head to produce 64 spatial logits.
- Token $64$ is passed through a linear value head to produce 3 logits (Win, Draw, Loss).

The model's `forward_classification` method dynamically applies action masking based on the experimental configuration. In masked configurations, a boolean tensor identifies illegal squares, and `mask_fill` is applied to set corresponding logits to $-1 times 10^9$ prior to softmax activation. In unmasked configurations, this step is bypassed, forcing the network to output raw logits over all squares.

== Training Pipeline
The training pipeline orchestrates data generation, buffer management, and gradient updates, specifically engineered to support the study of rule internalisation.

=== Batch Expansion and Synchronisation
The interface between the CPU-bound MCTS and the GPU-bound neural network is implemented in `expand_batch` (@expand_batch). During the self-play loop, MCTS traversals execute asynchronously across all threads until a leaf node is reached. The threads then synchronize, aggregating the canonicalized input tensors and legal move masks into a single batch. Following a unified forward pass, the resulting `NetworkLabels` (policy and value arrays) are scattered back to their respective MCTS instances for edge expansion and value backpropagation.

#figure(
  kind: "algorithm",
  supplement: [Algorithm],
  pseudocode-list(booktabs: true, title: smallcaps("Batched Inference and Prior Renormalisation"))[
    - *Require:* Set of leaf nodes ${n_1, ..., n_B}$, Configuration $C$
    + $X_"batch", M_"batch" <-$ Extract spatial and meta tensors from ${n_1, ..., n_B}$
    + *if* $C."masked"$ is True *then*
      + $M_"batch" <-$ Generate legal move boolean masks
    + *else*
      + $M_"batch" <-$ None
    + *end*
    + $P_"raw", V_"raw" <-$ TransformerForward($X_"batch", M_"batch", M_"ask_batch"$)
    + *for* $b = 1$ *to* $B$ *do*
      + $lambda <- 0$
      + *if* $C."masked"$ is False *then*
        + *for each* illegal square $i$ in $n_b$ *do*
          + $lambda <- lambda + P_"raw"[b][i]$
        + *end*
        + *for each* legal square $j$ in $n_b$ *do*
          + $P_"raw"[b][j] <- P_"raw"[b][j] / (1 - lambda)$ // Renormalize
        + *end*
      + *end*
      + $n_b."illegal_mass" <- lambda$
      + $n_b."value" <- V_"raw"[b]$
      + Populate $n_b$ edges using $P_"raw"[b]$
    + *end*
  ],
) <expand_batch>

=== Self-Annealing Loss Function <self-annealing-loss>
To facilitate learning in the unmasked configuration, a self-annealing loss function dynamically shifts the optimisation objective from kinematic legality to strategic evaluation. During batch expansion, the system calculates $lambda$, representing the mean probability mass assigned to illegal moves across the batch.

Within the `calculate_loss` function, the composite loss is defined as:
$ L = L_"policy"(1 + lambda) + L_"value"(1 - lambda) $

Where $L_"policy"$ is the cross-entropy of the policy output against MCTS visit counts, and $L_"value"$ is the cross-entropy of the value output against the terminal game outcome. When the network proposes a high volume of illegal moves ($lambda approx 1$), the value gradient is heavily suppressed. This acts as an automated, mathematically rigorous curriculum: the network is forced to optimize for piece kinematics ($L_"policy" approx 2$) before strategic optimisation ($L_"value"$) can commence. As the model internalizes the rules ($lambda -> 0$), the weights naturally balance to a $1:1$ ratio.

== Performance and Correctness
Engine throughput is dictated by the interplay between CPU-bound tree search and GPU-bound tensor operations. Profiling indicates that the dominant computational cost is the batched forward pass of the Transformer.

The strict memory isolation of the MCTS arenas allows CPU scaling to remain perfectly linear until the GPU becomes saturated by the aggregated batch size. However, as noted in @arenas, the inability to cull unreferenced `ChessPosition` structs from the flat arenas results in high memory pressure. For a batch size of 256, concurrent games can consume upwards of 60 GB of system RAM during deep endgames, establishing memory capacity as the primary bottleneck for scaling the number of parallel self-play environments. Despite this, the architecture successfully sustains the high-throughput data generation required to train the four distinct agent configurations under identical hyperparameters.

Move generation is unit tested against multiple positions. Zobrist hashing uses 64 bit keys, providing cryptographic-level collision resistance for the search depth of the training configuration. Tensor shapes are checked at compile-time.

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
