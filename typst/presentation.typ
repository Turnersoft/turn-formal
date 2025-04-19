#import "common.typ": *

#set document(
  title: project-title,
  author: project-author,
)

#set page(
  paper: "presentation-16-9",
  margin: 1em,
  numbering: none,
)

#set text(
  font: "New Computer Modern",
  size: 20pt,
)

#set heading(
  numbering: none,
)

#show heading: it => {
  set text(fill: turn-blue)
  block(inset: (bottom: 0.5em), it)
}

// Title page
#align(center + horizon)[
  #block(
    width: 100%,
    inset: 2em,
    fill: turn-blue.lighten(90%),
    radius: 0.5em,
    [
      #text(size: 32pt, fill: turn-blue, weight: "bold")[#project-title]
      #v(0.5em)
      #text(size: 18pt, style: "italic")[#project-subtitle]
      #v(1em)
      #text(size: 16pt)[#project-author]
      #v(0.5em)
      #text(size: 14pt)[#datetime.today().display()]
    ]
  )
]

#pagebreak()

// Outline slide
#block(height: 100%, width: 100%, [
  #heading(level: 1)[Presentation Overview]

  #text(size: 18pt)[
    Today we'll discuss:
  ]
  
  #enum(
    numbering: "1.",
    spacing: 0.65em,
    [What is formal mathematics and why we care?],
    [Why "high-level" formal mathematics?],
    [How Turn-Formal is better than Lean4 & others],
    [Building a developer-friendly formal system],
    [Future roadmap]
  )
])

#pagebreak()

// Section 1
#block(height: 100%, width: 100%, [
  #heading(level: 1)[What is Formal Mathematics?]
  
  #text(size: 16pt)[
    *Formal systems provide:*
  ]
  
  #block(spacing: 0.65em,
    [- Mathematical foundation with machine-checkable proofs
     - Precise, unambiguous language and syntax
     - Well-defined rules of inference
     - Mechanically verifiable correctness]
  )
  
  #align(center)[
    #rect(width: 70%, height: 40%, fill: turn-blue.lighten(90%))[
      #align(center + horizon)[
        #text(fill: turn-blue)[Formal System Diagram]
      ]
    ]
  ]
])

#pagebreak()

#block(height: 100%, width: 100%, [
  #heading(level: 1)[Why We Care About Formalization]

  #grid(
    columns: (1fr, 1fr),
    gutter: 1em,
    [
      #text(size: 16pt)[
        *Why formalization matters:*
      ]
      
      #block(spacing: 0.65em,
        [- Eliminates ambiguity in mathematical proofs
         - Enables machine verification of correctness
         - Facilitates the development of verified software
         - Bridges the gap between theory and applications]
      )
    ],
    [
      #text(size: 16pt)[
        *Applications span:*
      ]
      
      #block(spacing: 0.65em,
        [- Cryptographic protocols
         - Safety-critical systems
         - Verified compilers
         - Advanced mathematics
         - AI reasoning systems]
      )
    ]
  )
])

#pagebreak()

// Section 2
#block(height: 100%, width: 100%, [
  #heading(level: 1)[Why "High-Level" Formal Mathematics?]
  
  #text(size: 16pt)[
    Traditional formal systems operate at a low level of abstraction:
  ]
  
  #block(spacing: 0.65em,
    [- Human mathematicians don't think in terms of basic inference rules
     - Proofs become overly verbose and hard to understand
     - The gap between informal and formal proofs is too wide]
  )
  
  #text(size: 16pt)[
    *High-level formal mathematics aims to:*
  ]
  
  #block(spacing: 0.65em,
    [- Match the intuition and workflow of human mathematicians
     - Abstract away mechanical details while maintaining rigor
     - Provide a natural language-like experience]
  )
])

#pagebreak()

#block(height: 100%, width: 100%, [
  #heading(level: 1)[The Abstraction Gap]
  
  #align(center)[
    #rect(width: 90%, height: 75%, stroke: none, fill: turn-blue.lighten(95%))[
      #align(center + horizon)[
        #text(fill: turn-blue)[
          *The Abstraction Continuum in Formal Mathematics*
          
          #v(0.5em)
          #grid(
            columns: (1fr, auto, 1fr, auto, 1fr),
            rows: (auto),
            column-gutter: 0.8em,
            [
              #align(center)[
                #text(size: 14pt)[Low-level formalism]
                #v(0.3em)
                #text(size: 12pt)[Machine-friendly]
                #text(size: 12pt)[Human-hostile]
              ]
            ],
            [$arrow.long$],
            [
              #align(center)[
                #text(weight: "bold", size: 14pt)[Turn-Formal]
                #v(0.3em)
                #text(size: 12pt)[Balance]
                #text(size: 12pt)[Best of both]
              ]
            ],
            [$arrow.long$],
            [
              #align(center)[
                #text(size: 14pt)[Informal mathematics]
                #v(0.3em)
                #text(size: 12pt)[Human-friendly]
                #text(size: 12pt)[Machine-hostile]
              ]
            ]
          )
        ]
      ]
    ]
  ]
])

#pagebreak()

// Section 3
#block(height: 100%, width: 100%, [
  #heading(level: 1)[How Turn-Formal is Better than Lean4 & Others]
  
  #text(size: 16pt)[
    *Key advantages of Turn-Formal:*
  ]
  
  #grid(
    columns: (1fr, 1fr),
    gutter: 1em,
    [
      #block(spacing: 0.65em,
        [- *Performance*: Rust's speed & memory safety
         - *Accessibility*: Lower barrier to entry
         - *Integration*: Seamless Rust ecosystem interop]
      )
    ],
    [
      #block(spacing: 0.65em,
        [- *Modularity*: Flexible architecture
         - *Expressiveness*: Rich syntax for intuitive proofs
         - *Developer-centric*: Built for engineers]
      )
    ]
  )
  
  #v(1em)
  
  // Add a comparison table
  #align(center)[
    #table(
      columns: (auto, auto, auto, auto),
      inset: 8pt,
      align: center,
      fill: (col, row) => if row == 0 { turn-blue.lighten(90%) } else { none },
      stroke: 1pt + turn-blue,
      [*Feature*], [*Turn-Formal*], [*Lean4*], [*Coq*],
      [Implementation], [Rust], [Lean], [OCaml],
      [Memory safety], [Native], [Runtime], [Runtime],
      [Learning curve], [Moderate], [Steep], [Steep],
    )
  ]
])

#pagebreak()

// Section 4
#block(height: 100%, width: 100%, [
  #heading(level: 1)[Building a Developer-Friendly Formal System]
  
  #text(size: 16pt)[
    *Turn-Formal makes verification accessible through:*
  ]
  
  #grid(
    columns: (1fr, 1fr),
    gutter: 1em,
    [
      #block(spacing: 0.65em,
        [- Familiar Rust syntax and semantics
         - Strong type system
         - Flexible tactics system
         - Chainable, fluent API]
      )
    ],
    [
      #text(size: 16pt)[
        *Core components:*
      ]
      
      #block(spacing: 0.65em,
        [- *ProofState*
         - *Tactics*
         - *TheoremBuilder*
         - *ProofBranch*
         - *MathRelation*]
      )
    ]
  )
])

#pagebreak()

#block(height: 100%, width: 100%, [
  #heading(level: 1)[Code Example]
  
  #text(size: 16pt)[
    *Example: Creating a proof branch*
  ]
  
  #align(center)[
    #block(
      width: 95%,
      inset: 0.5em,
      fill: turn-blue.lighten(95%),
      radius: 0.5em,
      [
        #proof-branch-example
      ]
    )
  ]
])

#pagebreak()

// Section 5
#block(height: 100%, width: 100%, [
  #heading(level: 1)[Future Roadmap]
  
  #text(size: 16pt)[
    *Development timeline:*
  ]
  
  #grid(
    columns: (1fr, 1fr, 1fr),
    gutter: 0.8em,
    [
      #text(size: 14pt, weight: "bold")[Short-term (2023-2024)]
      
      #block(spacing: 0.6em,
        [- Core library
         - Foundation math
         - Basic tactics
         - Developer tools]
      )
    ],
    [
      #text(size: 14pt, weight: "bold")[Medium-term (2024-2026)]
      
      #block(spacing: 0.6em,
        [- Domain libraries
         - Proof search
         - Interoperability
         - Community ecosystem]
      )
    ],
    [
      #text(size: 14pt, weight: "bold")[Long-term (2026+)]
      
      #block(spacing: 0.6em,
        [- AI integration
         - Formal verification for all
         - Cross-system translation
         - Industrial applications]
      )
    ]
  )
  
  #v(1em)
  
  #align(center)[
    #rect(width: 90%, height: auto, stroke: 1pt + turn-blue, fill: turn-blue.lighten(95%), radius: 0.3em, inset: 0.8em)[
      #align(center)[
        #text(fill: turn-blue)[Our vision: Make formal verification a standard part of software development]
      ]
    ]
  ]
])

#pagebreak()

// Final slide
#align(center + horizon)[
  #block(
    width: 90%,
    inset: 1.5em,
    fill: turn-blue.lighten(90%),
    radius: 0.5em,
    [
      #align(center)[
        #text(fill: turn-blue, size: 24pt, weight: "bold")[Thank You!]
        #v(1em)
        #text(fill: turn-blue, size: 18pt)[Get involved with Turn-Formal:]
        #v(0.5em)
        #text(size: 16pt)[Website: #link("https://turn-lang.com")[turn-lang.com]]
        #text(size: 16pt)[GitHub: #link("https://github.com/turn-lang/turn-formal")[github.com/turn-lang/turn-formal]]
        #text(size: 16pt)[Documentation: #link("https://docs.turn-lang.com/turn-formal")[docs.turn-lang.com/turn-formal]]
      ]
    ]
  )
] 