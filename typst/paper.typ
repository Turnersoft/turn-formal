#import "common.typ": *

#set document(
  title: project-title,
  author: project-author,
)

#set page(
  numbering: "1",
  number-align: center,
  margin: (x: 2.5cm, y: 2.5cm),
)

#set text(
  font: "New Computer Modern",
  size: 11pt,
)

#set heading(
  numbering: "1.1",
)

#set par(
  justify: true,
  leading: 0.8em,
)

#show heading: it => {
  set text(fill: turn-blue)
  block(it)
}

// Title page
#align(center)[
  #block(
    width: 100%,
    inset: 2em,
    fill: turn-blue.lighten(90%),
    radius: 0.5em,
    [
      #text(size: 24pt, fill: turn-blue, weight: "bold")[#project-title]
      #linebreak()
      #text(size: 16pt, style: "italic")[#project-subtitle]
      #v(2em)
      #text(size: 14pt)[#project-author]
      #v(1em)
      #text(size: 12pt)[#datetime.today().display()]
    ]
  )
]

#pagebreak()

// Abstract
#heading(level: 1, numbering: none)[Abstract]

Turn-Formal is a new approach to formal mathematics and verification, built in Rust. This paper introduces the Turn-Formal library, its architecture, and its advantages over existing systems. We discuss the motivation behind high-level formal systems, demonstrate how Turn-Formal makes verification accessible to developers, and outline our roadmap for future development. Turn-Formal aims to bridge the gap between rigorous mathematical proof and practical software development, providing an expressive yet powerful framework for formal verification.

// Table of contents
#outline(
  title: "Contents",
  indent: auto,
)

#pagebreak()

// Main content
#heading[Introduction to Formal Systems]

#section1-content

#heading[High-Level Formal Mathematics]

#section2-content

#figure(
  rect(width: 100%, height: 5cm, stroke: none, fill: turn-blue.lighten(90%))[
    #align(center)[
      #text(fill: turn-blue)[
        *The Abstraction Continuum in Formal Mathematics*
        
        #v(1em)
        
        Low-level formalism #h(2em) $arrow.long$ #h(2em) Turn-Formal #h(2em) $arrow.long$ #h(2em) Informal mathematics
        
        #v(1em)
        
        Machine verification #h(1em) $arrow.l.r$ #h(1em) Balance #h(1em) $arrow.l.r$ #h(1em) Human intuition
      ]
    ]
  ],
  caption: [The position of Turn-Formal in the spectrum of mathematical formalization approaches]
)

#heading[Turn-Formal vs. Existing Systems]

#section3-content

The table below provides a feature comparison between Turn-Formal and other popular formal systems:

#figure(
  table(
    columns: (auto, auto, auto, auto),
    inset: 10pt,
    align: center,
    table.header(
      [*Feature*], [*Turn-Formal*], [*Lean4*], [*Coq*]
    ),
    [Implementation language], [Rust], [Lean], [OCaml],
    [Memory safety], [Native], [Runtime], [Runtime],
    [Metaprogramming], [Rust macros], [Meta-Lean], [Ltac],
    [Proof style], [Tactic & declarative], [Tactic & term], [Tactic-focused],
    [Learning curve], [Moderate], [Steep], [Steep],
    [Integration with standard tools], [Strong], [Limited], [Limited],
    [Performance], [High], [Moderate], [Moderate],
  ),
  caption: [Comparison of Turn-Formal with other formal verification systems]
)

#heading[Developer-Friendly Formal Verification]

#section4-content

Turn-Formal's API is designed to be intuitive and chainable:

#block(
  width: 100%,
  inset: 0.5em,
  fill: turn-blue.lighten(95%),
  radius: 0.5em,
  [
    *Example: Creating and building a proof*
    
    #proof-branch-example
  ]
)

The architecture of Turn-Formal follows a layered approach:

#figure(
  rect(width: 100%, height: 8cm, stroke: none, fill: none)[
    #align(center)[
      #table(
        columns: (1fr),
        rows: (auto, auto, auto, auto),
        stroke: 1pt + turn-blue,
        fill: (col, row) => if row == 3 { turn-blue.lighten(90%) } else { white },
        inset: 10pt,
        align: center,
        [*User-Facing API*\
         ProofBranch, TheoremBuilder, Tactics],
        [*Proof Structure*\
         ProofForest, ProofNode, CaseAnalysis],
        [*Mathematical Foundation*\
         MathRelation, MathExpression, Identifiers],
        [*Rust Core*\
         Memory Safety, Performance, Ecosystem],
      )
    ]
  ],
  caption: [The layered architecture of Turn-Formal]
)

#heading[Creating Theorems with Turn-Formal]

Turn-Formal makes it easy to express and prove theorems in various mathematical domains. Below is an example of proving a theorem in group theory:

#block(
  width: 100%,
  inset: 0.5em,
  fill: turn-blue.lighten(95%),
  radius: 0.5em,
  [
    *Example: Proving a theorem about group inverses*
    
    #theorem-example
  ]
)

This example demonstrates how Turn-Formal enables developers to express mathematical concepts directly in Rust, with a clear and readable syntax that closely mirrors mathematical notation while maintaining the full power of formal verification.

#heading[Future Roadmap]

#section5-content

#figure(
  rect(width: 100%, height: 6cm, stroke: none, fill: turn-blue.lighten(95%))[
    #align(center)[
      #text(fill: turn-blue)[
        *Turn-Formal Development Timeline*
        
        #v(1em)
        
        #table(
          columns: (1fr, 1fr, 1fr),
          inset: 10pt,
          align: center,
          [*Phase 1:*\
           Foundation], 
          [*Phase 2:*\
           Expansion], 
          [*Phase 3:*\
           Integration],
          [Core tactics], [Domain libraries], [Industry adoption],
          [Basic proof system], [ML assistance], [Education resources],
          [Foundational theorems], [Interactive tools], [Ecosystem growth],
        )
      ]
    ]
  ],
  caption: [Development phases for the Turn-Formal project]
)

As we continue to develop Turn-Formal, we invite the community to join us in building a more rigorous and reliable software ecosystem, where formal verification becomes a standard part of the development process rather than a specialized niche.

#pagebreak()

#heading(level: 1, numbering: none)[References]

// Create a bibliography
#bibliography("bibliography.bib", style: "ieee") 