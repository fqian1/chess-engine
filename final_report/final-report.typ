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

  [$A_f$], [The source message, being a sequence of $f$ source symbols $a_1 a_2 ... a_f$],
  [$a_i$], [The $i^("th")$ symbol in the source message, where $a_i in S_m$],
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

  [BER], [Bit Error Rate],
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

== Dissertation Format

This template is provided to facilitate the process of writing up your dissertation while ensuring its format is consistent with requirements. In using this template, do not, under any circumstances, make any changes to the class file provided. In particular, do not attempt to make any changes to the title page, the statement of originality, or the copyright page.

Use of this template is not mandatory; if you choose not to use it, then you must make your final layout resemble this output as closely as possible. In particular, the textual content and layout of the title page, statement of originality, and copyright page must not be changed.

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

=== Adding code fragments

Source code can be included using "raw" blocks (triple backticks). Typst provides built-in syntax highlighting for various programming languages.

```c
typedef int v4si __attribute__ ((vector_size (16)));

void ArrayAdd(int *c, const int *a, const int *b, int n)
{
  const v4si *va = (const v4si *)a;
  const v4si *vb = (const v4si *)b;
  v4si *vc = (v4si *)c;
  int i = 0;
  if(n > 4)
    for(; i < n / 4; i++)
      vc[i] = va[i] + vb[i];
  for(i *= 4; i < n; i++)
    c[i] = a[i] + b[i];
}
```

=== Adding references

Typst handles bibliographies through the #bibliography function, supporting both .bib and .yml formats. This separates your source data from the citation style, which is managed automatically by the template. Examples of various reference types included in the project are:

#list(
  [Conference papers (Briffa, Schaathun & Wesemeyer 2010)],
  [Journal articles (el Gamal, Hemachandra, Shperling & Wei 1987)],
  [Dissertations (Tunstall 1968)],
  [Books (Press, Teukolsky, Vetterling & Flannery 1992)],
  [User or technical manuals (NVI 2009, Henderson 2009)],
  [Technical Reports (Burrows & Wheeler 1994)],
  [Other material that cannot be easily classified (Farrell 1992)]
)

// ==========================================
// BIBLIOGRAPHY
// ==========================================
#front-heading("Bibliography")

#set par(hanging-indent: 1cm)

Briffa, J. A., Schaathun, H. G. & Wesemeyer, S. (2010), An improved decoding algorithm for the Davey-MacKay construction, in ‘Proc. IEEE Intern. Conf. on Commun.’, Cape Town, South Africa.

Burrows, M. & Wheeler, D. J. (1994), A block-sorting lossless data compression algorithm, Technical report, Digital SRC Research Report.

el Gamal, A. A., Hemachandra, L. A., Shperling, I. & Wei, V. K. (1987), ‘Using simulated annealing to design good codes’, _IEEE Transactions on Information Theory_ *33*(1), 116–123.

Farrell, P. G. (1992), ‘Notes on source coding’. University of Manchester.

Henderson, B. (2009), _Netpbm_. \
URL: #link("http://netpbm.sourceforge.net/doc/")[http://netpbm.sourceforge.net/doc/]

NVI (2009), _NVIDIA CUDA Programming Guide_. Version 2.3.

Press, W. H., Teukolsky, S. A., Vetterling, W. T. & Flannery, B. P. (1992), _Numerical Recipes in C: The Art of Scientific Computing_, second edn, Cambridge Universty Press.

Tunstall, B. P. (1968), Synthesis of Noiseless Compression Codes, PhD thesis, Georgia Institute of Technology.
