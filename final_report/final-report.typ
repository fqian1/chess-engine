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
Modern machine learning often relies on scaling data and compute over opaque architectures. However, these models are often blind to the "rules" of the environment, aided by *Logit Masking* - a heuristic that artificially zeroes out the probability of illegal actions prior to softmax activation. While efficient, this deprives the network of feedback for invalid predictions, bypassing the models need to internally represent the physical constraints of the environment. Furthermore, standard RL agents are protected against making bad decisions through the rules of the environment. This prevents observation of how internal action space pruning to match a more constrained environment before developing strategy influences learning. Chess provides an optimal substrate for this investigation: it presents highly ambiguous intermediate states bound by objective ground truths and a mathematically perfect, yet computationally intractable, solution space.

== Project Description
Existing literature rarely assesses the impact of these environmental guardrails on the learning dynamics of transformer-based agents. This project deconstructs these dynamics by investigating two orthogonal axes of environmental constraint: Action Space Constraints and Logit Masking. 

To facilitate explicit measurement of model interpretability, a custom two-pass autoregressive transformer encoder and a bipartite Monte Carlo Tree Search (MCTS) are introduced. This architecture decouples piece selection from destination selection, providing a granular window into the model's spatial reasoning.

The experimental configurations are defined across two axes:
+ *Axis 1: Rule Set Convergence (Action Space Constraints)*
  - *Control:* Training on a strict legal ruleset.
  - *Test:* Training on a pseudo-legal ruleset where explicit king capture is permitted and terminates the game.
  - *Hypothesis:* The network is first bootstrapped learning the rules and goal of the environment. Optimal play in a pseudo-legal environment will naturally converge to legal play. Concepts such as pins and checkmates will manifest not as hardcoded rules, but as emergent survival heuristics to secure or prevent king capture.

+ *Axis 2: Logit Masking*
  - *Control:* Masked logits (illegal moves are mathematically eliminated prior to selection).
  - *Test:* Unmasked logits (illegal moves are permitted by the network but penalized by the environment/loss function).
  - *Hypothesis:* Unmasked training will incur an initial sample-efficiency penalty but will force the network to internally map the physical mechanics of the board, yielding more robust and generalizable long-term representations.

== Aims and Objectives
The primary aim of this project is to train multiple transformer-based agents via self-play reinforcement learning across varying rule sets and action spaces, quantitatively evaluating their performance, learning trajectories, and emergent behaviors.

Specific objectives include:
- Develop a bespoke, high-performance Rust chess engine supporting both legal and pseudo-legal state generation and validation.
- Architect and implement a two-pass autoregressive transformer and bipartite MCTS to explicitly model piece-then-square action selection.
- Train four distinct model configurations via self-play: (Legal, Masked), (Legal, Unmasked), (Pseudo-Legal, Masked), and (Pseudo-Legal, Unmasked).
- Quantitatively evaluate the convergence rates, illegal move proposal frequencies, and the emergence of rule-abiding play across the unconstrained configurations.

== Thesis Structure
The remainder of this dissertation is structured as follows: 
- *Chapter 2* reviews the literature surrounding reinforcement learning, transformer architectures, and action masking. 
- *Chapter 3* details the system design, including the bipartite MCTS and two-pass encoder architecture. 
- *Chapter 4* outlines the implementation details of the Rust engine and training pipeline.
- *Chapter 5* presents the results and evaluation of the four experimental configurations.
- *Chapter 6* concludes the project and discusses avenues for future work.

// ========================================== // CHAPTER 2: LITERATURE REVIEW // ==========================================
// Grokking: Generalization beyond overfitting on small algorithmic datasets
// power2022grokking

// XXII. Programming a computer for playing chess (Shannon)
// Shannon01031950

// An overview of the action space for deep reinforcement learning
// zhu2021overview

// Mastering Chess and Shogi by Self-Play with a General Reinforcement Learning Algorithm (Alphazero)
// silver2017masteringchessshogiselfplay

// Mastering Atari, Go, chess and shogi by planning with a learned model (Muzero)
// Schrittwieser_2020

// A Closer Look at Invalid Action Masking in Policy Gradient Algorithms 
// DBLP:journals/corr/abs-2006-14171

// Mastering Chess with a Transformer Model (2024)
// monroe2026chessformer

// Evidence of Learned Look-Ahead in a Chess-Playing Neural Network (2024)
// jenner2024evidencelearnedlookaheadchessplaying

// Acquisition of Chess Knowledge in AlphaZero (2021)
// DBLP:journals/corr/abs-2111-09259

// Giraffe: Using Deep Reinforcement Learning to Play Chess (2015)
// DBLP:journals/corr/Lai15a

// Curriculum learning
// 10.1145/1553374.1553380

// Action Space Shaping in Deep Reinforcement Learning
// 9231687

= Literature Review
== Chess as a MDP
Chess is a deterministic, zero-sum, perfect-information game. It is formally modeled as a Markov Decision Process (MDP) defined by a tuple $(S, A, T, R)$, where $S$ is the state space ($|S| approx 10^{40}$), $A$ is the action space, $T: S times A -> S$ is the deterministic transition function, and $R: S times A -> {-1, 0, 1}$ is the reward function @Shannon01031950.

In standard reinforcement learning (RL) implementations, the environment strictly enforces a legal action space $A_L(s) subset A$, where $A_L$ contains only moves that comply with the rules of chess (e.g., preventing self-check) @fide2023laws. This project introduces a relaxed pseudo-legal action space $A_P(s)$, defined as the set of all geometrically valid piece movements regardless of king safety. Under $A_P$, the termination condition is shifted from the abstract concept of "checkmate" to the explicit physical capture of the King ($K_"cap"$).

For any state $s$, an optimal policy $pi^*$ trained in $A_P$ must implicitly learn to prune the action space, such that:
$ pi^*(a | s) -> 0 quad "for all" a in (A_P(s) backslash A_L(s)) $
In this paradigm, concepts such as pins and checkmates are not hardcoded environmental constraints, but emergent survival heuristics within the value landscape.

== Chess played by Machines
=== AlphaZero, MuZero
The paradigm of self-play RL in chess was defined by AlphaZero @silver2017masteringchessshogiselfplay, which combined Monte Carlo Tree Search (MCTS) with deep convolutional neural networks (CNNs) for policy and value evaluation. However, AlphaZero relies entirely on a hardcoded environment to generate $A_L(s)$, bypassing the need for the network to internalize the rules of the game. 

Subsequent architectures, such as MuZero @Schrittwieser_2020, removed the need for a known transition dynamics model by learning a latent environment simulator. While MuZero demonstrates the ability to adhere to environment constraints implicitly within its hidden state transitions, its policy head still simulates within the legal action space during MCTS rollouts; the literature lacks insight into how the removal of these environmental guardrails impacts the sample efficiency and representational robustness of the underlying network.
// muzero internalises the rules in order to play in latent space, which means even with masking, muzero learns the (legal) action space (but pseudo legal not tested)

=== ChessFormer
Recent advancements have demonstrated the efficacy of Transformer architectures in chess, shifting away from the spatial inductive biases of CNNs. #cite(label("monroe2024masteringchesstransformermodel")) demonstrated that Transformers can achieve Grandmaster-level performance without explicit search, relying entirely on attention mechanisms to evaluate board states. Furthermore, #cite(label("jenner2024evidencelearnedlookaheadchessplaying")) provided evidence of learned look-ahead within chess-playing neural networks, showing that attention heads naturally encode future board states.

Standard chess engines flatten the action space into a single discrete distribution (e.g., 4,672 possible moves). To explicitly measure spatial reasoning and rule internalization, this project utilizes a custom two-pass autoregressive Transformer encoder. By decoupling piece selection $P("from" | s)$ and destination selection $P("to" | s, "from")$, the architecture forces the attention mechanism to explicitly map the geometric constraints of the selected piece, providing a granular window into the model's internal representation of board topology.

== Invalid Action Masking
In policy gradient algorithms, it is standard practice to prevent the selection of illegal actions via Invalid Action Masking (Logit Masking). Masking applies a transformation to the policy logits $z$ prior to the softmax activation:

$ P(a_i | s) = exp(z_i + M_i) / (sum_j exp(z_j + M_j)) $

where $M_i = -infinity$ if $a_i limits(in.not)A_L(s)$, and $0$ otherwise. 

While usually done on intuition, and to ease implementation, #cite(label("DBLP:journals/corr/abs-2006-14171")) demonstrated that invalid action masking significantly improves sample efficiency and asymptotic performance in environments with large, state-dependent action spaces. @9231687 similarly emphasised the importance of action space transformations to create learnable environments, especially for continuous action spaces. However, masking renders the network "blind" to the rules; the invalidity of an action is treated as an external mathematical absolute rather than a learned feature.

== Curriculum learning
#cite(label("10.1145/1553374.1553380")) showed the importance of incremental learning... can i say something here? i want to say training on pseudo legal and unmasked is incremental learning like, learn how pieces move, then learn to capture the king, then learn to checkmate and keep own king safe. my mcts implementation seems to shortcut this in a detrimental way however...

== Learning Dynamics and Grokking
The requirement for an unmasked network to learn the rules of chess from scratch introduces complex learning dynamics. #cite(label("power2022grokking")) identified the phenomenon of "grokking," where neural networks trained on small algorithmic datasets exhibit delayed generalization long after overfitting the training data. This was possible because of weight decay pressure, illustrating the importance of weight regularisation.

== Summary and Research Gap
While the efficacy of action masking is well-documented in general RL #cite(label("DBLP:journals/corr/abs-2006-14171")), and Transformers have proven capable of modeling chess @jenner2024evidencelearnedlookaheadchessplaying, the intersection of these domains remains unexplored. Existing literature does not isolate the impact of environmental guardrails on the representational quality of Transformer-based MCTS agents. 

In the context of unmasked, pseudo-legal chess training, the network faces a dual-optimization problem: it must learn the physical constraints of the board (the rules) and the strategic evaluation of states (the game). It is hypothesized that the network will exhibit a grokking-like phase transition: an initial period of high illegal-move proposal (random walk), followed by a sharp convergence toward $A_L(s)$ as the physical constraints are internalized, preceding the development of high-level strategy.

This project addresses this gap by training four distinct configurations across two orthogonal axes: Action Space Constraints (Legal vs. Pseudo-Legal) and Logit Masking (Masked vs. Unmasked). By utilizing a bespoke two-pass autoregressive encoder, this research provides a novel, quantitative analysis of how rule internalization and action space pruning manifest within the attention layers of a chess-playing agent.

// ========================================== // CHAPTER 3: DESIGN // ==========================================
= System Design and Methodology
The code for this project contains 3 main components. 


== Chess Library
A bespoke client was developed in Rust to meet the high-performance demands of MCTS rollouts and ensure memory safety during concurrent self-play.

=== Move Generation
Ray casting is optimized using constant directional masks and pre-computed "between" masks, allowing for $O(1)$ validation of sliding piece moves.

== Model Architecture
A 2 pass autoregressive encoder was chosen as a means to expose model interpretability in a measurable way. The same policy head is re used to generate both pieces to select from in the first pass, then squares to move to given an extra input plane in the first pass one hot encoded with the selected piece. The model is given a $64 times 14$ tensor representing the 12 chessboard bitboards, the en passant square, and the optional selected square.
[cls] token

=== Inputs and Outputs
- *Inputs:* 12 piece bitboards, 1 en passant bitboard, 1 selected square bitboard, castling rights (4x1 f32), 50-move rule scalar (1 f32).
- *Outputs:* Policy head (64-square distribution) and Value head (Win/Draw/Loss buckets). Both utilize softmax activation and KL divergence loss.

=== Two-Pass Execution Flow
1. *Pass 1:* Select origin square.
2. *Pass 2:* Select destination square (Pass 1 output fed back into input). Promotions are handled automatically during MCTS rollouts.

== Monte Carlo Tree Search
A custom bipartite mcts search algorithm is architected to support the 2 pass encoder. The MCTS utilizes the PUCT (Predictor + Upper Confidence Bound applied to Trees) algorithm to balance exploration and exploitation. Dirichlet noise is added at the root node to encourage exploration of other lines.

$
  alpha(iota, x_2) = sum_(x_1, d_(iota-1)) alpha(iota - 1, x_1) op("Pr") { bold(r)_(n(iota-1)+x_1, n iota + x_2), 
bold(t)_(iota-1) }
$

== Self-Play Pipeline
Multiple games are vectorised and initialised with their own monte carlo tree search. Each simulation occurs independently of each other and in parallel via Rayon. The network is updated via backpropagation using autograd, minimizing the combined policy and value loss.

// ========================================== // CHAPTER 4: Implementation // ==========================================
// how?
= Implementation Details
== The Chess Client
=== ChessSquare
=== Bitboard
The board is represented using 12 primary bitboards (unsigned 64-bit integers), packing the state into 96 bytes. Additional convenience bitboards (occupancy masks) are maintained to accelerate move generation. Bitboards enable high-performance move generation via bitwise intrinsics. In deep learning, these are expanded into $64 times 14$ tensors.
=== ChessBoard
=== ChessPosition
=== ChessGame
```rust
pub struct ChessGame {
    pub castling_rights: CastlingRights,
    pub chessboard: ChessBoard,
    pub en_passant: Option<ChessSquare>,
    pub fullmove_counter: u32,
    pub game_history: alloc::vec::Vec<GameStateEntry>,
    pub halfmove_clock: u32,
    pub outcome: Outcome,
    pub rule_set: RuleSet,
    pub side_to_move: chess_piece::Color,
    pub zobrist_hash: u64,
}
```
== The Model
=== Transformer
=== Data

== The Monte Carlo Tree Search
=== Nodes
==== Select
==== Move
=== Edges
=== Arenas
=== The Orchestrator

== Main()

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
- *Compute Constraints:* Hardware limitations restricted the total number of training epochs, preventing observation of 
deep late-stage grokking.
- *Move Generation:* Future iterations should replace standard bitboard raycasting with Magic Bitboards for optimal 
performance.
- *Endgame Tablebases (EGTB):* Injecting perfect knowledge at varying stages of the training pipeline remains a highly 
viable area for future research to bootstrap the value head and sharpen endgame play.

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
