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
#import "@preview/cetz:0.5.2"
#import "@preview/cetz-plot:0.1.3"

#set cite(form: "prose")

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
    )[The Impact of Action Masking and \
      Environmental Constraints on \ Rule
      Internalization in Transformer-Based \
      Chess Agents]

    #v(1cm)
    by
    #v(1cm)

    #text(size: 16pt)[Francois Qian] \
    URN: 6702759/7

    #v(2cm)

    A dissertation submitted in partial fulfilment of the \
    requirements for the award of

    #v(1cm)

    #text(size: 16pt)[BACHELOR OF SCIENCE IN COMPUTER SCIENCE]

    #v(0.5cm)

    May 2026

    #v(2.5cm)

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
  [$c_("puct")$],
  [A hyper-parameter controlling the trade-off between exploration and exploitation in the search tree.],

  [$A_L(s)$], [The set of strictly legal moves available in state $s$ according to FIDE rules.],
  [$A_P(s)$], [The set of pseudo-legal moves (geometrically valid but potentially leaving the king in check).],
  [$N_"select"$],
  [A node in the bipartite MCTS representing a board state where the agent must select a 'from' square.],

  [$N_"move"$],
  [A node in the bipartite MCTS where an origin square has been fixed and the agent selects a 'to' square.],

  [$K_"cap"$], [The terminal event of king capture used as the reward signal in the pseudo-legal rule set.],
  [$d_("model")$], [The hidden dimensionality of the transformer encoder's internal representations.],
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
Chess is a deterministic, perfect-information zero-sum game played on an $8 times 8$ grid. Two players command 16 pieces of 6 distinct classes, each governed by strict kinematic rules with the objective of trapping the opponent's king (checkmate). Historically dubbed the "Drosophila of AI" (#cite(label("10.1007/978-1-4613-9080-0_14"))), chess provides an optimal substrate for studying machine intelligence: it presents highly ambiguous intermediate states bound by objective ground truths and a mathematically perfect, yet computationally intractable, solution space (approximately $10^40$ states).

Neural networks learn chess through *reinforcement learning (RL)*, a machine learning paradigm where an autonomous agent learns to make optimal actions in an environment to maximize a reward signal. This typically involves decoupling the model from environment transition dynamics via *logit masking*, a heuristic that artificially zeroes out the probability of illegal actions prior to softmax activation. This is a known optimization that accelerates convergence (#cite(label("DBLP:journals/corr/abs-2006-14171"))). Consequently, State Of The Art (SOTA) engines (e.g., AlphaZero - @silver2017masteringchessshogiselfplay) utilize hardcoded legal move generators to mask out invalid actions. While computationally efficient, this deprives the network of negative feedback for invalid predictions, bypassing the model's need to internally represent the mechanics of the game and leaves the representational dynamics of rule acquisition from scratch in complex Markov Decision Processes (MDP) unexplored.

Literature has largely overlooked the impact of removing these guardrails because standard RL in chess prioritizes asymptotic Elo gain per Floating Point Operation. As AI systems scale toward generalized world models, the ability of an agent to internalize the causal rules of its environment (as opposed to hardcoded environmental constraints) serves as a critical proxy for architectural robustness, reliability, and interpretability.

== Aims and Objectives
This project investigates the emergent learning dynamics and internal representations of a transformer-based self-playing chess agent when logit masking and environmental constraints are removed.

Specific objectives include:
- *Implement high-throughput chess library:* Develop a dual ruleset engine optimized for RL self-play.
- *Architect action-selection models:* Design a two-pass autoregressive transformer and bipartite MCTS to model piece to square selection.
- *Execute comparative training:* Train five configurations, varying move legality (legal/pseudo-legal), masking, and loss-annealing, using identical hyperparameters.
- *Quantify performance metrics:* Measure convergence rates, illegal-move frequencies, and the evolution of rule-abiding behavior in unconstrained models.
- *Analyze spatial reasoning:* Correlate the emergence of legal play with attention distribution patterns in the two-pass encoder.

== Project Description
A custom two-pass autoregressive transformer encoder and a bipartite Monte Carlo Search Tree (MCTS) are introduced to facilitate explicit measurement of model interpretability. This architecture decouples piece selection from destination selection to provide a granular window into the model's spatial reasoning.
Two orthogonal axes of environmental constraint are examined. Crucially, these axes isolate distinct learning targets: *Logit Masking* dictates the policy head's acquisition of piece kinematics, while *Environmental Constraints* dictate the value head's acquisition of survival heuristics.

The experimental configurations are defined across two axes:
+ *Axis 1: Rule Set Convergence via Environmental Constraints*
  - *Control:* Training on a strict legal ruleset.
  - *Test:* Training on a pseudo-legal ruleset where the terminal state is the simpler, denser king capture $(K_"cap")$ over the highly sparse, complex ($A_l = nothing$). This variant of Chess fundamentally alters the Markov Decision Process (MDP) reward function $R(s, a)$ yielding a potentially smoother reward landscape, forcing the value head to learn survival heuristics (e.g., pins) rather than relying on hardcoded environment evaluations.
  - *Hypothesis:* A neural network trained in a pseudo-legal action space will autonomously acquire core chess rules and strategic behaviours through self-play.

+ *Axis 2: Piece Kinematics Internalization via Logit Masking*
  - *Control:* Masked logits (illegal moves are mathematically eliminated prior to selection).
  - *Test:* Unmasked logits (illegal moves are permitted by the network but penalized by the environment/loss function).
  - *Ablation:* Unmasked logits with a fixed policy/value loss ratio, isolating the impact of the proposed self-annealing curriculum loss (@self-annealing).
  - *Hypothesis:* Unmasked training will accelerate convergence by first learning piece kinematics before strategy, forming an automated curriculum.

In the unmasked, pseudo-legal regime, the network faces a dual-optimization problem: it must learn the physical constraints of the board (the rules) and the strategic evaluation of states (the game). It is predicted that this configuration will exhibit non-monotonic learning dynamics: an initial regime of high illegal-move proposals, followed by a sharp convergence toward $A_P(s)$ as kinematics are internalized, acting as an emergent curriculum before strategic play develops in $A_L(s)$.


== Scope and Limitations
Several limitations bound the scope of the work:
+ The comparison of interest is across five training configurations over asymptotic performance. As such, models are not benchmarked against external engines; absolute Elo against SOTA requires significantly more compute than the project budget allows.
+ The value head is bootstrapped against MCTS root values over terminal outcomes to accelerate convergence, constrained by GPU throughput.

== Thesis Structure
This dissertation is structured as follows:
- *Chapter 2* surveys the literature surrounding reinforcement learning, transformer architectures, action masking, and curriculum learning, identifying the gap this work addresses.
- *Chapter 3* details the system design, including the bipartite MCTS, the two-pass encoder, and the self-annealing loss function.
- *Chapter 4* outlines the implementation of the Rust engine and training pipeline, with particular attention to the engineering decisions that make high-throughput self-play tractable.
- *Chapter 5* presents the experimental design.
- *Chapter 6* presents the results and evaluation across the four configurations.
- *Chapter 7* concludes and discusses avenues for future work.

// ==========================================
// CHAPTER 2: LITERATURE REVIEW
// ==========================================
// Grokking: Generalization beyond overfitting on small algorithmic datasets [power2022grokking]
// XXII. Programming a computer for playing chess (Shannon) [Shannon01031950]
// An overview of the action space for deep reinforcement learning [zhu2021overview]
// Mastering Chess and Shogi by Self-Play with a General Reinforcement Learning Algorithm (Alphazero) [silver2017masteringchessshogiselfplay]
// Mastering Atari, Go, chess and shogi by planning with a learned model (Muzero) [Schrittwieser_2020]
// A Closer Look at Invalid Action Masking in Policy Gradient Algorithms (huang) [DBLP:journals/corr/abs-2006-14171]
// Mastering Chess with a transformer Model (2024) [monroe2026chessformer]
// Evidence of Learned Look-Ahead in a Chess-Playing Neural Network (2024) [jenner2024evidencelearnedlookaheadchessplaying]
// Acquisition of Chess Knowledge in AlphaZero (2021) [DBLP:journals/corr/abs-2111-09259]
// Giraffe: Using Deep Reinforcement Learning to Play Chess (2015) [DBLP:journals/corr/Lai15a]
// Curriculum learning [10.1145/1553374.1553380]
// Action Space Shaping in Deep Reinforcement Learning (Kanervisto et al) [9231687]
// Factored Action Space Representations for Deep Reinforcement learning [DBLP:journals/corr/SharmaSRR17]

= Literature Review
== Chess as a Markov Decision Process (MDP)
In reinforcement learning (RL), chess is modelled as a MDP defined by the tuple $(S, A, T, R)$, where $S$ is the state space ($|S| approx 10^40$), $A$ is the action space, $T: S times A -> S$ is the deterministic transition function, and $R: S times A -> {-1, 0, 1}$ is the reward function (@andrew2018reinforcement).

@Shannon01031950 examined the complexity of chess as a MDP by introducing the distinction between Type A (brute force + fixed depth) and Type B (selective search + heuristic pruning) search strategies, which then provided the groundwork for modern learned selective search (MCTS + neural priors) as a hybrid strategy.

Standard RL chess engines enforce a legal action space $A_L(s) subset A$ in which moves leaving the king in check are excluded by the environment (@fide2023laws). This project introduces a relaxed pseudo-legal action space $A_P(s)$ such that $A_L(s) subset A_P(s) subset A$, permitting all geometrically valid piece movements and shifting the terminal condition from the abstract _checkmate_ to the explicit _king capture_ $K_("cap")$, fundamentally altering the MDP while still retaining the same optimal policy.

== Chess Played by Machines <chess_by_machines>
=== Giraffe
#cite(label("DBLP:journals/corr/Lai15a")) introduced Giraffe, a chess engine that learned to autonomously learn an evaluation function via RL self-play. The system relied on the TD-leaf($gamma$) algorithm to train the neural network, a form of value bootstrapping, to reach International Master (IM) level performance, and marked a turning point in the neural-network revolution in chess engines.

=== AlphaZero
The paradigm of self-play RL in chess was defined by AlphaZero (@silver2017masteringchessshogiselfplay), which combined Monte Carlo Tree Search (MCTS) with deep convolutional neural networks (CNNs) for policy and value evaluation. AlphaZero relies entirely on a hardcoded environment to generate $A_L(s)$, bypassing the need for the network to internalize the rules of the game.

=== MuZero
MuZero (@Schrittwieser_2020) removed the need for a known transition dynamics model by learning a latent environment simulator, outperforming models trained in hard-coded environments. While MuZero demonstrates the ability to adhere to $A_L(s)$ implicitly within its hidden state transitions, its policy head still undergoes logit masking during MCTS rollouts and the network never explores $A_P(s)$. The literature lacks insight into how the removal of environmental guardrails or action masking impacts the sample efficiency and representational robustness and performance of the underlying network.

While both AlphaZero and MuZero proved the viability of MCTS + RL, their success is reliant on the embedded spatial CNNs biases that transformers remove, making transformers a superior substrate for studying pure rule internalization.

=== Transformers in Chess
Recent advancements demonstrate the efficacy of transformer architectures in chess, shifting away from the spatial inductive biases of CNNs.
@McGrath_2022 demonstrated that AlphaZero-style networks acquire human-interpretable concepts (material balance, mobility, king safety) over the course of training. Specifically, concepts like piece value appear in early clusters within the network, with checkmate detection appearing later in deeper layers.

#cite(label("monroe2024masteringchesstransformermodel")) achieved Grandmaster-level performance with a transformer relying purely on attention to evaluate board states, without explicit search. #cite(label("jenner2024evidencelearnedlookaheadchessplaying")) provided complementary evidence of learned look-ahead in chess-playing networks, showing that attention heads encode candidate future board states.

=== Efficiently Updatable Neural Networks (NNUE)
At the time of writing, Stockfish (@Stockfish_developers_Stockfish) is the strongest chess engine (@CCRL2026), utilizing the NNUE architecture, a shallow, SIMD-optimized neural network optimized for cpu inference to leverage massive MCTS throughput. As such, the network lacks the attention layers required to develop the "intuition" of a transformer, making up for in vastly superior search depth via incremental updates (@Nasu2018).

== AlphaStar
In domains with highly structured action spaces, such as StarCraft II, AlphaStar (@Vinyals2019) demonstrated the efficacy of autoregressive policy heads to factor complex joint distributions. Adapting this to chess, factoring the joint probability as $ P(a|s) = P("from"|s) dot P("to"|s, "from") $ collapses the output dimensionality from $d_"model" times 4672$ to $d_"model" times 64$ and gives rise to two distinct, observable failure modes: attention failure (identifying movable pieces) and kinematic failure (how pieces move).

== The ELIZA effect <eliza>
The historical development of AI is haunted by the ELIZA effect, a phenomenon where users attribute humanlike cognitive depth or intentionality to systems performing simple
symbol manipulation (@e5327ff8-7444-3fe8-a163-48f11e4bdba2). In chess, this effect manifests when observers interpret the outputs of high-dimensional, flat-policy engines as "strategic intuition". AlphaZero masks thousands of illegal actions, creating a discursive illusion of understanding; the model appears to "know" how to play, yet may simply be filtering a massive probability distribution through environmental guardrails.

These results frame the central architectural decision behind the factored auto-regressive encoder: decoupling $P("from"|s)$ from $P("to"|s, "from")$ provides an attempt to move from mimicry to internalization, while simultaneously providing an observable window into distinct failure modes: attention failure (identifying movable pieces) and kinematic failure (how pieces move).

== Invalid Action Masking
In policy gradient algorithms, it is standard practice to prevent the selection of illegal actions via invalid action masking (logit masking) (@vinyals2017pointernetworks). Masking applies a transformation to the policy logits $z$ prior to the softmax activation:

$ P(a_i|s) = exp(z_i + M_i) / (sum_j exp(z_j + M_j)) $

where $M_i = -infinity$ if $a_i in.not A_L(s)$, and $0$ otherwise.

Masking can also be introduced through environmental constraints. The typical legal action space $A_L(s)$ is one such mask - a subset of $A_P(s)$, where the concept of "check" is substantiated, encoding king safety. /*The relaxed pseudo-legal state $A_P(s)$ is re-introduced, allowing king capture and removing the concept of "check".*/ For any state $s$, an optimal policy $pi^*$ trained in $A_P$ must implicitly learn to prune the action space such that:
$ pi^*(a|s) -> 0 quad "for all" a in (A_P(s) without A_L(s)) $
Under this paradigm, concepts such as pins and checkmating nets cease to be hard-coded environmental properties (valid in $A_P(s)$) and instead become _emergent survival heuristics_ within the value landscape.

#cite(label("DBLP:journals/corr/abs-2006-14171")) demonstrated that collapsing the exploration space via action masking significantly improves sample efficiency. For an MCTS-based agent, masking is required as simulations into invalid state transitions will propagate corrupted value estimates up the search tree. However, Huang and Ontañón's analysis is restricted to small, flat action spaces where the legal subset is a fixed function of a few features (MicroRTS).

Chess presents a qualitatively harder case: legality depends on the global board configuration, and the function $A_L(s)$ is itself nontrivial to compute.
Logit masking prevents the network receiving feedback from masked indices as gradients are nullified in the compute graph, preventing backpropagation of loss. This work investigates whether the conclusions of #cite(label("DBLP:journals/corr/abs-2006-14171")) generalize to environments where the rules constitute an algorithmic learning target.

== Learning Dynamics
The unmasked, pseudo-legal regime forces the network to learn the rules of chess from scratch as a precondition for strategic evaluation, raising the prospect of non-monotonic learning dynamics. #cite(label("power2022grokking")) identified the phenomenon of _grokking_, in which neural networks trained on small algorithmic datasets exhibit sharply delayed generalization long after overfitting the training set, mediated by weight-decay pressure. Chess is an algorithmic dataset embedded in a strategic MDP, showcasing the potential of analogous phase transitions.

#cite(label("10.1145/1553374.1553380")) formalized _curriculum learning_, showing that ordering training data from simple to complex acts as a regularizer, guiding optimization toward superior local minima inaccessible through random shuffling. A pseudo-legal environment serves implicitly as curriculum where an agent first receives rewards through piece kinematics and king capture before survival heuristics under $A_L$.

#cite(label("DBLP:journals/corr/SharmaSRR17")) introduced a framework for factoring discrete action spaces into compositional, basis-like components, enabling cross-action generalization among actions that share common factors. This principle is instantiated in the factored engine architecture, where each selection and destination shares a common output head.

== Research Gap
While the efficacy of action masking is well-documented in general RL (#cite(label("DBLP:journals/corr/abs-2006-14171"))), and transformers have proven capable of modelling chess at a high level (@jenner2024evidencelearnedlookaheadchessplaying), the intersection of these domains is unexplored. No published literature isolates the impact of environmental guardrails on the representational quality of transformer-based MCTS agents, the dynamics of rule internalization, or overall agent performance when those guardrails are removed.

This project addresses this gap by training four distinct configurations across two orthogonal axes: Environment Constraints (Legal vs. Pseudo-Legal) and Logit Masking (Masked vs. Unmasked) to investigate how the environment manifests within a transformer based chess engine and the impact on performance.

// ==========================================
// CHAPTER 3: DESIGN
// ==========================================
= System Design
== System Architecture Overview <overview>
The system is designed as a closed-loop reinforcement learning pipeline, decoupling the environment transition dynamics from the neural network's internal representations. The architecture consists of three primary components: a custom deterministic environment simulator supporting dual rulesets, a two-pass autoregressive transformer encoder, and a bipartite MCTS algorithm. The system operates via self-play, generating trajectories to optimize the network.

Each component is designed such that the experimental axes can be toggled on or off independently without altering any other part of the system. This is essential to the validity of the comparison: the four configurations differ only in the two boolean flags governing their respective axes, with all hyperparameters held constant.

== Functional and Non-Functional Requirements <requirements>
The methodology imposes four hard requirements on the implementation, each of which constrains the design space from @env-state to @self-annealing.

+ *Configuration Parity:* The four cells of the experimental matrix must differ only in the two boolean flags governing rule set and masking. No code path may exist that is reachable in one configuration but not another, with the sole exception of the masking step itself. This rules out per-configuration training scripts and motivates the unified `forward_classification` implementation in @encoder.

+ *Self-Play Throughput:* Multiple full training runs under a single GPU budget requires massive throughput. This rules out cache-hostile reference-counted tree structures and dynamic move-list allocation (heap pressure), motivating arena allocation (@arenas) and stack-allocated move lists (@move_gen).

+ *Reproducibility:* Self-play is intrinsically stochastic, but every source of randomness must be seedable. Dirichlet noise (@Silver2017), temperature sampling, replay sampling, and weight initialization all consume from seeded RNGs.

+ *Auditability:* Observed learning dynamics must be attributable to the network rather than to engine artefacts. This requires the move generator to be independently testable against a reference (@testing) and the masking boundary (@move_gen) to be unambiguous in code.

== Environment Formulation and State Representation <env-state>
=== Dual Ruleset MDPs <rulesets>
The *Legal Environment $E_l$* adheres to standard FIDE rules. The action space ($A_l(s)$) is strictly contained to moves that do not leave the king in check. The terminal reward $R(s, a)$ is evaluated upon Checkmate, Stalemate, or standard draw conditions.
The *Pseudo-Legal Environment $E_P$* relaxes the action space to $A_P(s)$, permitting all geometrically valid piece movements regardless of king safety. The terminal condition shifts from checkmate to the explicit king-capture event $K_"cap"$. FIDE draw conditions (e.g., insufficient material, fifty-move rule) are retained to guarantee finite rollouts. In $E_P$, stalemate is redefined: if a king is not in check but the agent possesses no pseudo-legal moves (e.g., all pieces are physically blocked), the state evaluates as a draw.

=== State Canonicalization and Tensor Formulation <canonicalisation_tensors>
To enforce symmetric learning and halve the state space, all board states are canonicalized to the perspective of the side-to-move (@silver2017masteringchessshogiselfplay). If it is Black's turn, the board, and castling rights are geometrically flipped.
The state $s$ is mapped to a spatial tensor representation $X$ and a scalar meta-tensor $M$, the exact dimensionalities of which are detailed in @tensor_construction.

To explicitly measure spatial reasoning, the system factors the joint probability of a move $P(a|s)$ into an autoregressive sequence: $P(a|s) = P("from"|s) times P("to"|s,"from")$. The network executes two forward passes using shared weights:
+ *Pass 1 (Piece Selection)*: The network evaluates the board state with the selected square plane zeroed out, outputting a policy distribution over the 64 squares to select the origin square (_from_). The value head will evaluate the board state.
+ *Pass 2 (Destination Selection)*: The origin square is encoded into the 14th plane of the spatial tensor. The network re-evaluates the state, outputting a policy distribution over the 64 squares to select the destination (_to_). The value head will evaluate the position given a piece to move, effectively evaluating the viability of the selected piece.

The factored $64 -> 64$ output lacks a third dimension for promotion piece selection. Promotions are instead handled via the search topology: when a pawn destination on the back rank is expanded, the corresponding edge is replaced by four sub-edges (Q, R, B, N), equally dividing the prior probability. This delegates under-promotion to the search tree, where the additional branching factor is amortized over many simulations.

=== Network Topology <network_topology>
The model uses a transformer Encoder architecture without convolutions (@dosovitskiy2020image). The spatial tensor $X$ is linearly projected to $d_"model"$, and $2D$ learned positional embeddings are added to retain geometric context. The meta-tensor $M$ is linearly projected and concatenated as a pseudo-[CLS] token, resulting in a sequence length of 65 (@monroe2024masteringchesstransformermodel).

The model is symmetrically scaled as a variant of ChessTransformer (@monroe2024masteringchesstransformermodel): $n_"heads" = 8$, $n_"layers" = 8$, $d_"model" = 8 times 64 = 512$, $d_"ff" = 4 times d_"model"$, meeting a middle ground between CF-6M and CF-240M.

=== Output Heads <output_heads>
The final latent representation is routed into two distinct heads (#cite(label("DBLP:journals/corr/MnihBMGLHSK16"))):
+ *Policy head:* A linear projection of the 64 spatial tokens to $RR^64$, producing unnormalized logits over square selection.
+ *Value head:* A linear projection of the [CLS] token to $RR^3$, producing W/D/L logits. In Pass 1 this evaluates the raw state; in Pass 2 it evaluates the state conditional on the selected origin square.

== Bipartite Monte Carlo Tree Search <method-mcts>
=== Topology <mcts_topology>
To accommodate the two-pass autoregressive policy, the search tree alternates between two node types:
- *Selection Nodes ($N_"select")$:* Represent a board state. Edges represent the choice of an origin square.
- *Action Nodes ($N_"move")$:* Represent a board state plus a selected origin square. Edges represent the destination square; transitioning the environment to a new $N_"select"$ node.

A complete move thus traverses a (Selection, Action) pair. This bipartite structure explicitly mirrors the models autoregressive nature, physically separating the two failure modes of factored policy in the search tree.

=== Edge Selection <puct>
During traversals, edges are selected using a modified Predictor + Upper Confidence Bound applied to Trees (PUCT) algorithm (@silver2017masteringchessshogiselfplay). To mitigate draw-seeking behaviour, a contempt factor is baked into the empirical action value $Q(s, a)$.
$
       Q(s, a) & = W - L - 0.05 D \
       U(s, a) & = c_("puct") dot P(s, a) dot (sqrt(N(s)) + 10^(-8)) / (1 + N(s, a)) \
  "PUCT"(s, a) & = Q(s, a) + U(s, a)
$
where $W, D, L$ are the mean W/D/L value estimates accumulated on the edge, $P(s, a)$ is the prior, and $N(s)$, $N(s, a)$ are the parent and edge visit counts respectively.

=== Exploration Mechanisms <exploration>
Exploration is injected through two mechanisms. Dirichlet noise is applied exclusively to the root prior, with $P(s, a) <- (1 - epsilon) P(s, a) + epsilon dot eta$, with $eta tilde "Dir"(alpha)$, $alpha = 0.3$, and $epsilon = 0.25$ (@Silver2017). At the root, the move played is sampled in proportion to $N(s,a)^(1/t)$, adapting the @silver2017masteringchessshogiselfplay method by scaling $t$ dynamically with ply count and remaining material, transitioning from exploratory in the opening to deterministic in endgames.

=== Simulation Integrity and the Masking Boundary <integrity>
Invalid state transitions will corrupt value propogation throughout the search tree. To maintain simulation integrity, action masking is performed indiscriminately by the orchestrator to ensure the search space involves only valid squares. The experimental distinction lies in whether action masking is applied to the network's raw logits during loss calculation. In masked configurations, illegal logits are clamped to $-10^9$ prior to softmax, training the model within a distribution restricted to valid squares. In unmasked configurations, the policy loss is computed over the raw 64 square distribution, punishing the model for assigning weight to invalid squares. The training signal still differs in whether the network is shielded from or penalized for invalid predictions, despite MCTS rollouts being strictly contained to the valid action space.

== Reinforcement Learning Pipeline
=== Self-Play and Replay Buffer <method-self-play>
Trajectories are generated via highly parallelized self-play. Samples are stored in a FIFO replay buffer of capacity $2^(19) = 524288$. To prevent infinite games, rollouts are forcibly terminated as draws after 400 plies, or whenever the MCTS root value estimate for a draw exceeds $0.95$, relaxed to $0.75$ after 60 plies (@silver2017masteringchessshogiselfplay).

== Optimization and the Self-Annealing Loss <self-annealing>
The network is optimized with AdamW (#cite(label("DBLP:journals/corr/abs-1711-05101"))) ($beta_1 = 0.9$, $beta_2 = 0.99$, weight decay $10^(-4)$) under a Noam learning-rate schedule (#cite(label("DBLP:journals/corr/VaswaniSPUJGKP17"))) with factor $0.01$ and 4000 warm-up steps. The value head is bootstrapped against the MCTS root-value distribution rather than the eventual game outcome (TD-like) (@Sutton1988) for computational feasibility.

The principal methodological contribution lies in the loss function. For a sample $(s, pi, z)$ with policy prediction $p$ and value prediction $v$, the loss is:
$
  L = (1 + lambda) dot underbrace((-sum_a pi(a|s) log p(a|s)), L_("policy")) + (1 - lambda) dot underbrace((-sum_i z_i log v_i), L_("value"))
$
where $lambda in [0, 1]$ is the mean probability mass that the network has assigned to illegal moves over the most recent batch of leaf expansions.

This acts as a strict mathematical curriculum. When the network proposes many illegal moves $lambda approx 1$, the policy loss weight approaches $2$ and the value loss weight approaches $0$. The network is forced to learn piece kinematics before state evaluation. As the model internalizes piece kinematics, the weights balance to $1:1$. This approach to curriculum learning is unique to the unmasked configuration, as $lambda$ is dependent on illegal move weights ($lambda$ is naturally zero in masked configurations).

== Discussion of Design Challenges <design_challenges>
Three design problems proved materially harder than expected.

+ *Bipartite MCTS Architecture:* Implementing a novel bipartite Monte Carlo Tree Search (MCTS) in Rust without existing reference code introduced significant ownership conflicts. Standard pointer-based tree structures collided with the borrow checker, while trait-based abstractions incurred unacceptable dynamic dispatch overhead. The architecture was ultimately refactored using *arenas* (@Albrecht2009) which utilized manual indexing to bypass the borrow checker and maximize throughput.

+ *Memory Management:* High-frequency node generation (batch size 256; 256 simulations per move) led to rapid heap exhaustion under a naive "reset-on-gameover" policy. The resulting memory pressure triggered the OOM killer during extended training sessions. To stabilize the system, a custom generational garbage collector was implemented to reclaim unexplorable nodes within the arena, trading minor allocation overhead for long-term operational stability.

+ *Performance Evaluation:* Initial plans to evaluate engine strength post-hoc via stored checkpoints proved unfeasible due to storage overflow. The system was transitioned to inline Average Centipawn Loss (ACPL) evaluation using Stockfish. Implementation required embedding a UPX-compressed Stockfish binary for reproducibility and managing synchronous I/O overhead. This shift eliminated the need for excessive checkpoint storage and allowed for real-time performance tracking, despite introducing minor mutex-related bottlenecks.

// ==========================================
// CHAPTER 4: IMPLEMENTATION
// ==========================================
= Implementation Details
This chapter details the software architecture and engineering paradigms utilized to construct the self-play reinforcement learning system. The implementation translates the methodology into a high-throughput, reproducible execution environment optimized for three criteria: clean experimental toggling, sufficient throughput to complete four training runs within the project timeframe, and system auditability to isolate learning dynamics from engine artifacts.

The architecture comprises a custom bitboard-based chess engine, a bipartite MCTS utilizing flat arena allocators, and a shared-weight transformer encoder implemented via the Burn deep learning framework (@burn_framework). All components are parallelized to bridge theoretical design with optimized execution.

== Software Stack and Overview <stack>
The system is implemented entirely in Rust (@matsakis2014rust, @rust_language). The choice of Rust over C++ or Python is driven by the strict requirements of high-throughput MCTS. Python's Global Interpreter Lock (GIL) precludes true multithreading, forcing reliance on multi-processing which incurs prohibitive IPC serialization overhead for tree search. While C++ offers raw performance, complex concurrent tree mutations frequently introduce data races. Rust's ownership model and strict aliasing rules guarantee thread safety at compile time, enabling "fearless concurrency".

Neural network operations are executed via the Burn framework (@burn_framework). Unlike PyTorch's C++ bindings (LibTorch), which carry heavy binary bloat and tie execution to NVIDIA CUDA, Burn provides a backend-agnostic autograd module. By utilizing Burn's WGPU backend, the engine compiles to a standalone binary capable of executing hardware-accelerated tensor operations across CUDA, Metal, and Vulkan without environment configuration (@BurnDocs2026).

The reinforcement learning loop is designed for maximum throughput via lock-free concurrency. At the initialization of each training iteration, $B$ parallel game instances are spawned. As each MCTS instance encapsulates its own memory arenas, the Rayon library's `par_iter_mut()` is utilized to distribute tree traversals across all available CPU cores without mutex contention.

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
The environment is a custom chess engine optimized specifically for MCTS rollouts. State representation is tightly packed (@bitboards), and move generation relies on compile-time precomputed masks (@move_gen) to minimize branching and minimize CPU cache misses.

=== Bitboards <bitboards>
Alternative state representations, such as $8 times 8$ mailbox arrays, require branching loops to evaluate sliding piece attacks. To maximize throughput, the board state is instead encoded using Bitboards (@cpw_bitboards). The `ChessBoard` struct uses a `[[Bitboard; 6]; 2]` array, representing the 6 piece types across 2 colors, alongside 3 aggregate occupancy bitboards (White, Black, All). The struct is annotated `#[repr(align(64))]` ensuring alignment with cache-line boundaries, preventing false sharing and making move generation cache-friendly under aggressive parallel access. Hardware intrinsics (`trailing_zeros` and `leading_zeros`) are used for $O(1)$ piece iteration and bit isolation.

=== Move Generation <move_gen>
The engine employs a two-step deferred verification architecture for move generation:

*Precomputed Lookup Tables:* Knight, King, Pawn, Rook, Bishop, and Rook/Bishop direction masks, together with a $64 times 64$ `BETWEEN` table, are constructed via `const fn` and embedded directly in the binary as `pub const` arrays. This elides the static initialization phase entirely and allows the compiler to fold lookups into immediate operands where it can prove the index.

*Sliding-Piece Move Generation:* Sliding pieces are generated by AND-ing each direction mask with the global occupancy and isolating the first blocker via LSB or MSB depending on the direction (@cpw_bitscan):
```rust
let to_sq = if i < 2 { blockers.pop_msb() } else { blockers.pop_lsb() };
```
The blocker square is included in the ray if it is occupied by an enemy piece (a capture), and the precomputed `BETWEEN[from][blocker]` mask supplies the squares strictly between origin and blocker; the viable destination squares.

*Stack-Allocated Move Lists:* Generated moves are written to a stack-allocated `ArrayVec<ChessMove, 128>`, capping the branching factor to eliminate heap overhead while maintaining a statistically sufficient window for over 99% of chess positions.

*Deferred Legality Verification:* Rather than implementing computationally expensive pin-detection logic during generation, legality is verified post-hoc. The `is_legal()` method copies only the necessary board state, applies the pseudo-legal move, and evaluates if the resulting state leaves the active King under attack via `is_square_attacked()`. This deferred approach significantly reduces branching complexity while maintaining high throughput.

=== Zobrist Hashing <zobrist>
State repetition detection requires hashing board states. Cryptographic hashes (e.g., SHA-256) are computationally prohibitive, and full-state serialization is cache-inefficient. Therefore, Zobrist hashing @Zobrist1970 is implemented. A static table of 64-bit pseudo-random keys (`ZobristKeys`) is lazily initialized using Rust's `OnceLock`. The hash is computed in $O(P)$ time, where $P$ is the number of active pieces, by XOR-ing the keys corresponding to the current board state, castling rights, en-passant file, and side-to-move. While incremental $O(1)$ hashing is theoretically optimal for sequential play, the $O(P)$ approach was selected to eliminate complex state-tracking across independent MCTS nodes. Profiling indicates this computational cost is negligible relative to network inference (@throughput).

== Bipartite Monte Carlo Tree Search <bipartite_mcts>
A standard MCTS represents a complete action as a single edge between two states (#cite(label("conf/cg/Coulom06")). To map the search topology onto the network's two-pass autoregressive policy, the search tree is implemented as a bipartite graph alternating `PieceSelect` and `PieceMove` nodes (@method-mcts).

=== Arena Allocators <arenas>
To avoid memory fragmentation and pointer-chasing overhead typically associated with recursive tree structures, the MCTS utilizes flat, index-based arena allocators (@hanson1988fast):
```rust
struct Arena<T> {
    buffer:   Vec<T>,
    freelist: Vec<usize>,
}

struct Mcts {
  node_arena: Arena<MctsNode>,
  edge_arena: Arena<MctsEdge>,
  position_arena: Arena<ChessPosition>,
  dead_nodes: Vec<usize>,
  ...
}
```

Nodes reference child edges and associated board states via `usize` indices. A dedicated `position_arena` is implemented to prevent deduplication of board state over consecutive `PieceSelect` and `PieceMove` nodes.

While Hanson's original arenas are deallocated in bulk, this implementation incorporates slab-style object reuse (@bonwick1994slab) to handle the high churn of MCTS subtrees. When subtrees are abandoned (e.g. after a move is chosen), their indices are returned to the freelist. By maintaining a freelist within the arena, abandoned node indices are recycled without requiring a full heap deallocation. Bulk edge insertion (`push_block`) searches for contiguous free slots. A garbage-collection pass recursively frees all nodes, edges, and board positions of abandoned subtrees. This keeps RAM proportional to the active search tree, enabling larger batch sizes.

=== Node Expansion, Promotions, and Terminals <expansion>
Leaf expansion transitions the state machine. Expanding a `PieceSelect` edge generates a `PieceMove` node without altering the underlying board state. Expanding a `PieceMove` edge applies the move, computes the new Zobrist hash (@Zobrist1970), evaluates terminal states (including threefold repetition via path history), and generates a new `PieceSelect` node.

Promotions are handled natively by the bipartite structure (@canonicalisation_tensors). When a pawn-to-back-rank destination is expanded, the MCTS intercepts the spatial prior $p$ and distributes it uniformly ($p slash 4$) across four discrete edges (Q, R, B, N), each carrying the corresponding promotion. This enables the model to handle underpromotions through search topology rather than increased output dimensionality.

The fifty-move rule has been shortened to forty full moves (eighty plies) to bound maximum episode length and reduce MCTS memory footprint.

#figure(
  kind: "algorithm",
  supplement: [Algorithm],
  pseudocode-list(booktabs: true, title: smallcaps[Leaf Expansion])[
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



=== Edge Selection and Value Propagation <edge_selection>
PUCT scoring uses the contempt-augmented variant derived in @puct:

- $Q_e = W_e - L_e - 0.05 D_e$
- $U_e = c_("puct") dot p_e dot (sqrt(N(s)) + 10^(-8)) / (1 + N(s, a))$

The selected edge maximizes $Q_e + U_e$. On leaf evaluation, the value vector $[W, D, L]$ is back-propagated up the path. The vector is inverted ($[W, D, L] <- [L, D, W]$) if the current side-to-move $!=$ leaf side-to-move. $10^(-8)$ is added to the exploitation numerator to ensure non-zero exploration of unvisited nodes. The path is cleared after each simulation for the next traversal.

=== Mask Integration and Prior Renormalization <mask_integration>
The network outputs a probability distribution over all 64 squares in unmasked configurations. To produce a valid prior for PUCT, the MCTS computes the illegal mass $lambda = sum_(i thin cancel(in) thin A_X (s)) p_i$ and renormalizes the legal priors:

$ p'_i = p_i / (1 - lambda) quad forall i in A_X (s) $

where $A_X(s)$ denotes the active configuration ($A_L$ or $A_P$). The same $lambda$ is surfaced to the loss function as the curriculum coefficient described in @self-annealing-loss.

== Neural Network Architecture
The `ChessTransformer` is a shared-weight encoder mapping the spatial tensor and the meta-vector to a policy distribution and a W/D/L estimate. As mentioned in @chess_by_machines, transformer architectures hold superior representational quality over alternative architectures such as the CNN, which hold spatial biases, and NNUE, which optimizes for search throughput over one-shot strength.

=== Tensor Construction <tensor_construction>
Inputs are canonicalized to the side-to-move via `flip_board()` before being written into flat `Vec<f32>` slices and reshaped through Burn's `TensorData` API. This guarantees a contiguous memory layout before the host-to-device transfer, avoiding allocation churn.
Tensor shapes are as follows:
- *Spatial tensor ($X in RR^(64 times 14)$):* 12 piece planes, 1 en-passant plane, 1 selected-square plane (zero on the first pass, the highlighted origin square on the second).
- *Meta-tensor ($M in RR^5$):* 4 binary castling rights and 1 continuous half-move clock.

=== Encoder and Output Heads <encoder>
As stated in @network_topology, the 64 spatial vectors are linearly projected to $d_("model") = 512$, and learnable rank/file embeddings of size 8 each are summed and broadcast across the grid before being added to the spatial token sequence. The meta-vector is projected and concatenated as a pseudo-[CLS] token at index 64, yielding a $65 times d_("model")$ sequence to be consumed by the encoder ($n_("layers") = 8$, $n_("heads") = 8$).

After the encoder, the sequence bifurcates:
- Tokens 0 ... 63 pass through a linear policy head producing 64 spatial logits.
- Token 64 is passed through a linear value head producing the W/D/L logits.

The model's `forward_classification` method dynamically applies action masking based on configuration. In masked configurations, a boolean tensor identifies illegal squares and `mask_fill` clamps the corresponding logits to $-10^9$ before softmax. In unmasked configurations this step is bypassed and the cross-entropy is computed against the raw distribution, enabling the creation of the curriculum coefficient $lambda$ in @self-annealing-loss.

== Training Pipeline
The training pipeline orchestrates data generation, buffer management, and gradient updates, with each stage engineered to minimize idle time on either CPU or GPU.

=== Value Head Bootstrapping <pretraining>
Before the self-play loop begins, the value head is trained against a small dataset of positions annotated with mate-in-N evaluations (`mate_evals.tsv`, @lichess_db). One hundred gradient steps are run with the loss ratio fixed at $-1$ to ignore policy output. This step is negligible in wall-clock cost while greatly improving search tree efficiency.

=== Batch Expansion and Synchronization <expansion_synchronisation>
The interface between the CPU-bound MCTS and the GPU-bound inference is implemented in `expand_batch`. MCTS instances traverse independently in parallel until each has selected a leaf. The threads then synchronize, aggregating canonicalized input tensors and masks into a single batch. Following a unified forward pass, the resulting `NetworkLabels` (policy and value arrays) are scattered back to their respective MCTS instances for edge expansion and value backpropagation. This single-batch boundary is the only synchronization point in the entire self-play loop, scaling compute cost with batch size so long as the GPU has memory to spare.

#figure(
  kind: "algorithm",
  supplement: [Algorithm],
  pseudocode-list(booktabs: true, title: smallcaps("Batched Inference and Prior Renormalization"))[
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
The composite loss defined in @self-annealing is implemented via `log_softmax` followed by exact multiplication:
```rust
let policy_probs = log_softmax(policy_pred, 1);
let policy_loss = (target_policy * policy_probs).sum_dim(1).mean().neg();

let value_probs = log_softmax(value_pred, 1);
let value_loss = (target_value * value_probs).sum_dim(1).mean().neg();

(policy_loss * (1.0 + ratio)) + (value_loss * (1.0 - ratio))
```
The `log_softmax`-then-multiply formulation is mathematically equivalent to a softmax followed by KL divergence against soft targets, but avoids the numerical instability of computing the softmax explicitly when logits span a wide range. `ratio` is naturally zero in masked configurations, and set to zero in the fixed-ratio ablation configuration based on an `annealing` flag. This unified implementation recovers the AlphaZero objective, the self-annealing curriculum, and the fixed-ratio ablation without separate code paths.

== Performance and Correctness
=== Throughput and Memory <throughput>
Engine throughput is dominated by the batched forward pass of the transformer, with tree search and arena garbage collection running concurrently behind it. At a batch size and simulation count of $256$ each, steady-state resident memory sits at approximately $11$ GB system RAM and $10$ GB VRAM. CPU utilization remains at 7% while GPU (NVIDIA RTX 4000 Ada) utilization reaches 100%, confirming network inference as the primary system bottleneck.

=== Verification and Testing <testing>
Three independent layers of verification target the three independent failure modes of the system.

*Move generation* is tested against the standard perft position suite (Kiwipete, Position 3, Position 4, Position 5, plus the initial position), exercising en-passant, castling through check, promotion, and double-pawn-push edge cases.

*Tensor shapes* are verified at compile time through Burn's typed tensor API, eliminating tensor mismatch bugs. *Zobrist hashes* use 64-bit keys, providing cryptographic-grade collision resistance over the search depths encountered during training.

*Assertions* are used throughout the stages of MCTS rollouts to ensure correctness in simulations.

=== Reproducibility <reproducibility>
All hyperparameters used across the training runs are summarized in @hyperparams. The same values are used for every cell of the experimental matrix; the only variables are the two boolean flags `legal` and `masked`.

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
  caption: [Hyperparameters held constant across all five configurations.],
) <hyperparams>

// ========================================== // CHAPTER 5: Experimental Design // ==========================================
= Experimental Design
To ensure that observed differences in learning dynamics are attributable solely to the experimental variables, a *2 x 2 experimental matrix* (plus one ablation control) is employed. All configurations share an identical transformer architecture, optimizer settings, and MCTS simulation counts, as specified in @hyperparams.

== Axis 1: Environmental Constraints ($E$)
This axis evaluates how the complexity of the Reward Function $R(s, a)$ affects the value head’s ability to internalize high-level survival heuristics (e.g., "don't move into a pin").
- *Legal Configuration ($E_L$):*
  The environment utilizes standard FIDE rules. The network never explores states where a king is captured. The reward signal is sparse, occurring only at checkmate or draws. This forces the value head to learn the abstract concept of "check" and "legality" as a hard boundary provided by the engine.
- *Pseudo-Legal Configuration ($E_P$):*
  The environment allows any geometrically valid move, including those that leave the king in check. The game ends only when a king is physically captured ($K_"cap"$).
- *Purpose:* To test if the value head can autonomously learn "king safety" as a survival heuristic. In $E_P$, a "pin" is not a legal restriction but a state with a high probability of immediate value collapse (losing the king).

== Axis 2: Action Masking ($M$)
This axis isolates the policy head’s acquisition of piece kinematics (the "physical" rules of the board).
- *Masked Configuration ($M_("mask")$):*
  The policy logits for illegal squares are clamped to $-1^8$ before the softmax. The gradient for these indices is zeroed. The network is effectively "blindfolded" to the existence of illegal moves, focusing purely on choosing the best move among legal options.
- *Unmasked Configuration ($M_("unmask")$):*
  The network generates a distribution over all 64 squares. While the MCTS orchestrator still filters these for the simulation, the loss function penalizes the network for any probability mass placed on illegal squares.
- *Purpose:* To observe if the network can internalize the "geometry" of chess (e.g., Knights move in L-shapes) as a representational prior without hardcoded guardrails.

== Experimental Configurations
The intersection of these axes creates four primary experimental groups and one ablation group:

#figure(
  table(
    columns: (auto, 1fr, 1fr, 1fr),
    inset: 10pt,
    align: horizon,
    table.header([*Configuration ID*], [*Ruleset ($E$)*], [*Masking ($M$)*], [*Loss Curriculum*]),
    [*Control* (AlphaZero-lite)], [Legal], [Masked], [Fixed (1:1)],
    [*Kinematic Learner*], [Legal], [Unmasked], [Self-Annealing],
    [*Heuristic Learner*], [Pseudo-Legal], [Masked], [Fixed (1:1)],
    [*The "Tabula Rasa"*], [Pseudo-Legal], [Unmasked], [Self-Annealing],
    [*Ablation* (Fixed-Ratio)], [Legal], [Unmasked], [Fixed (1:1)],
  ),
  caption: [The $2 times 2$ experimental matrix with ablation control.],
) <config_matrix>

=== The "Tabula Rasa" Challenge
The *Pseudo-Legal + Unmasked* configuration represents the most difficult learning environment. The network begins with zero knowledge of how pieces move (Axis 2) and no environmental protection against losing its King (Axis 1). Success in this regime suggests that the transformer architecture can autonomously construct a world model of the game’s causal physics.

=== The Self-Annealing Ablation
To isolate the efficacy of the *Self-Annealing Loss* ($lambda$) defined in @self-annealing, the *Ablation (Fixed-Ratio)* configuration is used.
- In the *Kinematic Learner*, the network prioritizes policy loss ($L_("policy")$) when illegal move rates are high.
- In the *Ablation*, the network is penalized for illegal moves but maintains a constant 1:1 ratio between policy and value loss regardless of how many illegal moves it proposes.
- *Prediction:* The self-annealing configuration will show a faster "phase transition" from random play to rule-abiding play by prioritizing policy evaluation (piece kinematics) over value evaluation when policy distribution is uniform. // something about knowing how pieces move might be pre requisite to value evaluation

== Evaluation Procedure
Each configuration is trained under a fixed time limit. During training, the following metrics are recorded:
+ *Rule Convergence ($lambda$):* The rate at which the unmasked models stop proposing illegal moves.
+ *Strategic Depth:* Average game length, Wins / Draws, Nodes expanded, Loss, Games started, ACPL against Stockfish.
+ *Move‑accuracy (ACPL):* Average centipawn loss measured against Stockfish eval. _Note:_ ACPL logging was added approximately halfway through the training runs. Early‑phase comparisons are incomplete and trends should be interpreted with caution.

// ========================================== // CHAPTER 6: Results // ==========================================
// TODO
= Results
Metrics logged: iteration, games_started, avg_loss, avg_game_length, wins, draws, nodes_expanded, avg_illegal_prob, acpl.
ACPL logging was added partway (@design_challenges), so early data is incomplete.
*Unfortunately, all presented graphs are plagues with dips and spikes from the result of constant interruptions from people randomly rebooting lab machines as well as frequent, unnotified scheduled reboots by IT*

Due to frequent interruptions during training, much data had to be discarded, and data had to be concatenated from a restarted run. The cleared replay buffer shows a temporary regression in loss and policy quality; subsequent iterations show recovery trends consistent with relearning from a smaller buffer.
spikes are due to training being reloaded from a checkpoint. Although the model weights and optimizer state was checkpointed, the replay buffer was reset to 0 likely shocking model weights and causing the sharp spike in loss. The noam lr scheduler state was not recorded between runs, leading to a period of slower convergence after checkpoints. model checkpoints were similarly only saved every 25 iterations to conserve space, resulting in large regressions post reboots.

#let legal_masked = csv("output/legal_masked_metrics_fixed.csv")
#let legal_unmasked = csv("output/legal_unmasked_metrics_fixed.csv")
#let legal_unmasked_annealing = csv("output/legal_unmasked_metrics_annealing_fixed.csv") // without annealing
#let pseudo_masked = csv("output/pseudo_masked_metrics_fixed.csv")
#let pseudo_unmasked = csv("output/pseudo_unmasked_metrics_fixed.csv")

#let headers = legal_masked.first()
#let legal_masked_data = legal_masked.slice(1)
#let legal_unmasked_data = legal_unmasked.slice(1)
#let legal_unmasked_annealing_data = legal_unmasked_annealing.slice(1)
#let pseudo_masked_data = pseudo_masked.slice(1)
#let pseudo_unmasked_data = pseudo_unmasked.slice(1)
// iteration,games_started,avg_loss,avg_game_length,wins,draws,nodes_expanded,avg_illegal_prob,acpl
// ~ 300 rows

== Loss <loss>
#cetz.canvas({
  import cetz.draw: *
  import cetz-plot: *

  let loss-data(table) = table.map(row => (float(row.at(0)), float(row.at(2))))

  plot.plot(
    size: (12, 8),
    x-label: "Iteration",
    y-label: "Average Loss",
    x-tick-step: 25,
    y-grid: true,
    y-min: 0,
    y-max: 8,
    {
      plot.add(loss-data(legal_masked_data), label: "Legal Masked")
      plot.add(loss-data(legal_unmasked_data), label: "Legal Unmasked")
      plot.add(loss-data(legal_unmasked_annealing_data), label: "Legal Unmasked + Annealing")
      plot.add(loss-data(pseudo_masked_data), label: "Pseudo Masked")
      plot.add(loss-data(pseudo_unmasked_data), label: "Pseudo Unmasked")
    },
  )
})


loss for legal configurations naturally remains flat throughout training as the model is always chasing mcts search values, essentially a stronger version of itself. convergence would mean the model solves chess, unlikely within the limitations of this project. the ablation, legal unmasked + annealing experienced issues in configuration and as a result ran for less iterations. however the graph already shows a departure in learning dynamics - the loss follows a logistic curve downwards, as opposed to the asymptotic approach in both unmasked + annealing configurations. the spikes across the graph are the result of the many hiccups from training for multiple days on high demand lab machines.
 
== Game Length
#let game-length-data(table, alpha: 0.9) = {
  let clean-data = table.filter(row => float(row.at(3)) > 0)

  if clean-data.len() > 0 {
    let offset = float(clean-data.first().at(0))
    let first-y = float(clean-data.first().at(3))

    let result = clean-data.fold((first-y, ()), ((prev-ema, points), row) => {
      let current-y = float(row.at(3))
      let current-x = float(row.at(0)) - offset
      let new-ema = alpha * current-y + (1 - alpha) * prev-ema
      (new-ema, points + ((current-x, new-ema),))
    })

    result.at(1)
  } else {
    ()
  }
}

#cetz.canvas({
  import cetz.draw: *
  import cetz-plot: *

  plot.plot(
    size: (12, 6),
    x-label: "Iterations",
    y-label: "Average Game Length",
    y-min: 0,
    y-max: 275,
    y-grid: true,
    {
      plot.add(game-length-data(pseudo_masked_data), label: "Pseudo Masked")
      plot.add(game-length-data(pseudo_unmasked_data), label: "Pseudo Unmasked")
      plot.add(game-length-data(legal_unmasked_annealing_data), label: "Legal Unmasked ~Annealing")
      plot.add(game-length-data(legal_unmasked_data), label: "Legal Unmasked")
      plot.add(game-length-data(legal_masked_data), label: "Legal Masked")
    },
  )
})

These graphs is a testament to the efficacy of mcts. random walks would have produced much noisier graphs


_errata_: the legal unmasked without annealing loss configuration exhibited consistent below average game length until the reboot. this was due to an error in configuration where the same seed was applied across the batch resulting in identical playouts of the same game. after restarting from a checkpoint, this was rectified and average game length naturally approached the other legal configurations.

== Win / Draw Ratios
#let win-rate-data(table) = table.map(row => {
  let iter = float(row.at(0))
  let wins = float(row.at(3))
  let draws = float(row.at(4))
  let games = float(row.at(1))
  (iter, (wins / (wins + draws + 1)) * 100, (draws / (wins + draws + 1)) * 100)
})

#let plot-win-draw(data, title, x-axis: false, y-axis: true, draw_legend: false) = {
  cetz.canvas({
    import cetz.draw: *
    import cetz-plot: *

    let wd = win-rate-data(data)
    let iters = wd.map(x => x.at(0))
    let wins = wd.map(x => x.at(1))
    let draws = wd.map(x => x.at(2))

    plot.plot(
      size: (6, 4),
      x-label: if x-axis { "Iteration" } else { none },
      y-label: if y-axis { "Percentage (%)" } else { none },
      y-min: 0,
      y-max: 100,
      x-tick-step: 50,
      y-grid: true,
      {
        let win-args = (hypograph: true)
        let draw-args = (hypograph: true)
        if draw_legend {
          win-args.insert("label", "wins")
          draw-args.insert("label", "draws")
        }
        plot.add(
          wins.zip(iters).map(((w, i)) => (i, w)),
          ..win-args,
        )
        plot.add(
          draws.zip(iters).map(((d, i)) => (i, d)),
          ..draw-args,
        )
      },
    )
  })
}

#grid(
  columns: (1fr, 1fr),
  row-gutter: 1em,
  column-gutter: 1em,

  [
    #text(weight: "bold", size: 0.8em)[Legal Masked]
    #plot-win-draw(legal_masked_data, "Legal Masked", x-axis: false, y-axis: false)
  ],
  [
    #text(weight: "bold", size: 0.8em)[Legal Unmasked]
    #plot-win-draw(legal_unmasked_data, "Legal Unmasked", x-axis: false, y-axis: false)
  ],

  [
    #text(weight: "bold", size: 0.8em)[Legal Unmasked ~Annealing]
    #plot-win-draw(legal_unmasked_annealing_data, "Legal Unm + Annealing", x-axis: false, y-axis: false)
  ],
  [
    #text(weight: "bold", size: 0.8em)[Pseudo Masked]
    #plot-win-draw(pseudo_masked_data, "Pseudo Masked", x-axis: false, y-axis: false)
  ],

  [
    #text(weight: "bold", size: 0.8em)[Pseudo Unmasked]
    #plot-win-draw(pseudo_unmasked_data, "Pseudo Unmasked", x-axis: false, y-axis: false, draw_legend: true)
  ],
)

Draws are preferred, as a game ending in checkmate or king capture is often the result of a significant blunder (a major tactical oversight). Seeing loss overall is far from convergence (@loss), and the draw/win ratio is high for pseudo legal games, the model seems yet to learn $A_L$. 
Legal games exhibit much healthier win/draw ratios due to checkmate being much harder to create, even with random walks. the exception of legal masked play which abnormally found many checkmates - possibly due to a configuration error, more runs need to be carried out to verify its validity.

Pseudo-legal games predominantly ended in wins (king captures). The average game length for pseudolegal games average at 60, meaning the mcts is able to postpone king capture until end games, the search depth is likely unable to navigate the complex positions without the legal guardrails in place to avoid king capture and ends arriving at terminal nodes involving king capture before draw conditions can be met. 

== Illegal Move Rates
#let illegal-data(table) = table.map(row => (float(row.at(0)), float(row.at(7))))

#cetz.canvas({
  import cetz.draw: *
  import cetz-plot: *

  plot.plot(
    size: (12, 6),
    x-label: "Iteration",
    y-label: "Average Illegal Move Probability",
    y-min: 0,
    y-max: 1,
    x-tick-step: 25,
    y-grid: true,
    {
      plot.add(illegal-data(pseudo_unmasked_data), label: "Pseudo-Legal Unmasked")
      plot.add(illegal-data(legal_unmasked_data), label: "Legal Unmasked")
      plot.add(illegal-data(legal_unmasked_annealing_data), label: "Legal Unmasked+Annealing")
    },
  )
})


== ACPL
#let acpl-data(table, alpha: 0.2) = {
  let clean-data = table.filter(row => float(row.at(8)) > 0)

  if clean-data.len() > 0 {
    let offset = float(clean-data.first().at(0))
    let first-y = float(clean-data.first().at(8))

    let result = clean-data.fold((first-y, ()), ((prev-ema, points), row) => {
      let current-y = float(row.at(8))
      let current-x = float(row.at(0)) - offset
      let new-ema = alpha * current-y + (1 - alpha) * prev-ema
      (new-ema, points + ((current-x, new-ema),))
    })

    result.at(1)
  } else {
    ()
  }
}

#cetz.canvas({
  import cetz.draw: *
  import cetz-plot: *

  plot.plot(
    size: (12, 6),
    x-label: "Iterations since Tracking Started",
    y-label: "ACPL (EMA)",
    y-min: 0,
    x-min: 0,
    y-grid: true,
    {
      plot.add(acpl-data(pseudo_masked_data), label: "Pseudo Masked")
      plot.add(acpl-data(pseudo_unmasked_data), label: "Pseudo Unmasked")
      plot.add(acpl-data(legal_unmasked_annealing_data), label: "Legal Unmasked ~Annealing")
      plot.add(acpl-data(legal_unmasked_data), label: "Legal Unmasked")
      plot.add(acpl-data(legal_masked_data), label: "Legal Masked")
    },
  )
})

These results are likely meaningless as the model is nowhere near convergence. However the tracking is functional. Smoothing is 0.2, N is 9. EMA is applied to smooth inherently noisy highly fluctuating ACPL calculations.
Legal games tend to have high ACPL than pseudo legal games, as the games tend to last much longer approx 240 moves, where end games are sharp but terminal conditions more easily avoidable via search due to the guardrails present in legal play. Pseudo legal games end much sooner resulting in more dubious early to mid games followed by a sharp terminal state where dubious early play align with drawish evals resulting in overall lower acpl scores.

== Ruleset Convergence Analysis
== Masking and Grokking Analysis

This project was significantly severely strained by compute!!!

// ========================================== // CHAPTER 7: CONCLUSION // ==========================================
// TODO
= Conclusion
== Project Evaluation
== Limitations and Future Work
- *Compute Constraints:* Hardware limitations restricted the total number of training epochs, preventing observation of deep late-stage grokking.
- *Move Generation:* Future iterations should replace standard bitboard raycasting with Magic Bitboards for optimal
performance.
- *Policy Bootstrapping:* Boot strap the policy head with uniform distributions of valid moves, before self play.
- *Value Bootstrapping:* Boot strap the value head with a robust suite of perfect knowledge (mate evals, endgame tablebases, certain stockfish evaluations).

// ========================================== // CHAPTER 6: Ethics // ==========================================
= Statement of Ethics
== Legal
The project is publicly available under the GNU General Public License v3. The repository includes a UPX-compressed, statically linked Stockfish binary (@Stockfish_developers_Stockfish, GPLv3) and a filtered subset of 100 000 mate-in-N positions from the Lichess open database (@lichess_db, CC0); both components are credited in the repository. All Rust crate dependencies carry permissive licences compatible with GPLv3. No primary data involving human or animal subjects were collected. The work was executed on University of Surrey laboratory machines; no unauthorised system access occurred and all activities comply with the Computer Misuse Act. No derivative work beyond that explicitly cited is contained.

== Legal
The project is publicly available under the GNU General Public License v3. The repository includes a UPX-compressed, statically linked Stockfish binary (GPLv3) and a filtered subset of 100 000 mate-in-N positions from the Lichess open database (CC0); both components are credited in the repository. All Rust crate dependencies carry permissive licences compatible with GPLv3. No primary data involving human or animal subjects were collected. The work was executed on University of Surrey laboratory machines; no unauthorised system access occurred and all activities comply with the Computer Misuse Act. These measures support SDG 16 (Peace, Justice and Strong Institutions) by making the development process transparent, auditable, and legally traceable.

== Social
Full open-source release of the training pipeline and engine promotes freedom of information, fostering reproducible research and public education in machine learning and reinforcement learning, an expression of SDG 4 (Quality Education). The system cross-compiles to all major platforms and is hardware-agnostic; the novel factored transformer and bipartite MCTS architecture advance state-of-the‑art efficient self-play, aligning with SDG 9 (Industry, Innovation and Infrastructure). Chess engines already exceed human grandmaster strength and this work adds negligible marginal advantage; its contribution to competitive imbalance is negligible. The command-line interface is not accessible to visually impaired users; this limitation is acknowledged and could be addressed in future work through alternative output modalities, though the primary audience is researchers interacting programmatically.

== Ethical
The tool operates entirely offline, collects no personally identifiable information, and functions as a standalone CLI application. The system’s ability to internalize the rules of chess without hard-coded legality masks raises broader dual-use considerations beyond online cheating: autonomous decision-making agents that learn environmental constraints from scratch may exhibit emergent behaviours that are difficult to predict or constrain, with potential implications for safety‑critical applications. Extended training on 5 NVIDIA RTX 4000 Ada GPUs consumed approximately ($5 "GPUs" times 0.13 "Kw" times 120h approx) 78 "kWh"$ over all runs. Rust’s zero-cost abstractions and lack of a garbage-collector reduce per-operation energy usage compared to equivalent Python pipelines, partially mitigating the carbon footprint. Addressing SDG 13 (Climate Action), future work should explore training-time energy monitoring and dynamic resource scaling. The availability of far simpler, stronger engines renders misuse for cheating impractical, but the learning dynamics documented here could inform the design of more transparent and auditable rule-acquisition systems.

== Professional
The work was conducted in accordance with the BCS Code of Conduct, upholding public good through open dissemination, professional competence via rigorous testing and reproducible hyperparameters, and integrity by isolating experimental variables and avoiding over-claimed results. The exclusive use of Rust guarantees memory safety and data-race freedom at compile time, eliminating entire classes of runtime errors. Engineering choices such as seeded RNGs, Nix Flakes and Cargo locks ensure reproducibility, reliability, and long-term maintainability. These practices reflect professional standards of transparent and robust software engineering, and they directly embody the BCS principle of “making IT for everyone” by lowering the barrier to auditing and extending the system. The complete specification of all hyperparameters, the move generator, and the public repository all contribute to a trustworthy, peer-reviewable artefact, furthering SDG 16’s aim of accountable institutions.

// ========================================== // BIBLIOGRAPHY // ==========================================
#bibliography("refs.bib", style: "harvard-cite-them-right")

