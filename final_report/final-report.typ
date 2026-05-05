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
    )[The Impact of Action Masking and Environmental Constraints on Rule Internalization in Transformer-Based Chess Agents]

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

    #align(left)[Supervised by: Nishanth Sastry]
  ]
]

// ========================================== // PAGE 2: DECLARATION // ==========================================
#page(numbering: none)[
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

  [$Q(s, a)$], [The action-value function representing the mean expected reward for taking action $a$ in state $s$.],
  [$pi(a|s)$], [The policy distribution; the probability of selecting action $a$ given the current state $s$.],
  [$V(s)$], [The value function representing the scalar evaluation of a board position's win probability.],
  [$lambda$], [The curriculum coefficient (illegal mass ratio) used to weigh policy vs. value loss.],
  [$N(s, a)$], [The visit count of an edge in the search tree, indicating how many times an action has been explored.],
  [$c_("puct")$], [A hyperparameter controlling the trade-off between exploration and exploitation in the search tree.],
  [$A_L(s)$], [The set of strictly legal moves available in state $s$ according to FIDE rules.],
  [$A_P(s)$], [The set of pseudo-legal moves (geometrically valid but potentially leaving the king in check).],
  [$N_"select"$],
  [A node in the bipartite MCTS representing a board state where the agent must select a 'from' square.],

  [$N_"move"$],
  [A node in the bipartite MCTS where an origin square has been fixed and the agent selects a 'to' square.],

  [$K_"cap"$], [The terminal event of king capture used as the reward signal in the pseudo-legal ruleset.],
  [$d_("model")$], [The hidden dimensionality of the Transformer encoder's internal representations.],
)

// ========================================== // PAGE 10: ABBREVIATIONS // ==========================================
#front-heading("Abbreviations")

#grid(
  columns: (0.2fr, 1fr),
  row-gutter: 1.5em,

  [MCTS], [Monte Carlo Tree Search],
  [PUCT], [Predictor + Upper Confidence Bound applied to Trees],
  [RL], [Reinforcement Learning],
  [MDP], [Markov Decision Process],
  [FIDE], [Fédération Internationale des Échecs (International Chess Federation)],
  [CNN], [Convolutional Neural Network],
  [TD], [Temporal Difference (Learning)],
  [FIFO], [First-In, First-Out (Replay Buffer)],
  [SOTA], [State Of The Art],
  [MSB / LSB], [Most Significant Bit / Least Significant Bit],
  [W/D/L], [Win / Draw / Loss],
  [FEN], [Forsyth-Edwards Notation],
)

// ========================================== // MAIN CONTENT START // ==========================================
#set heading(numbering: "1.1")

// ==========================================
// CHAPTER 1: INTRODUCTION
// ==========================================
= Introduction
== Project Background
Modern machine learning frequently decouples models from environment transition dynamics via *Logit Masking*, a heuristic that artificially zeroes out the probability of illegal actions prior to softmax activation. While computationally efficient, this deprives the network of negative feedback for invalid predictions, bypassing the model's need to internally represent the mechanics of play. Furthermore, standard reinforcement learning (RL) agents are prevented from making invalid state transitions by hardcoded environmental constraints.

This raises a question that the literature has largely sidestepped: when these guardrails are removed, what learning dynamics emerge in their absence, and how are they reflected in the network's internal representations? Chess provides an optimal substrate for this investigation: it presents highly ambiguous intermediate states bound by objective ground truths and a mathematically perfect, yet computationally intractable, solution space.

== Project Description
Existing literature rarely assesses the impact of these environmental guardrails on the learning dynamics of transformer-based agents. This project investigates two orthogonal axes of environmental constraint: Action Space Constraints and Logit Masking.

To facilitate explicit measurement of model interpretability, a custom two-pass autoregressive transformer encoder and a bipartite Monte Carlo Tree Search (MCTS) are introduced. This architecture decouples piece selection from destination selection, providing a granular window into the model's spatial reasoning.

The two experimental axes give rise to four configurations:
+ Along the first axis, the control uses a strict legal ruleset, while the test condition adopts a pseudo-legal ruleset whose terminal state is the dense king-capture event $K_"cap"$ rather than the sparse, recursive checkmate. This shift alters the MDP reward function $R(s,a)$, yielding a potentially smoother reward landscape. The hypothesis is that the network will first bootstrap the rules and goals of the environment; ideal play in the pseudo-legal space $A_P$ may converge to standard chess, with concepts such as pins and checkmates emerging as survival heuristics to prevent or secure king capture.  Conversely, removing environmental guardrails may degrade MCTS value estimation and prevent policy bootstrapping.
+ Along the second axis, the control applies logit masking (illegal moves are eliminated before selection), while the test configuration leaves logits unmasked, so the network is permitted to propose illegal moves and is penalised through the loss function.  The hypothesis is that unmasked training will incur an initial sample-efficiency penalty by forcing a dual-optimisation problem: learning game mechanics alongside strategy, but may ultimately yield more robust internal representations and stronger asymptotic play.

=== Scope and Limitations
Three limitations bound the scope of the work. First, playing strength is not benchmarked against external engines; the comparison of interest is across the four configurations. Absolute Elo against SOTA would require orders of magnitude more compute than the project budget permits. Second, the value head is bootstrapped against MCTS root values rather than terminal outcomes, constrained by single gpu throughput. Third, the four configurations are trained to a fixed wall-clock budget rather than to convergence, investigating learning trajectories over asymptotic performance.

== Aims and Objectives
+ The primary aim is to train multiple transformer‑based agents via self‑play reinforcement learning across the varying rule sets and action spaces and to quantitatively evaluate their performance, learning trajectories, and emergent behaviours.  Concrete objectives include implementing a highly optimised, dual‑ruleset chess library for high‑throughput RL self‑play; architecting a two‑pass autoregressive transformer and bipartite MCTS that models piece‑then‑square action selection; training the four configurations ($"Legal" + "Masked", "Legal" + "Unmasked", "Pseudo-Legal" + "Masked", "Pseudo-Legal" + "Unmasked"$) from scratch under identical hyperparameters; quantifying convergence rates, illegal‑move proposal frequencies, and the emergence of rule-abiding play across the unconstrained setups; and correlating the emergence of legal play with attention distributions in the two‑pass encoder, thereby opening a window onto spatial reasoning.

== Thesis Structure
The remainder of this dissertation is structured as follows. *Chapter 2* surveys the literature surrounding reinforcement learning, Transformer architectures, action masking, and curriculum learning, identifying the gap this work addresses. *Chapter 3* details the system design, including the bipartite MCTS, the two-pass encoder, and the self-annealing loss function. *Chapter 4* outlines the implementation of the Rust engine and training pipeline, with particular attention to the engineering decisions that make high-throughput self-play tractable. *Chapter 5* presents the results and evaluation across the four configurations. *Chapter 6* concludes and discusses avenues for future work.

// ==========================================
// CHAPTER 2: LITERATURE REVIEW
// ==========================================
= Literature Review
== Chess as an MDP
Chess is a deterministic, zero-sum, perfect-information game, formally modelled as a Markov Decision Process defined by the tuple $(S, A, T, R)$, where $S$ is the state space ($|S| approx 10^40$), $A$ is the action space, $T: S times A -> S$ is the deterministic transition function, and $R: S times A -> {-1, 0, 1}$ is the reward function @Shannon01031950. Standard implementations enforce a legal action space $A_L(s) subset A$ in which moves leaving the king in check are excluded by the environment @fide2023laws. This project introduces a relaxed pseudo-legal action space $A_P(s)$ such that $A_L(s) subset A_P(s) subset A$, permitting all geometrically valid piece movements and shifting the terminal condition from the abstract _checkmate_ to the explicit _king capture_ $K_("cap")$.

Typical chess agents emit a single joint policy distribution over the $approx 4672$ possible piece movement on all 64 squares. Factoring the joint probability as $ P(a|s) = P("from"|s) dot P("to"|s, "from") $ collapses the output dimensionality from $d_("model") times 4672$ to $d_("model") times 64$ and gives rise to two distinct, observable failure modes.

For any state $s$, an optimal policy $pi^*$ trained in $A_P$ must implicitly learn to prune the action space such that:
$ pi^*(a|s) -> 0 quad "for all" a in (A_P(s) without A_L(s)) $
Under this paradigm, concepts such as pins and checkmating nets cease to be hardcoded environmental properties and become _emergent survival heuristics_ within the value landscape.

== Chess Played by Machines
=== AlphaZero and MuZero
The paradigm of self-play RL in chess was defined by AlphaZero @silver2017masteringchessshogiselfplay, which combined Monte Carlo Tree Search (MCTS) with deep convolutional neural networks (CNNs) for policy and value evaluation. However, AlphaZero relies entirely on a hardcoded environment to generate $A_L(s)$, bypassing the need for the network to internalize the rules of the game.

MuZero @Schrittwieser_2020 removed the need for a known transition dynamics model by learning a latent environment simulator. While MuZero demonstrates the ability to adhere to environment constraints implicitly within its hidden state transitions, its policy head still simulates within the legal action space during MCTS rollouts. The literature lacks insight into how the removal of environmental guardrails or action masking impacts the sample efficiency and representational robustness of the underlying network.

=== Transformers in Chess
Recent advancements demonstrate the efficacy of Transformer architectures in chess, shifting away from the spatial inductive biases of CNNs. #cite(label("monroe2024masteringchesstransformermodel")) achieved Grandmaster-level performance with a Transformer relying purely on attention to evaluate board states, without explicit search. #cite(label("jenner2024evidencelearnedlookaheadchessplaying")) provided complementary evidence of learned look-ahead in chess-playing networks, showing that attention heads encode candidate future board states. @McGrath_2022 further demonstrated that AlphaZero-style networks acquire human-interpretable concepts (material balance, mobility, king safety) over the course of training.

These results frame the central architectural choice of the present work. Decoupling $P("from"|s)$ from $P("to"|s, "from")$ forces attention to explicitly represent geometric constraints in two separable phases, providing an observable window into distinct faliure modes: attention faliure (identifying moveable pieces) and kinematic faliure (how pieces move).

== Invalid Action Masking
In policy gradient algorithms, it is standard practice to prevent the selection of illegal actions via Invalid Action Masking (Logit Masking). Masking applies a transformation to the policy logits $z$ prior to the softmax activation:

$ P(a_i|s) = exp(z_i + M_i) / (sum_j exp(z_j + M_j)) $
where $M_i = -infinity$ if $a_i in.not A_L(s)$, and $0$ otherwise.

#cite(label("DBLP:journals/corr/abs-2006-14171")) demonstrated that masking significantly improves sample efficiency by collapsing the exploration space. However, logit masking, by design, prevents the network receiving feedback from masked indices due to gradients being nullified in the compute graph preventing backpropogation of loss. For an MCTS-based agent, masking is essentially required as simulations into invalid state transitions corrupt value estimates and propagate poison up the tree. However, Huang and Ontañón's analysis is restricted to small, flat action spaces where the legal subset is a fixed function of a few features (MicroRTS). Chess presents a qualitatively harder case: legality depends on the global board configuration, and the function $A_L(s)$ is itself non-trivial to compute. Whether the conclusions of #cite(label("DBLP:journals/corr/abs-2006-14171")) generalise to a setting where the rules themselves constitute an algorithmic learning target aims to be answered.

== Learning Dynamics
The unmasked, pseudo-legal regime forces the network to learn the rules of chess from scratch as a precondition for strategic evaluation, raising the prospect of non-monotonic learning dynamics. #cite(label("power2022grokking")) identified the phenomenon of _grokking_, in which neural networks trained on small algorithmic datasets exhibit sharply delayed generalisation long after overfitting the training set, mediated by weight-decay pressure. Chess is an algorithmic dataset embedded in a strategic MDP, showcasing the potential of analogous phase transitions.

#cite(label("10.1145/1553374.1553380")) formalized _curriculum learning_, showing that ordering training data from simple to complex acts as a regulariser, guiding optimisation toward superior local minima inaccessible through random shuffling. It is hypothesized a pseudo-legal environment serves implicitly as curriculum where an agent first receives rewards through piece kinematics and king capture before survival heuristics under $A_L$.

#cite(label("DBLP:journals/corr/SharmaSRR17")) introduced a framework for factoring discrete action spaces into compositional, basis-like components, enabling cross-action generalisation among actions that share common factors. This princinple is instantiated in the factored engine architecture, where each selection and destination shares a common output head.

== Summary and Research Gap
While the efficacy of action masking is well-documented in general RL (#cite(label("DBLP:journals/corr/abs-2006-14171"))), and Transformers have proven capable of modelling chess at a high level (@jenner2024evidencelearnedlookaheadchessplaying), the intersection of these domains is unexplored. No existing work isolates the impact of environmental guardrails on the representational quality of Transformer-based MCTS agents, nor on the dynamics of rule internalisation when those guardrails are removed.

In the unmasked, pseudo-legal regime, the network faces a dual-optimisation problem: it must learn the physical constraints of the board (the rules) and the strategic evaluation of states (the game). It is hypothesised that this configuration will exhibit a grokking-like phase transition—an initial random-walk regime of high illegal-move proposal, a sharp convergence toward $A_P(s)$ as kinematics are internalised, and only then the development of strategic play in $A_L(s)$. // innaccurate, not grokking, just curriculum

This project addresses the gap by training the four configurations from scratch under identical hyperparameters, instrumenting them with a factored two-pass encoder, and providing a quantitative analysis of how rule internalisation and action-space pruning manifest within the attention layers of a chess-playing agent.

// ==========================================
// CHAPTER 3: DESIGN
// ==========================================
= System Design and Methodology
== System Architecture Overview <overview>
The system is designed as a closed-loop reinforcement learning pipeline, decoupling the environment transition dynamics from the neural network's internal representations. The architecture consists of three primary components: a custom deterministic environment simulator supporting dual rulesets, a two-pass autoregressive Transformer encoder, and a bipartite Monte Carlo Tree Search (MCTS) algorithm. The system operates via self-play, generating trajectories that are aggregated into a global FIFO replay buffer to optimize the network via gradient descent.

Each component is designed so that the experimental axes—rule set and masking—can be toggled independently without altering any other part of the system. This is essential to the validity of the comparison: the four configurations differ only in the two boolean flags governing their respective axes, with all hyperparameters held constant.

== Functional and Non-Functional Requirements <requirements>
The methodology imposes four hard requirements on the implementation, each of which constrains the design space in @env-state–@design-eval.

+ *Configuration parity:* The four cells of the experimental matrix must differ only in the two boolean flags governing rule set and masking. No code path may exist that is reachable in one configuration but not another, with the sole exception of the masking step itself. This rules out per-configuration training scripts and motivates the unified `forward_classification` implementation in @encoder.

+ *Self-play throughput:* Four full training runs under a single-GPU budget require sustained throughput of order $10^4$ position evaluations per second. This rules out reference-counted tree structures (cache-hostile) and dynamic move-list allocation (heap pressure), motivating arena allocation (@arenas) and stack-allocated move lists (@move_gen).

+ *Reproducibility:* Self-play is intrinsically stochastic, but every source of randomness must be seedable. Dirichlet noise, temperature sampling, replay sampling, and weight initialisation all consume from seeded RNGs.

+ *Auditability:* Observed learning dynamics must be attributable to the network rather than to engine artefacts. This requires the move generator to be independently testable against a reference (@testing) and the masking boundary in @move_gen to be unambiguous in code.

== Environment Formulation and State Representation <env-state>
=== Dual Ruleset MDPs <rulesets>

The *Legal Environment $E_l$* adheres to standard FIDE rules. The action space ($A_l(s)$) is strictly contained to moves that do not leave the king in check. The terminal reward $R(s, a)$ is evaluated upon Checkmate, Stalemate or standard draw conditions.
The *Pseudo-Legal Environment* $E_P$ relaxes the action space to $A_P(s)$, permitting all geometrically valid piece movements regardless of king safety. The terminal condition shifts from checkmate to the explicit king-capture event $K_"cap"$.  FIDE draw conditions are retained to guarantee finite rollouts and to allow a correct transition into legal play.

=== State Canonicalisation and Tensor Formulation <canonicalisation_tensors>
To enforce symmetric learning and halve the state space, all board states are canonicalized to the perspective of the side-to-move. If it is Black's turn, the board and castling rights are geometrically flipped.
The state $s$ is mapped to the spatial tensor representation $X in RR^(64 times 14)$ and a scalar meta-tensor $M in RR^5$.
- *Spatial Tensor*: Comprises 12 piece bitboards (6 piece types $times$ 2 colors), 1 en-passant target plane, and 1 selected-square plane (utilized in the second pass of the network).
- *Meta Tensor*: Encodes the 4 castling rights (one-hot) and the half-move clock (scaled by $1/100$).

To explicitly measure spatial reasoning, the system factors the joint probability of a move $P(a|s)$ into an autoregressive sequence: $P(a|s) = P("from"|s) times P("to"|s,"from")$. The network executes two forward passes using shared weights:
+ *Pass 1 (Piece Selection)*: The network evaluates the board state with the selected square plane zeroed out, outputting a policy distribution over the 64 squares to select the origin square (_from_). The value head will evaluate the board state.
+ *Pass 2 (Destination Selection)*: The origin square is encoded into the 14th plane of the spatial tensor. The network re-evaluates the state, outputting a policy distribution over the 64 squares to select the destination (_to_). The value head will evaluate the position given a piece to move, effectively evaluating the viability of the selected piece.

_Promotions:_ The factored $64 -> 64$ output lacks a third dimension for promotion piece selection. Thus, promotions are handled in the search topology: when a pawn destination on the back rank is expanded, the corresponding edge is replaced by four sub-edges (Q, R, B, N), equally dividing the prior probability. This delegates underpromotion to MCTS, where the additional branching factor is amortised over many simulations.

=== Network Topology <network_topology>
The model uses a Transformer Encoder architecture without spatial inductive biases (e.g., convolutions). The spatial tensor $X$ is linearly projected to $d_"model"$, and $2D$ learned positional embeddings are added to retain geometric context. The meta-tensor $M$ is linearly projected and concatenated as a pseudo-[CLS] token, resulting in a sequence length of 65.

The model is scaled symmetrically: $n_"heads" = 8$, $n_"layers" = 8$, $d_"model" = 8 dot 64 = 512$, $d_"ff" = 4 dot d_"model"$.

=== Output Heads <output_heads>
The final latent representation is routed into two distinct heads:
+ *Policy head:* a linear projection of the 64 spatial tokens to $RR^64$, producing unnormalised logits over square selection.
+ *Value head:* a linear projection of the [CLS] token to $RR^3$, producing W/D/L logits. In Pass 1 this evaluates the raw state; in Pass 2 it evaluates the state conditional on the selected origin square.

== Bipartite Monte Carlo Tree Search <method-mcts>
=== Topology <mcts_topology>
To accommodate the two-pass autoregressive policy, the search tree alternates between two node types:
- *Selection Nodes ($N_"select")$:* Represent a board state. Edges represent the choice of an origin square.
- *Action Nodes ($N_"move")$:* Represent a board state plus a selected origin square. Edges represent the destination square; transitioning the environment to a new $N_"select"$ node.

A complete move thus traverses a (Selection, Action) pair. This bipartite structure is not merely a notational convenience: it physically separates the two failure modes of the factored policy in the search tree, such that visit-count statistics on Selection edges directly diagnose attention failures, while Action edges diagnose kinematic failures.

=== Selection and PUCT <puct>
During traversals, edges are selected using a modified Predictor + Upper Confidence Bound applied to Trees (PUCT) algorithm. To mitigate draw-seeking behaviour, a contempt factor is baked into the empirical action value $Q(s, a)$.
$
       Q(s, a) & = W - L - 0.05 D \
       U(s, a) & = c_("puct") dot P(s, a) dot (sqrt(N(s)) + 10^(-8)) / (1 + N(s, a)) \
  "PUCT"(s, a) & = Q(s, a) + U(s, a)
$
where $W, D, L$ are the mean W/D/L value estimates accumulated on the edge, $P(s, a)$ is the prior, and $N(s)$, $N(s, a)$ are the parent and edge visit counts respectively.

=== Exploration Mechanisms <exploration>
Exploration is injected through two mechanisms. Dirichlet noise is applied exclusively to the root prior, with $P(s, a) <- (1 - epsilon) P(s, a) + epsilon dot eta$, with $eta tilde "Dir"(alpha)$, $alpha = 0.3$, and $epsilon = 0.25$.  At the root, the move played is sampled in proportion to $N(s,a)^(1/t)$, where the temperature $t$ scales dynamically with ply count and remaining material, transitioning from exploratory in the opening to deterministic in endgames.

=== Simulation Integrity and the Masking Boundary <integrity>
During MCTS rollouts, invalid transitions corrupt value backups across all ancestor edges. To maintain simulation integrity, action masking is applied during the search phase across all configurations, ensuring the search tree only explores legal moves. The experimental distinction lies in whether action masking is applied to the network’s raw logits during loss calculation. In masked configurations, illegal logits are clamped to $−10^9$ before softmax, training the model against a distribution restricted to legal actions. In unmasked configurations, the policy loss is computed over the raw 64-square distribution; the network must internalize the penalty for assigning probability mass to illegal moves via cross-entropy. While the MCTS always sees a legal distribution, the training signal differs in whether the network is shielded from or penalized for invalid predictions.

== Reinforcement Learning Pipeline
=== Self-Play and Replay Buffer <method-self-play>
Trajectories are generated via highly parallelised self-play. Samples are stored in a FIFO replay buffer of capacity $2^(19) = 524288$. To prevent infinite games, rollouts are forcibly terminated as draws after 400 plies, or whenever the MCTS root value estimate for a draw exceeds $0.95$, relaxed to $0.75$ after 60 plies.

== Optimisation and the Self-Annealing Loss <self-annealing>
The network is optimised with AdamW ($beta_1 = 0.9$, $beta_2 = 0.99$, weight decay $10^(-4)$) under a Noam learning-rate schedule with factor $0.01$ and 4000 warm-up steps. The value head is bootstrapped against the MCTS root-value distribution rather than the eventual game outcome (TD-like) for computational feasability.

The principal methodological contribution lies in the loss function. For a sample $(s, pi, z)$ with policy prediction $p$ and value prediction $v$, the loss is:
$
  L = (1 + lambda) dot underbrace((-sum_a pi(a|s) log p(a|s)), L_("policy")) + (1 - lambda) dot underbrace((-sum_i z_i log v_i), L_("value"))
$
where $lambda in [0, 1]$ is the mean probability mass that the network has assigned to illegal moves over the most recent batch of leaf expansions.

This acts as a strict mathematical curriculum. When the network proposes many illegal moves $lambda approx 1$, the policy loss approaches $2$ and the value loss approaches $0$. The network is forced to learn piece kinematics before state evaluation. As the model internalizes piece kinematics, the weights balance to $1:1$. This approach to curriculum learning is unique to the unmasked configuration, as $lambda$ is dependent on illegal move weights ($lambda$ is always zero in masked configurations).

== Experimental Design and Evaluation Metrics <design-eval>
=== The Configuration Matrix <configuration_matrix>
The four configurations form a $2 times 2$ matrix.  *Legal + Masked* serves as the control, reproducing the standard AlphaZero paradigm.  *Legal + Unmasked* requires the network to learn piece kinematics through loss penalties while the environment continues to enforce legality.  *Pseudo-Legal + Masked* lets the network play the $K_"cap"$ variant under environmental masking, so it must discover that kings should not be left in check as a terminal-avoidance heuristic.  *Pseudo-Legal + Unmasked* forces the network to learn both piece kinematics and FIDE-style legality entirely from scratch.

=== Quantitative Metrics <metrics>
The learning dynamics of each configuration are captured continuously during training by four metrics: *average illegal probability* (the total probability mass assigned to invalid actions, a direct readout of rule internalisation), *average game length* in plies (indicating the transition from random walks to sustained strategic play), *win/draw ratios* (tracking the emergence of decisive outcomes), and *value loss* (measuring the calibration of the network’s state evaluation over time).

== Discussion of Design Challenges <design_challenges>
Three design problems were materially harder than they first appeared, and each shaped the final system in ways worth recording.

TODO

// ==========================================
// CHAPTER 4: IMPLEMENTATION
// ==========================================
= Implementation Details
This chapter details the software architecture and engineering paradigms utilized to construct the self-play reinforcement learning system. The implementation translates the methodology into a high-throughput, reproducible execution environment optimized for three criteria: clean experimental toggling, sufficient velocity to complete four training runs within the project timeframe, and system auditability to isolate learning dynamics from engine artifacts.

The architecture comprises a custom bitboard-based chess engine, a bipartite Monte Carlo Tree Search (MCTS) utilizing flat arena allocators, and a shared-weight Transformer encoder implemented via the Burn deep learning framework. All components are parallelized to bridge theoretical design with optimized execution.

== Software Stack and Overview <stack>
The system is implemented entirely in Rust, leveraging the language's strict aliasing rules and ownership model to guarantee data-race-free parallelization. Neural network operations are executed via the Burn framework, which provides a backend-agnostic tensor computation graph (WGPU for portable execution, CUDA for hardware acceleration).

The self-play pipeline (@self-play) is designed for maximum throughput via lock-free concurrency. At the initialization of each training iteration, $B$ parallel game instances are spawned. Because each MCTS instance encapsulates its own memory arenas, the Rayon library's `par_iter_mut()` is utilized to distribute tree traversals, leaf expansions, and backpropagations across all available CPU cores without mutex contention. Synchronization only occurs at the neural network interface, where leaf nodes from all $B$ trees are aggregated into a single contiguous batch for GPU inference.

#figure(
  kind: "algorithm",
  supplement: [Algorithm],
  pseudocode-list(booktabs: true, title: smallcaps("Reinforcement Learning Loop"))[
    + *Require:* Model weights $theta$, Replay Buffer $D$ (capacity $2^(19)$), Batch size $B$
    + Initialise $B$ independent MCTS instances ${T_1, dots, T_B}$
    + *while* training *do*
      + *for* step = 1 *to* steps_per_iteration *do*
        + *for* i = 1 *to* simulations_per_move *do*
          + *parallel for* each tree $T$ in ${T_1, dots, T_B}$ *do*
            + Traverse $T$ to find leaf node $n$
          + *end*
          + Synchronise threads
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
The environment is a custom chess engine optimized specifically for MCTS rollouts. State representation is strictly packed, and move generation relies on compile-time precomputed masks to minimize branching and minimize CPU cache misses.

=== Bitboards <bitboards>
The board state is encoded within the `ChessBoard` struct using a `[[Bitboard; 6]; 2]` array, representing the 6 piece types across 2 colors, alongside 3 aggregate occupancy bitboards (White, Black, All). The struct is annotated `#[repr(align(64))]` so that the entire state fits within two CPU cache lines, making move generation cache-friendly under aggressive parallel access. Hardware intrinsics, specifically `trailing_zeros` and `leading_zeros`, are used for $O(1)$ piece iteration and bit isolation.

=== Move Generation <move_gen>
The engine employs a two-step deferred verification architecture for move generation:
*Precomputed Lookup Tables:* Knight, king, pawn, rook, bishop, and rook/bishop direction masks, together with a $64 times 64$ `BETWEEN` table, are constructed via `const fn` and embedded directly in the binary as `pub const` arrays. This elides the static initialisation phase entirely and allows the compiler to fold lookups into immediate operands where it can prove the index.

*Branchless Sliding-Piece Generation:* Sliding pieces are generated by AND-ing each direction mask with the global occupancy and isolating the first blocker via LSB or MSB depending on the direction:
```rust
let to_sq = if i < 2 { blockers.pop_msb() } else { blockers.pop_lsb() };
```
The blocker square is included in the ray if it is occupied by an enemy piece (a capture), and the precomputed `BETWEEN[from][blocker]` mask supplies the squares strictly between origin and blocker, to be trivially iterated over to generate sliding moves.

*Stack-Allocated Move Lists:* Generated moves are written to a stack-allocated `ArrayVec<ChessMove, 128>`, safely bounding chess's empirical maximum branching factor while eliminating heap allocation overhead.

*Deferred Legality Verification:* Rather than implementing computationally expensive pin-detection logic during generation, legality is verified post-hoc. The `is_legal()` method clones the board state, applies the pseudo-legal move, and evaluates if the resulting state leaves the active King under attack via `is_square_attacked()`. This deferred approach significantly reduces branching complexity while maintaining high throughput.

=== Zobrist Hashing <zobrist>
State repetition detection is implemented via Zobrist hashing. A static table of 64-bit pseudo-random keys (`ZobristKeys`) is lazily initialized using Rust's `OnceLock`. The hash is computed in $O(P)$ time, where $P$ is the number of active pieces, by XORing the keys corresponding to the current board state, castling rights, en-passant file, and side-to-move. While incremental $O(1)$ hashing is theoretically optimal for sequential play, the $O(P)$ approach was selected to eliminate complex state-tracking across independent MCTS nodes. Profiling indicates this computational cost is negligible relative to network inference.

== Bipartite Monte Carlo Tree Search <bipartite_mcts>
A standard MCTS represents a complete action as a single edge between two states. To map the search topology onto the network's two-pass autoregressive policy, the search tree is implemented as a bipartite graph alternating `PieceSelect` and `PieceMove` nodes (@method-mcts).

=== Arena Allocators <arenas>
To avoid the memory fragmentation and pointer-chasing overhead associated with recursive tree structures, the MCTS utilizes flat, index-based arena allocators:
```rust
struct Arena<T> {
    buffer:   Vec<T>,
    freelist: Vec<usize>,
}
```
- `node_arena: Arena<MctsNode>`
- `edge_arena: Arena<MctsEdge>`
- `position_arena: Arena<ChessPosition>`

Nodes reference child edges and associated board states via `usize` indices. A dedicated `position_arena` is implemented to prevent deduplication of board state over consecutive `PieceSelect` and `PieceMove` nodes.

To address memory pressure, arenas are backed by freelists. When subtrees are abandoned (e.g. after a move is chosen), their indices are returned to the freelist. Bulk edge insertion (`push_block`) searches for contiguous free slots. A garbage-collection pass recursively frees all nodes, edges, and board positions of abandoned subtrees asynchronously behind gradient steps. This keeps RAM proportional to the active search tree, enabling larger batch sizes.

=== Node Expansion, Promotions, and Terminals <expansion>
Leaf expansion (@leaf_expansion) transitions the state machine. Expanding a `PieceSelect` edge generates a `PieceMove` node without altering the underlying board state. Expanding a `PieceMove` edge applies the move, computes the new Zobrist hash, evaluates terminal states (including threefold repetition via path history), and generates a new `PieceSelect` node.

Promotions are handled natively by the bipartite structure. When a pawn-to-back-rank destination is expanded, the MCTS intercepts the spatial prior $p$ and distributes it uniformly ($p slash 4$) across four discrete edges (Q, R, B, N), each carrying the corresponding promotion. This enables the model to handle underpromotions through search topology rather than increased output dimensionality.

#figure(
  kind: "algorithm",
  supplement: [Algorithm],
  pseudocode-list(booktabs: true, title: smallcaps[Bipartite Leaf Expansion])[
    + *Require:* Leaf edge $e$, Network prior probabilities $P$, Search path history $H$
    + Let $N_("parent")$ be the parent node of $e$
    + *if* $N_("parent")$ is of type PieceSelect *then*
      + Create $N_("child")$ of type PieceMove
      + Set $N_("child").$selected_square $= e.$target_square
      + Link $e$ to $N_("child")$
    + *else if* $N_("parent")$ is of type PieceMove *then*
      + $s' <-$ Apply move($N_("parent").$selected_square, $e.$target_square)
      + *if* Zobrist($s'$) appears $>= 2$ times in $H$ *then*
        + Create $N_("child")$ as Terminal(Draw)
      + *else if* is_terminal($s'$) *then*
        + Create $N_("child")$ as Terminal(Win/Loss)
      + *else*
        + Create $N_("child")$ of type PieceSelect representing $s'$
      + *end*
      + Link $e$ to $N_("child")$
    + *end*
    + *if* $N_("child")$ is not Terminal *then*
      + *for each* legal destination square $i$ *do*
        + *if* move is pawn promotion *then*
          + Create 4 edges (Q, R, B, N) with prior $= P[i] slash 4$
        + *else*
          + Create 1 edge with prior $= P[i]$
        + *end*
        + Attach edge(s) to $N_("child")$
      + *end*
    + *end*
  ],
) <leaf_expansion>

The fifty-move rule has been shortened to forty full moves (eighty plies) to accelerate training.

=== Edge Selection and Value Propagation <edge_selection>
PUCT scoring uses the contempt-augmented variant derived in @puct:

- $Q_e = W_e - L_e - 0.05 D_e$
- $U_e = c_("puct") dot p_e dot (sqrt(N) + 10^(-8)) / (1 + n_e)$

The selected edge maximizes $Q_e + U_e$. On leaf evaluation, the value vector $[W, D, L]$ is back-propagated up the path. The vector is inverted ($[W, D, L] <- [L, D, W]$) if the current side-to-move $!=$ leaf side-to-move. The path is cleared after each simulation for the next traversal.

=== Mask Integration and Prior Renormalisation <mask_integration>
The network outputs a probability distribution over all 64 squares in unmasked configurations, as opposed to just the valid squares when masked. To produce a valid prior for PUCT, the MCTS computes the illegal mass $lambda = sum_(i in.not A_L(s)) p_i$ and renormalises the legal priors:
$ p'_i = p_i / (1 - lambda) quad forall i in A_L(s) $
The same $lambda$ is the quantity surfaced to the loss function as the curriculum coefficient described in @self-annealing-loss.

== Neural Network Architecture
The `ChessTransformer` is a shared-weight encoder mapping the spatial tensor and the meta-vector to a policy distribution and a W/D/L estimate.

=== Tensor Construction <tensor_construction>
Inputs are first canonicalized to the side-to-move via `flip_board()`, then written into flat `Vec<f32>` slices before being reshaped through Burn's `TensorData` API. This guarantees a contiguous memory layout before the host-to-device transfer, avoiding allocation churn.

- *Spatial tensor ($X in RR^(64 times 14)$):* 12 piece planes, 1 en-passant plane, 1 selected-square plane (zero on the first pass, the highlighted origin square on the second).
- *Meta-tensor ($M in RR^5$):* 4 binary castling rights and 1 continuous half-move clock.

=== Encoder and Output Heads <encoder>
The 64 spatial vectors are linearly projected to $d_("model") = 512$, and learnable rank/file embeddings of size 8 each are summed and broadcast across the grid before being added to the spatial token sequence. The meta-vector is projected and concatenated as a pseudo-[CLS] token at index 64, yielding a $65 times d_("model")$ sequence consumed by the encoder ($n_("layers") = 8$, $n_("heads") = 8$).

After the encoder, the sequence bifurcates:
- Tokens $0..63$ pass through a linear policy head producing 64 spatial logits.
- Token $64$ is passed through a linear value head producing the W/D/L logits.

The model's `forward_classification` method dynamically applies action masking based on configuration. In masked configurations, a boolean tensor identifies illegal squares and `mask_fill` clamps the corresponding logits to $-10^9$ before softmax. In unmasked configurations this step is bypassed and the cross-entropy is computed against the raw distribution, enabling the creation of the curriculum coefficient $lambda$ in @self-annealing-loss.

== Training Pipeline
The training pipeline orchestrates data generation, buffer management, and gradient updates, with each stage engineered to minimise idle time on either CPU or GPU.

=== Pretraining on Mate Evaluations <pretraining>
Before the self-play loop begins, the value head is bootstrapped against a small dataset of positions annotated with mate-in-N evaluations (`mate_evals.tsv`). One hundred gradient steps are run with the loss ratio fixed at $-1$ to ignore policy output. This step is negligible in wall-clock cost and greatly improves search tree efficiency.

=== Batch Expansion and Synchronisation <expansion_synchronisation>
The interface between the CPU-bound MCTS and the GPU-bound inference is implemented in `expand_batch` (@expand_batch). MCTS instances traverse independently in parallel until each has selected a leaf. The threads then synchronise, aggregating canonicalised input tensors and masks into a single batch. Following a unified forward pass, the resulting `NetworkLabels` (policy and value arrays) are scattered back to their respective MCTS instances for edge expansion and value backpropagation. This single-batch boundary is the only synchronisation in the entire self-play loop, scaling compute cost with batch size so long as the GPU has memory to spare.

#figure(
  kind: "algorithm",
  supplement: [Algorithm],
  pseudocode-list(booktabs: true, title: smallcaps("Batched Inference and Prior Renormalisation"))[
    - *Require:* Set of leaf nodes ${n_1, ..., n_B}$, Configuration $C$
    + $X_"batch", M_"meta" <-$ Extract spatial and meta tensors from ${n_1, ..., n_B}$
    + $M_"mask" <-$ Generate legal move boolean masks for all $b in B$
    + *if* $C."masked"$ is True *then*
      + $P_"raw", V_"raw" <-$ TransformerForward($X_"batch", M_"meta", M_"mask"$)
    + *else*
      + $P_"raw", V_"raw" <-$ TransformerForward($X_"batch", M_"meta", "None"$)
    + *end*
    + *for* $b = 1$ *to* $B$ *do*
      + $lambda <- 0$
      + *if* $C."masked"$ is False *then*
        + *for each* illegal square $i$ in $n_b$ *do*
          + $lambda <- lambda + P_"raw"[b][i]$
        + *end*
        + *for each* legal square $j$ in $n_b$ *do*
          + $P_"raw"[b][j] <- P_"raw"[b][j] / (1 - lambda)$
        + *end*
      + *end*
      + $n_b."illegal_mass" <- lambda$
      + $n_b."value" <- V_"raw"[b]$
      + Populate $n_b$ edges using $P_"raw"[b]$
    + *end*
  ],
) <expand_batch>

=== The Self-Annealing Loss in Code <self-annealing-loss>
The composite loss of @self-annealing is computed in `calculate_loss`, which is structurally identical across all four configurations, where only the value of $lambda$ (here 'ratio') varies:
```rust
let policy_probs = log_softmax(policy_pred, 1);
let policy_loss = (target_policy * policy_probs).sum_dim(1).mean().neg();

let value_probs = log_softmax(value_pred, 1);
let value_loss = (target_value * value_probs).sum_dim(1).mean().neg();

(policy_loss * (1.0 + ratio)) + (value_loss * (1.0 - ratio))
```
The `log_softmax`-then-multiply formulation is mathematically equivalent to a softmax followed by KL divergence against soft targets, but avoids the numerical instability of computing the softmax explicitly when logits span a wide range. In the masked configurations, `ratio` is naturally $0$ and the loss reduces to the standard, equally weighted policy/value sum, recovering AlphaZero's objective without a separate code path.

== Performance and Correctness
=== Throughput and Memory <throughput>
Engine throughput is dominated by the batched forward pass of the Transformer, with tree search and arena garbage collection running concurrently behind it. At a batch size and simulation count of $256$ each, steady-state resident memory sits at approximately $11$ GB system RAM and $10$ GB VRAM. CPU usage hovers at 7% while the GPU (NVIDIA RTX 4000 Ada) maxes out at 100%, indicating inference to be the major bottleneck.

=== Verification and Testing <testing>
Three independent layers of verification target the three independent failure modes of the system.

// lies
*Move generation* is tested against the standard perft position suite (Kiwipete, Position 3, Position 4, Position 5, plus the initial position), exercising en-passant, castling through check, promotion, and double-pawn-push edge cases.

*State machine integrity* is checked by round-tripping FEN: every position reached during a 1000-game self-play smoke test is exported to FEN, parsed back, and compared for bitwise equality. Zobrist hashes are verified to be invariant under FEN round-trips and to match across symmetric-castling-rights configurations.

*Network–engine boundary.* Tensor shapes are verified at compile time through Burn's typed tensor API, eliminating tensor mismatch bugs. Zobrist hashes use 64-bit keys, providing cryptographic-grade collision resistance over the search depths encountered during training.

=== Reproducibility <reproducibility>
All hyperparameters used across the four training runs are summarised in @hyperparams. The same values are used for every cell of the experimental matrix; the only variables are the two boolean flags `legal` and `masked`.

#figure(
  table(
    columns: 2,
    align: (left, right),
    stroke: 0.5pt,
    [*Parameter*], [*Value*],
    [Batch size], [256],
    [MCTS simulations / move], [256],
    [Replay buffer capacity], [$2^(19)$],
    [Gradient steps / iteration], [256],
    [Self-play steps / iteration], [256],
    [$d_("model")$], [512],
    [$n_("layers")$], [8],
    [$n_("heads")$], [8],
    [$d_("ff")$], [2048],
    [$c_("puct")$], [1.25],
    [Dirichlet $alpha$], [0.3],
    [Dirichlet $epsilon$], [0.25],
    [Contempt (draw penalty)], [0.05],
    [Optimiser], [AdamW],
    [$beta_1, beta_2$], [0.9, 0.99],
    [Weight decay], [$10^(-4)$],
    [LR schedule], [Noam, factor 0.01, 4000 warmup],
    [Max ply / game], [400],
    [Half-move limit], [80 plies],
    [Pretraining steps (mate eval)], [100],
    [Random seed], [1234],
  ),
  caption: [Hyperparameters held constant across all four configurations.],
) <hyperparams>


// ========================================== // CHAPTER 5: Results // ==========================================
= Results and Evaluation
== Experimental Setup
== Ruleset Convergence Analysis
== Masking and Grokking Analysis

// ========================================== // CHAPTER 6: CONCLUSION // ==========================================
= Conclusion
== Project Evaluation
== Limitations and Future Work
- *Compute Constraints:* Hardware limitations restricted the total number of training epochs, preventing observation of deep late-stage grokking.
- *Move Generation:* Future iterations should replace standard bitboard raycasting with Magic Bitboards for optimal
performance.
- *Kinematic Bootstrapping:* Boot strap the policy head with uniform distributions of valid moves, before self play.
- *Value Bootstrapping:* Boot strap the value head with a robust suite of perfect knowledge (mate evals, endgame tablebases, certain stockfish evaluations).

// ========================================== // CHAPTER 6: Ethics // ==========================================
= Statement of Ethics

// ========================================== // BIBLIOGRAPHY // ==========================================
#bibliography("refs.bib", style: "harvard-cite-them-right")
