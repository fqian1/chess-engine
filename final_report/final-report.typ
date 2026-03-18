// ==========================================
// DOCUMENT SETUP
// ==========================================
#set page(
  paper: "a4",
  margin: (x: 3cm, y: 3cm),
  numbering: "1",
)

#set text(
  font: "New Computer Modern",
  size: 11pt,
  lang: "en"
)

#set par(
  justify: true,
  leading: 0.65em,
)

// ==========================================
// CUSTOM FUNCTIONS
// ==========================================

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

// ==========================================
// PAGE 1: TITLE PAGE
// ==========================================
#page(numbering: none)[
  #align(center)[
    #v(2fr)
    #text(size: 20pt)[An Investigation into Action Space Constraints in Reinforcement Learning]

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

// ==========================================
// PAGE 2: DECLARATION
// ==========================================
#page(numbering: none)[
  // Removed v(1fr) to match PDF top alignment
  #v(2cm)
  I declare that this dissertation is my own work and that the work of others is acknowledged and indicated by explicit references.

  #v(1cm)
  Francois Qian \
  May 2026
  #v(1fr)
]

// ==========================================
// PAGE 3: COPYRIGHT
// ==========================================
#page(numbering: none)[
  #align(center + horizon)[
    #sym.copyright Copyright Francois Qian, May 2026
  ]
]

// ==========================================
// PAGE 4 (Numbered 3): ABSTRACT
// ==========================================
#set page(numbering: "1")
#counter(page).update(3)

#front-heading("Abstract")

//TODO! do this at the end

// ==========================================
// PAGE 5: ACKNOWLEDGEMENTS
// ==========================================
#front-heading("Acknowledgements")

I would like to thank my supervisor, Nishanth Sastry, for overseeing the project.

// ==========================================
// PAGE 6: CONTENTS
// ==========================================
#show outline.entry.where(
  level: 1
): it => {
  v(12pt, weak: true)
  strong(it)
}

#outline(
  title: front-heading("Contents"),
  indent: auto,
  depth: 3
)

// ==========================================
// PAGE 7: LIST OF FIGURES
// ==========================================
#outline(
  title: front-heading("List of Figures"),
  target: figure.where(kind: image),
)

// ==========================================
// PAGE 8: LIST OF TABLES
// ==========================================
#outline(
  title: front-heading("List of Tables"),
  target: figure.where(kind: table),
)

// ==========================================
// PAGE 9: GLOSSARY
// ==========================================
#front-heading("Glossary")

#grid(
  columns: (1.5cm, 1fr),
  row-gutter: 1.5em,
  column-gutter: 1em,

  [$M C T S$], [Monte Carlo Search Tree, a heuristic search algorithm],
  [$P o l i c y$], [The $i^("th")$ symbol in the source message, where $a_i in S_m$],
  [$B_g$], [The decoded message, being a sequence of $g$ source symbols $b_1 b_2 ... b_g$],
  [$b_i$], [The $i^("th")$ symbol in the decoded message, where $b_i in S_m$],
  [$C_h$], [The transmitted (compressed) message, being a sequence of $h$ Tunstall codewords $c_1 c_2 ... c_h$],
  [$c_i$], [The $i^("th")$ codeword in the transmitted message, where $c_i in T_n$],
  [$D_h$], [The received (compressed) message, which for a complete Tunstall code is a sequence of $h$ Tunstall codewords $d_1 d_2 ... d_h$],
  [$d_i$], [The $i^("th")$ codeword in the received message, where $d_i in T_n$ for a complete Tunstall code],
)

// ==========================================
// PAGE 10: ABBREVIATIONS
// ==========================================
#front-heading("Abbreviations")

#grid(
  columns: (2cm, 1fr),
  row-gutter: 1.5em,

  [MCTS], [Monte Carlo Tree Search],
  [BPSK], [Binary Phase Shift Keying],
  [BSC], [Binary Symmetric Channel],
  [DCT], [Discrete Cosine Transform],
  [ECC], [Error Correcting Codes],
  [FEC], [Forward Error Correction],
  [JPEG], [Joint Photographic Experts Group],
  [MPEG], [Moving Pictures Experts Group],
  [SER], [Symbol Error Rate],
  [SNR], [Signal to Noise Ratio],
)

// ==========================================
// MAIN CONTENT START
// ==========================================
#set heading(numbering: "1.1")

// ==========================================
// CHAPTER 1
// ==========================================
= Introduction

== Opening Statement

Machine learning is plagued with an acceptance of the ann as a black box. In my project, I attempt
to deconstruct the learning process of transformer through a novel architecture, and by trying to
develop emergent behaviour through wider and unrestricted action spaces in more humanistic action representations.
the problem to be solved by the network will be chess, where there are highly ambiguous states with targets ranging from similarly ambiguous to
objective ground truths. a problem that technically has a perfect solution, but impossible to find, seems most fitting for this investigation.

This project's focus is distributed over 4 hypothesis:
 - whether self play in a pseudo legal chess environment converges to legal play.
 - how masking logits affect learning dynamics, investigation into "grokking" mechanics
 - the effectiveness of a 2 pass autoregressive encoder over a smaller action space in chess
 - how the injection of perfect data at different stages of the training process influences agent strength

== Aims of the Project
The aim of this project is to train multiple transformers on chess via self play reinforcement learning on legal and pseudo legal rule sets
and investigate how the different action spaces influence agent performance over time to speculate on the impact in ideal conditions (e.g.
with a lot more compute and time)

== Objectives
 - Build a bespoke chess client from scratch to handle both pseduo legal and legal game play, with:
  - a fucking move generator
  - zobrist hashing
  - state conversions to and from fen strings
  - bitboards and bitewise intrinsics
 - Architect a 2 pass autoregressive encoder and integrate it with the chess engine
 - construct a bipartite MCTS to self play on
 - train on generated self play data
 - collect statistics throughout training epochs, measuring leading statistics such as entropy, illegal move probability, elo
 - from snapshots of model weights, estimate elo of engines
 - evaluate impact of action space on agent
 - analyse model weights to search for implicit masking
 - suggest areas for improvement

== Project Motivations
i like chess and ml and systems programming

== Risks
there are a few risks with this project. one prominent risk is scope creep, especially with the bespoke construction of the chess client. in rust.
that was just a terrible mistake, however i am just built different

= Literature Review
== Chess fundamentals
=== Game rules
=== Board representation
=== Move generation
== Machine learning in chess
=== Policy
=== Value
=== MCTS
== History
=== Alpha beta pruning
=== NNUE
=== Transformers (attention is all you need)
== Now
=== Grokking (a classic)
@power2022grokking
=== AlphaZero (Masking)
@silver2017masteringchessshogiselfplay
=== MuZero (Latent space)
@Schrittwieser_2020
this destroys my disso, but i found out too late. maybe ill leave it out.
=== leela chess zero
@jenner2024evidencelearnedlookaheadchessplaying
=== Chess transformer
@monroe2024masteringchesstransformermodel
not read, please be different to mine
=== Chessformer
@monroe2026chessformer
i also didnt know about this. neither have i read it, god i hope its no good.

= Research Hypotheses
== Pseudo legal training
an agent learned to play pseudo legal chess should learn to play legal chess because king capture involves mating patterns and king safety involves pins
== Punishment propogation
allowing illegal/geometrically impossible moves during self play and allowing the propogating the punishment back should teach the model just as well if not better in the long run
== autoregressive move generator
more human like decision making approach in the action space is easier to train. trade off mcts depth vs width.
== Perfect data injection
how does injection of perfect data throughout training influence agent?
beginning: bootstrap value, forgetting.
throughout: probably best
at end: sharpens endgame, forgetting.

= Implementation details
the project is available on github.

== System architecture overview
== Client Architecture
=== Bitboard
=== Chessboard
=== ChessSquare
=== CastlingRights
=== ChessMove
=== ChessSquare
=== ChessGame
== Model Architecture
=== Encoder
=== Positional Embeddings
=== Policy
=== Value
== MCTS Architecture
=== Data
=== PUCT algorithm
=== Node
=== Edge

= Training & Self Play
== Self play
== MCTS
== Training (backprop via autograd)

= Results
== Hypotheses 1: pseudo legal
== Hypotheses 2: masking
== Hypotheses 2: EGTB injection
== Hypotheses 4: 2 pass model architecture.
=== not bothered to construct single pass architecture, requires building new mcts, fuck that

= evaluation
== fairness: same starting seed, epochs, search depth
== elo, benchmark a lot for good result
== illegal move probability
== policy entropy
== MCTS perft
== w/l to draw ratio
== graphs

= Conclusion
== what did i even do? i just fafo'd.

= challenges and areas of improvement
== building the chess client was challenging
== i shouldve used magic bitboards
== i shouldve adapted an existing chess engine
== i couldve trained and compared single vs dual pass transformer
== trained for longer, whole point was long term but my hardware is bad







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
  caption: [An example table, showing speed in operations per cycle per multiprocessor]
)

=== Adding figures

Figures are added using the `#figure` function. Typst automatically handles the numbering and placement of your images, as shown in the example that produces Figure 1.1. Common formats like PNG, JPEG, and SVG are supported natively.

=== Adding tables

Tables are defined directly in the source code using the `#table` function. This provides a highly flexible way to grid your data. An example of a styled table is given in Table 1.1.

=== Adding equations

A primary advantage of Typst is its intuitive mathematical notation. Equations can be written within `$ ... $` delimiters. It handles numbered equations easily, as in the recursive formula:

// Using op("Pr") for upright Pr
$ alpha(iota, x_2) = sum_(x_1, d_(iota-1)) alpha(iota - 1, x_1) op("Pr") { bold(r)_(n(iota-1)+x_1, n iota + x_2), bold(t)_(iota-1) } $

Inline math is also supported, for example to specify $1 <= iota < N$. Typst's syntax remains readable even for complex multiline expressions:

// Figure Example with subfigures
#figure(
  grid(
    columns: (1fr),
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
  caption: [An example figure, with two parts]
)

// Multiline equation with large brackets
$ op("Pr") { bold(r)_(0, n iota + x_2), sigma_(n iota) = x_2 } = sum_(x_1, d_(iota-1)) lr([
  op("Pr") { bold(r)_(0, n(iota-1)+x_1), sigma_(n(iota-1)) = x_1 } \
  times op("Pr") { bold(r)_(n(iota-1)+x_1, n iota + x_2), bold(t)_(iota-1) }
], size: #200%) $

=== code blocks

```rust
API:
pub struct ChessGame
pub ChessGame::castling_rights: CastlingRights
pub ChessGame::chessboard: ChessBoard
pub ChessGame::en_passant: Option<ChessSquare>
pub ChessGame::fullmove_counter: u32
pub ChessGame::game_history: alloc::vec::Vec<GameStateEntry>
pub ChessGame::halfmove_clock: u32
pub ChessGame::outcome: Outcome
pub ChessGame::rule_set: RuleSet
pub ChessGame::side_to_move: chess_piece::Color
pub ChessGame::zobrist_hash: u64
impl ChessGame
pub fn ChessGame::calculate_hash(&mut self) -> u64
pub fn ChessGame::check_game_state(&self) -> Outcome
pub fn ChessGame::fen_to_ascii(fen: &str)
pub fn ChessGame::from_fen(fen: &str) -> Self
pub fn ChessGame::generate_pseudolegal(&self) -> alloc::vec::Vec<chess_move::ChessMove>
pub fn ChessGame::is_legal(&self, mov: &chess_move::ChessMove) -> bool
pub fn ChessGame::make_move(&mut self, mov: &chess_move::ChessMove)
pub fn ChessGame::to_fen(&self) -> alloc::string::String
pub fn ChessGame::uci_to_move(&self, input: &str) -> result::Result<chess_move::ChessMove, &str>

```

// ==========================================
// BIBLIOGRAPHY
// ==========================================
#bibliography("refs.bib")
