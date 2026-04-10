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
    )[An Investigation into Action Space Constraints in Reinforcement Learning]

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

// ========================================== // PAGE 4 (Numbered 3): ABSTRACT // ==========================================
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

== Context and Motivation
Modern machine learning often relies on scaling data and compute over opaque architectures. This project deconstructs 
the learning dynamics of transformer models by introducing a novel architecture and evaluating emergent behaviors 
within unrestricted action spaces. Chess serves as the ideal environment: it presents highly ambiguous intermediate 
states bound by objective ground truths and a mathematically perfect, yet computationally intractable, solution space.

This research investigates four primary areas:
1. The convergence of self-play in a pseudo-legal environment toward strict legal play.
2. The impact of logit masking on learning dynamics, specifically regarding the "grokking" of physical board mechanics.
3. The efficacy of a two-pass autoregressive encoder in reducing the action space within a Monte Carlo Tree Search 
(MCTS) framework.

== Aims and Objectives
The primary aim is to train multiple transformer-based agents via self-play reinforcement learning across varying rule 
sets (legal vs. pseudo-legal) and action spaces (masked vs. unmasked logits), evaluating their performance and learning 
trajectories.

Specific objectives include:
- Developing a high-performance, bespoke chess client in Rust supporting both pseudo-legal and legal state generation.
- Implementing a custom move generator utilizing bitboards, bitwise intrinsics, and Zobrist hashing.
- Architecting a two-pass autoregressive transformer encoder tightly integrated with the engine.
- Constructing a bipartite MCTS algorithm for self-play data generation.
- Training four model variants and capturing leading metrics (policy entropy, illegal move probability, estimated ELO).

== Risk Management
The primary risk is scope creep, specifically regarding the bespoke Rust client implementation. This is mitigated 
through strict modularity, rigorous unit testing, and limiting the scope of the engine to the requirements of the 
neural network (e.g., deferring Endgame Tablebase integration to future work).

// ========================================== // CHAPTER 2: LITERATURE REVIEW // ==========================================
= Literature Review

== Chess Fundamentals
=== Game Rules and State Space
Chess is a zero-sum, perfect-information game with a state-space complexity of $10^40$ and a game-tree complexity of 
$10^120$ (Shannon Number). Traditionally, engines operate under strict legal constraints. This investigation introduces 
"pseudo-legal" environments—ignoring king safety constraints—to observe if transformers implicitly learn higher-order 
rules (pins, checks) via punishment propagation.

=== Board Representation and Move Generation
Representation has evolved from piece-lists to bitboards (64-bit integers). Bitboards enable high-performance move 
generation via bitwise intrinsics. In deep learning, these are expanded into $8 times 8 times N$ tensors to suit the 
inductive biases of the network.

== Machine Learning in Chess
=== Reinforcement Learning and MCTS
Review of AlphaZero's integration of MCTS with policy/value networks @silver2017masteringchessshogiselfplay.
=== Transformers and Action Spaces
Review of recent applications of attention mechanisms in chess @monroe2024masteringchesstransformermodel.
=== Grokking and Masking
Analysis of delayed generalization ("grokking") @power2022grokking and the standard practice of logit masking in 
AlphaZero-style architectures.

// ========================================== // CHAPTER 3: METHODOLOGY & HYPOTHESES // ==========================================
= Methodology and Hypotheses

== H1: Ruleset Convergence (Pseudo-Legal vs. Legal) - *Control:* Training on a strict legal ruleset.
- *Test:* Training on a pseudo-legal ruleset (explicit king capture terminates the game).
- *Hypothesis:* Pseudo-legal training naturally converges to legal play. Checkmates and pins will manifest as emergent 
behaviors to secure or prevent king capture.

== H2: Logit Masking vs. Punishment Propagation
- *Control:* Masking illegal moves prior to softmax (network is blind to rules).
- *Test:* Unmasked logits. Illegal moves are punished via the loss function.
- *Hypothesis:* Unmasked training will initially learn slower but forces the network to internally map ("grok") the 
physical mechanics of the board, potentially yielding more robust long-term representations.

== H3: Two-Pass Autoregressive Encoder
- *Hypothesis:* Decomposing the action space into a two-pass selection (origin square, then destination square) 
provides a more human-like decision-making approach. While this doubles MCTS tree depth, it significantly reduces the 
model's immediate action space complexity.

// ========================================== // CHAPTER 4: SYSTEM ARCHITECTURE // ==========================================
= System Architecture and Implementation

== Client Architecture (Rust)
A bespoke client was developed in Rust to meet the high-performance demands of MCTS rollouts and ensure memory safety 
during concurrent self-play.

=== Bitboard Implementation
The board is represented using 12 primary bitboards (unsigned 64-bit integers), packing the state into 96 bytes. 
Additional convenience bitboards (occupancy masks) are maintained to accelerate move generation.

=== Move Generation
Ray casting is optimized using constant directional masks and pre-computed "between" masks, allowing for $O(1)$ 
validation of sliding piece moves.

```rust
// Core API Structure
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

== Model Architecture
=== Inputs and Outputs
- *Inputs:* 12 piece bitboards, 1 en passant bitboard, 1 selected square bitboard, castling rights (4x1 f32), 50-move 
rule scalar (1 f32).
- *Outputs:* Policy head (64-square distribution) and Value head (Win/Draw/Loss buckets). Both utilize softmax 
activation and KL divergence loss.

=== Two-Pass Execution Flow
1. *Pass 1:* Select origin square.
2. *Pass 2:* Select destination square (Pass 1 output fed back into input). Promotions are handled automatically during 
MCTS rollouts.

// ========================================== // CHAPTER 5: TRAINING & MCTS // ==========================================
= Training and Self-Play

== Monte Carlo Tree Search
The MCTS utilizes the PUCT (Predictor + Upper Confidence Bound applied to Trees) algorithm to balance exploration and 
exploitation.

$
  alpha(iota, x_2) = sum_(x_1, d_(iota-1)) alpha(iota - 1, x_1) op("Pr") { bold(r)_(n(iota-1)+x_1, n iota + x_2), 
bold(t)_(iota-1) }
$

== Self-Play Pipeline
Data is generated asynchronously using Rayon. The network is updated via backpropagation using autograd, minimizing the 
combined policy and value loss.

// ========================================== // CHAPTER 6: RESULTS & EVALUATION // ==========================================
= Results and Evaluation

== Experimental Setup
To ensure fairness, all variants were initialized with identical seeds, trained for the same number of epochs, and 
evaluated at identical MCTS search depths.

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

== H1: Ruleset Convergence Analysis
[Insert analysis of pseudo-legal vs legal performance]

== H2: Masking and Grokking Analysis
[Insert analysis of policy entropy and illegal move probability over time]

// ========================================== // CHAPTER 7: CONCLUSION // ==========================================
= Conclusion and Future Work

== Summary of Findings
This project successfully implemented a bespoke Rust chess engine and a novel two-pass transformer architecture. 
Empirical validation of the four experimental branches provided insights into [insert core finding regarding 
masking/rulesets].

== Limitations and Future Work
- *Compute Constraints:* Hardware limitations restricted the total number of training epochs, preventing observation of 
deep late-stage grokking.
- *Move Generation:* Future iterations should replace standard bitboard raycasting with Magic Bitboards for optimal 
performance.
- *Endgame Tablebases (EGTB):* Injecting perfect knowledge at varying stages of the training pipeline remains a highly 
viable area for future research to bootstrap the value head and sharpen endgame play.


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
#bibliography("refs.bib")
