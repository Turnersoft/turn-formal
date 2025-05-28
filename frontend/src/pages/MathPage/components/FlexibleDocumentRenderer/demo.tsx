import React, { useState } from 'react';
import { FlexibleDocumentRenderer } from './index';
import { FlexibleDocument, DocumentType } from './types';
import styles from './demo.module.css';

// Comprehensive mathematical content library for demonstrations
const mathContent = {
  // Basic formulas
  eulerIdentity: "e^(i*π) + 1 = 0",
  quadraticFormula: "x = (-b ± √(b² - 4ac)) / (2a)",
  pythagorean: "a² + b² = c²",
  derivativeDefinition: "f'(x) = lim(h→0) [f(x+h) - f(x)]/h",
  
  // Calculus
  integral: "∫ f(x) dx",
  definiteIntegral: "∫[a,b] f(x) dx = F(b) - F(a)",
  partsByParts: "∫ u dv = uv - ∫ v du",
  fundamentalTheorem: "d/dx ∫[a,x] f(t) dt = f(x)",
  
  // Series and sequences
  summation: "∑(n=1 to ∞) 1/n²",
  geometricSeries: "∑(n=0 to ∞) ar^n = a/(1-r) for |r| < 1",
  taylorSeries: "f(x) = ∑(n=0 to ∞) f^(n)(a)/n! * (x-a)^n",
  
  // Linear algebra
  matrix: "[a b; c d]",
  matrixMultiplication: "AB = [∑(k) a_ik * b_kj]",
  determinant: "det(A) = ad - bc",
  eigenvalue: "Av = λv",
  
  // Complex analysis
  cauchyRiemann: "∂u/∂x = ∂v/∂y, ∂u/∂y = -∂v/∂x",
  residueTheorem: "∮_C f(z) dz = 2πi ∑ Res(f, z_k)",
  
  // Group theory
  groupAxioms: "∀a,b,c ∈ G: (ab)c = a(bc), ∃e: ae = ea = a, ∃a⁻¹: aa⁻¹ = a⁻¹a = e",
  lagrangeTheorem: "|G| = |H| · [G:H]",
  
  // Number theory
  primeCountingFunction: "π(x) ~ x/ln(x)",
  riemannHypothesis: "ζ(s) = ∑(n=1 to ∞) 1/n^s",
  
  // Topology
  continuity: "∀ε > 0, ∃δ > 0: |x-a| < δ ⇒ |f(x)-f(a)| < ε",
  
  // Statistics
  normalDistribution: "f(x) = (1/√(2πσ²)) * e^(-(x-μ)²/(2σ²))",
  
  // Limits
  limit: "lim(x→0) sin(x)/x = 1",
  lhopital: "lim(x→a) f(x)/g(x) = lim(x→a) f'(x)/g'(x)",
};

// Individual comprehensive document examples following section_node.rs types exactly

const scientificPaperDocument: FlexibleDocument = {
  id: "comprehensive-scientific-paper",
  title: "A Comprehensive Analysis of Euler's Identity and Its Applications in Modern Complex Analysis",
  document_type: "ScientificPaper",
  language: "en",
  version: "2.0",
  authors: ["Dr. Complex Analysis", "Prof. Mathematical Beauty", "Dr. Number Theory"],
  date_published: "2024-01-15",
  date_modified: "2024-01-20",
  abstract_content: {
    id: "abstract",
    content: [{
      type: "Paragraph",
      content: {
        segments: [
          { type: "Text", content: "This paper provides a comprehensive examination of Euler's identity " },
          { type: "Math", content: { content: mathContent.eulerIdentity, display_style: false } },
          { type: "Text", content: ", often regarded as the most beautiful equation in mathematics. We explore its profound connections to complex analysis, number theory, and topology. Through rigorous mathematical analysis and novel applications, we demonstrate how this elegant formula serves as a bridge between seemingly disparate areas of mathematics. Our results include new insights into the geometric interpretation of complex exponentials, applications to signal processing, and connections to modern cryptography." }
        ]
      }
    }]
  },
  body: [
    {
      id: "introduction",
      title: { segments: [{ type: "Text", content: "1. Introduction" }] },
      content: [
        {
          type: "Paragraph",
          content: {
            segments: [
              { type: "Text", content: "Euler's identity, " },
              { type: "Math", content: { content: mathContent.eulerIdentity, display_style: false } },
              { type: "Text", content: ", represents one of the most profound and elegant relationships in all of mathematics. This remarkable equation unifies five fundamental mathematical constants: " },
              { type: "Math", content: { content: "e", display_style: false } },
              { type: "Text", content: " (Euler's number), " },
              { type: "Math", content: { content: "i", display_style: false } },
              { type: "Text", content: " (the imaginary unit), " },
              { type: "Math", content: { content: "π", display_style: false } },
              { type: "Text", content: " (pi), " },
              { type: "Math", content: { content: "1", display_style: false } },
              { type: "Text", content: " (unity), and " },
              { type: "Math", content: { content: "0", display_style: false } },
              { type: "Text", content: " (zero)." }
            ]
          }
        },
        {
          type: "MathBlock",
          math: { content: mathContent.eulerIdentity, display_style: true },
          label: "eq:euler"
        },
        {
          type: "Paragraph",
          content: {
            segments: [
              { type: "Text", content: "The significance of this identity extends far beyond its aesthetic appeal. It emerges naturally from Euler's formula " },
              { type: "Math", content: { content: "e^(ix) = cos(x) + i*sin(x)", display_style: false } },
              { type: "Text", content: " when evaluated at " },
              { type: "Math", content: { content: "x = π", display_style: false } },
              { type: "Text", content: "." }
            ]
          }
        }
      ]
    },
    {
      id: "mathematical-foundations",
      title: { segments: [{ type: "Text", content: "2. Mathematical Foundations" }] },
      content: [
        {
          type: "StructuredMath",
          content: {
            type: "Definition",
            term_display: [{ type: "Text", content: "Complex Exponential Function" }],
            formal_term: { content: "e^z", display_style: false },
            label: "def:complex-exp",
            body: [
              {
                type: "Paragraph",
                content: {
                  segments: [
                    { type: "Text", content: "The complex exponential function is defined by the infinite series:" }
                  ]
                }
              },
              {
                type: "MathBlock",
                math: { content: "e^z = ∑(n=0 to ∞) z^n/n!", display_style: true }
              },
              {
                type: "Paragraph",
                content: {
                  segments: [
                    { type: "Text", content: "For purely imaginary arguments " },
                    { type: "Math", content: { content: "z = ix", display_style: false } },
                    { type: "Text", content: ", this yields Euler's formula through the separation of real and imaginary parts." }
                  ]
                }
              }
            ]
          }
        },
        {
          type: "StructuredMath",
          content: {
            type: "TheoremLike",
            kind: "Theorem",
            label: "thm:euler-formula",
            statement: [
              {
                type: "Paragraph",
                content: {
                  segments: [
                    { type: "StyledText", text: "Euler's Formula.", styles: ["Bold"] },
                    { type: "Text", content: " For any real number x:" }
                  ]
                }
              },
              {
                type: "MathBlock",
                math: { content: "e^(ix) = cos(x) + i*sin(x)", display_style: true }
              }
            ],
            proof: {
              title: { segments: [{ type: "StyledText", text: "Proof.", styles: ["Bold"] }] },
              steps: [
                {
                  type: "Statement",
                  claim: [
                    { type: "Text", content: "We start with the Taylor series for " },
                    { type: "Math", content: { content: "e^x", display_style: false } },
                    { type: "Text", content: ", " },
                    { type: "Math", content: { content: "cos(x)", display_style: false } },
                    { type: "Text", content: ", and " },
                    { type: "Math", content: { content: "sin(x)", display_style: false } },
                    { type: "Text", content: ":" }
                  ]
                },
                {
                  type: "Elaboration",
                  content: [
                    {
                      type: "MathBlock",
                      math: { content: "e^x = 1 + x + x²/2! + x³/3! + x⁴/4! + ...", display_style: true }
                    },
                    {
                      type: "MathBlock",
                      math: { content: "cos(x) = 1 - x²/2! + x⁴/4! - x⁶/6! + ...", display_style: true }
                    },
                    {
                      type: "MathBlock",
                      math: { content: "sin(x) = x - x³/3! + x⁵/5! - x⁷/7! + ...", display_style: true }
                    }
                  ]
                },
                {
                  type: "Statement",
                  claim: [
                    { type: "Text", content: "Substituting " },
                    { type: "Math", content: { content: "ix", display_style: false } },
                    { type: "Text", content: " into the exponential series and using " },
                    { type: "Math", content: { content: "i² = -1", display_style: false } },
                    { type: "Text", content: ", " },
                    { type: "Math", content: { content: "i³ = -i", display_style: false } },
                    { type: "Text", content: ", " },
                    { type: "Math", content: { content: "i⁴ = 1", display_style: false } },
                    { type: "Text", content: ":" }
                  ]
                },
                {
                  type: "Elaboration",
                  content: [
                    {
                      type: "MathBlock",
                      math: { content: "e^(ix) = 1 + ix + (ix)²/2! + (ix)³/3! + (ix)⁴/4! + ...", display_style: true }
                    },
                    {
                      type: "MathBlock",
                      math: { content: "= 1 + ix - x²/2! - ix³/3! + x⁴/4! + ix⁵/5! - ...", display_style: true }
                    }
                  ]
                }
              ],
              qed_symbol: "□"
            }
          }
        }
      ]
    },
    {
      id: "geometric-interpretation",
      title: { segments: [{ type: "Text", content: "3. Geometric Interpretation" }] },
      content: [
        {
          type: "Paragraph",
          content: {
            segments: [
              { type: "Text", content: "The geometric interpretation of Euler's identity reveals its deep connection to the unit circle in the complex plane." }
            ]
          }
        },
        {
          type: "SideBySideLayout",
          content: {
            left_panel: {
              id: "left-theory",
              title: { segments: [{ type: "Text", content: "Mathematical Description" }] },
              content: [
                {
                  type: "Paragraph",
                  content: {
                    segments: [
                      { type: "Text", content: "The parametric equations of the unit circle are:" }
                    ]
                  }
                },
                {
                  type: "MathBlock",
                  math: { content: "x = cos(θ), y = sin(θ)", display_style: true }
                },
                {
                  type: "Paragraph",
                  content: {
                    segments: [
                      { type: "Text", content: "At " },
                      { type: "Math", content: { content: "θ = π", display_style: false } },
                      { type: "Text", content: ", we reach the point (-1, 0)." }
                    ]
                  }
                }
              ],
              panel_role: "SourceTheory"
            },
            right_panel: {
              id: "right-application",
              title: { segments: [{ type: "Text", content: "Geometric Visualization" }] },
              content: [
                {
                  type: "Paragraph",
                  content: {
                    segments: [
                      { type: "Text", content: "In the complex plane, " },
                      { type: "Math", content: { content: "e^(iπ)", display_style: false } },
                      { type: "Text", content: " represents a rotation of π radians (180°) from the positive real axis." }
                    ]
                  }
                },
                {
                  type: "MathBlock",
                  math: { content: "e^(iπ) = cos(π) + i*sin(π) = -1 + 0i = -1", display_style: true }
                }
              ],
              panel_role: "TargetTheory"
            }
          }
        }
      ]
    },
    {
      id: "applications",
      title: { segments: [{ type: "Text", content: "4. Applications and Extensions" }] },
      content: [
        {
          type: "List",
          content: {
            style: { type: "Ordered", content: "Decimal" },
            items: [
              {
                content: [{
                  type: "Paragraph",
                  content: {
                    segments: [
                      { type: "StyledText", text: "Signal Processing", styles: ["Bold"] },
                      { type: "Text", content: ": Fourier transforms rely on " },
                      { type: "Math", content: { content: "e^(i2πft)", display_style: false } },
                      { type: "Text", content: " for frequency domain analysis." }
                    ]
                  }
                }]
              },
              {
                content: [{
                  type: "Paragraph",
                  content: {
                    segments: [
                      { type: "StyledText", text: "Quantum Mechanics", styles: ["Bold"] },
                      { type: "Text", content: ": Wave functions involve " },
                      { type: "Math", content: { content: "e^(iEt/ℏ)", display_style: false } },
                      { type: "Text", content: "." }
                    ]
                  }
                }]
              },
              {
                content: [{
                  type: "Paragraph",
                  content: {
                    segments: [
                      { type: "StyledText", text: "Number Theory", styles: ["Bold"] },
                      { type: "Text", content: ": Connections to " },
                      { type: "Math", content: { content: mathContent.riemannHypothesis, display_style: false } },
                      { type: "Text", content: "." }
                    ]
                  }
                }]
              }
            ]
          }
        }
      ]
    }
  ],
  bibliography: [
    { entry_type: "article", fields: [["author", "L. Euler"], ["title", "Introductio in analysin infinitorum"], ["year", "1748"]] },
    { entry_type: "book", fields: [["author", "R. P. Feynman"], ["title", "The Feynman Lectures on Physics"], ["year", "1963"]] },
    { entry_type: "article", fields: [["author", "G. H. Hardy"], ["title", "A Mathematician's Apology"], ["year", "1940"]] }
  ],
  presentation_config: {
    layout_style: "TwoColumn",
    interaction_features: ["ClickableLinks", "ExpandableProofs"],
    target_audience: "Graduate",
    formality_level: "FullyFormal"
  }
};

const blogPostDocument: FlexibleDocument = {
  id: "comprehensive-blog-post",
  title: "The Mathematical Beauty of Euler's Identity: A Journey Through Complex Numbers",
  document_type: "BlogPost",
  language: "en",
  version: "1.0",
  authors: ["Alex MathBlogger"],
  date_published: "2024-01-15",
  body: [{
    id: "main",
    content: [
      {
        type: "Paragraph",
        content: {
          segments: [
            { type: "Text", content: "Hey math lovers! 👋 Today we're diving into one of the most mind-blowing equations in all of mathematics. Have you ever wondered why the quadratic formula " },
            { type: "Math", content: { content: mathContent.quadraticFormula, display_style: false } },
            { type: "Text", content: " works so elegantly? Well, buckle up because we're about to explore something even more amazing!" }
          ]
        }
      },
      {
        type: "AlertBox",
        style: "Information",
        content: [{
          type: "Paragraph",
          content: {
            segments: [
              { type: "StyledText", text: "✨ Fun Fact:", styles: ["Bold"] },
              { type: "Text", content: " Euler's identity " },
              { type: "Math", content: { content: mathContent.eulerIdentity, display_style: false } },
              { type: "Text", content: " combines five of the most important numbers in mathematics!" }
            ]
          }
        }]
      },
      {
        type: "Paragraph",
        content: {
          segments: [
            { type: "StyledText", text: "What makes this equation special?", styles: ["Bold"] },
            { type: "Text", content: " Let me break it down for you:" }
          ]
        }
      },
      {
        type: "List",
        content: {
          style: { type: "Unordered", content: "Disc" },
          items: [
            {
              content: [{
                type: "Paragraph",
                content: {
                  segments: [
                    { type: "Math", content: { content: "e", display_style: false } },
                    { type: "Text", content: " - Euler's number (≈2.718), the base of natural logarithms" }
                  ]
                }
              }]
            },
            {
              content: [{
                type: "Paragraph",
                content: {
                  segments: [
                    { type: "Math", content: { content: "i", display_style: false } },
                    { type: "Text", content: " - The imaginary unit, where " },
                    { type: "Math", content: { content: "i² = -1", display_style: false } }
                  ]
                }
              }]
            },
            {
              content: [{
                type: "Paragraph",
                content: {
                  segments: [
                    { type: "Math", content: { content: "π", display_style: false } },
                    { type: "Text", content: " - Pi (≈3.14159), the ratio of a circle's circumference to its diameter" }
                  ]
                }
              }]
            }
          ]
        }
      },
      {
        type: "MathBlock",
        math: { content: mathContent.eulerIdentity, display_style: true },
        caption: { segments: [{ type: "Text", content: "The most beautiful equation in mathematics!" }] }
      },
      {
        type: "Paragraph",
        content: {
          segments: [
            { type: "StyledText", text: "The Journey to Understanding", styles: ["Bold"] }
          ]
        }
      },
      {
        type: "Columns",
        content: {
          columns_content: [
            [{
              type: "Paragraph",
              content: {
                segments: [
                  { type: "StyledText", text: "🎵 Music & Audio", styles: ["Bold"] },
                  { type: "Text", content: ": Digital signal processing uses Fourier transforms, which rely heavily on complex exponentials." }
                ]
              }
            }],
            [{
              type: "Paragraph",
              content: {
                segments: [
                  { type: "StyledText", text: "🔬 Physics", styles: ["Bold"] },
                  { type: "Text", content: ": Quantum mechanics describes particles using complex wave functions." }
                ]
              }
            }]
          ],
          column_widths: ["1fr", "1fr"]
        }
      },
      {
        type: "CodeBlock",
        content: {
          code: `# Python example: Euler's formula in action
import numpy as np
import matplotlib.pyplot as plt

# Generate complex numbers using Euler's formula
theta = np.linspace(0, 2*np.pi, 100)
z = np.exp(1j * theta)  # e^(i*theta)

# Plot the unit circle
plt.plot(z.real, z.imag)
plt.title("Euler's Formula: Tracing the Unit Circle")
plt.axis('equal')
plt.grid(True)`,
          language: "python",
          caption: { segments: [{ type: "Text", content: "Visualizing Euler's formula with Python" }] }
        }
      },
      {
        type: "QuoteBlock",
        content: {
          content: [{
            segments: [
              { type: "Text", content: "\"Euler's identity is absolutely paradoxical; we cannot understand it, and we don't know what it means, but we have proved it, and therefore we know it must be the truth.\"" }
            ]
          }],
          attribution: { segments: [{ type: "Text", content: "Benjamin Peirce" }] }
        }
      },
      {
        type: "AlertBox",
        style: "Tip",
        content: [{
          type: "Paragraph",
          content: {
            segments: [
              { type: "StyledText", text: "💡 Want to explore more?", styles: ["Bold"] },
              { type: "Text", content: " Try graphing " },
              { type: "Math", content: { content: "e^(ix)", display_style: false } },
              { type: "Text", content: " for different values of x and see how it traces out the unit circle!" }
            ]
          }
        }]
      }
    ]
  }],
  bibliography: [],
  presentation_config: {
    layout_style: "SingleColumn",
    interaction_features: ["HoverTooltips", "ClickableLinks"],
    target_audience: "GeneralPublic",
    formality_level: "Conversational"
  }
};

const tooltipSummaryDocument: FlexibleDocument = {
  id: "comprehensive-tooltip-summary",
  title: "Complex Analysis: Key Concepts",
  document_type: "TooltipSummary",
  language: "en",
  version: "1.0",
  body: [{
    id: "content",
    content: [
      {
        type: "Paragraph",
        content: {
          segments: [
            { type: "StyledText", text: "Complex Analysis", styles: ["Bold"] },
            { type: "Text", content: " is the study of functions of complex variables. It provides powerful tools for solving problems in mathematics, physics, and engineering." }
          ]
        }
      },
      {
        type: "List",
        content: {
          style: { type: "Unordered", content: "Disc" },
          items: [
            {
              content: [{
                type: "Paragraph",
                content: {
                  segments: [
                    { type: "StyledText", text: "Complex Numbers:", styles: ["Bold"] },
                    { type: "Text", content: " Numbers of the form " },
                    { type: "Math", content: { content: "z = a + bi", display_style: false } },
                    { type: "Text", content: " where " },
                    { type: "Math", content: { content: "i² = -1", display_style: false } }
                  ]
                }
              }]
            },
            {
              content: [{
                type: "Paragraph",
                content: {
                  segments: [
                    { type: "StyledText", text: "Euler's Formula:", styles: ["Bold"] },
                    { type: "Text", content: " " },
                    { type: "Math", content: { content: "e^(ix) = cos(x) + i*sin(x)", display_style: false } }
                  ]
                }
              }]
            },
            {
              content: [{
                type: "Paragraph",
                content: {
                  segments: [
                    { type: "StyledText", text: "Cauchy-Riemann Equations:", styles: ["Bold"] },
                    { type: "Text", content: " " },
                    { type: "Math", content: { content: mathContent.cauchyRiemann, display_style: false } }
                  ]
                }
              }]
            }
          ]
        }
      },
      {
        type: "AlertBox",
        style: "Information",
        content: [{
          type: "Paragraph",
          content: {
            segments: [
              { type: "StyledText", text: "Key Insight:", styles: ["Bold"] },
              { type: "Text", content: " Analytic functions (those satisfying Cauchy-Riemann) have remarkable properties: they are infinitely differentiable and satisfy the maximum principle." }
            ]
          }
        }]
      },
      {
        type: "MathBlock",
        math: { content: mathContent.residueTheorem, display_style: true },
        caption: { segments: [{ type: "Text", content: "Residue Theorem - fundamental for complex integration" }] }
      }
    ]
  }],
  bibliography: [],
  presentation_config: {
    layout_style: "Compact",
    interaction_features: ["HoverTooltips"],
    target_audience: "Student",
    formality_level: "SemiFormal"
  }
};

const animatedPresentationDocument: FlexibleDocument = {
  id: "comprehensive-animated-presentation",
  title: "Understanding Limits: An Interactive Journey",
  document_type: "AnimatedPresentation",
  language: "en",
  version: "1.0",
  body: [
    {
      id: "slide1",
      title: { segments: [{ type: "Text", content: "What is a Limit?" }] },
      content: [
        {
          type: "Paragraph",
          content: {
            segments: [
              { type: "Text", content: "A limit describes the behavior of a function as the input approaches a specific value." }
            ]
          }
        },
        {
          type: "MathBlock",
          math: { content: mathContent.limit, display_style: true }
        },
        {
          type: "AlertBox",
          style: "Information",
          content: [{
            type: "Paragraph",
            content: {
              segments: [
                { type: "Text", content: "This is one of the most important limits in calculus - it appears in the derivative of sine!" }
              ]
            }
          }]
        }
      ]
    },
    {
      id: "slide2",
      title: { segments: [{ type: "Text", content: "Formal Definition" }] },
      content: [
        {
          type: "Paragraph",
          content: {
            segments: [
              { type: "Text", content: "The formal epsilon-delta definition of a limit:" }
            ]
          }
        },
        {
          type: "MathBlock",
          math: { content: mathContent.continuity, display_style: true }
        },
        {
          type: "Paragraph",
          content: {
            segments: [
              { type: "Text", content: "This says that for any small distance ε from the target value, we can find a corresponding distance δ such that inputs within δ of a produce outputs within ε of the limit." }
            ]
          }
        }
      ]
    },
    {
      id: "slide3",
      title: { segments: [{ type: "Text", content: "L'Hôpital's Rule" }] },
      content: [
        {
          type: "Paragraph",
          content: {
            segments: [
              { type: "Text", content: "When we encounter indeterminate forms like 0/0, L'Hôpital's rule comes to the rescue:" }
            ]
          }
        },
        {
          type: "MathBlock",
          math: { content: mathContent.lhopital, display_style: true }
        },
        {
          type: "Paragraph",
          content: {
            segments: [
              { type: "Text", content: "This powerful technique allows us to evaluate many challenging limits by taking derivatives of the numerator and denominator." }
            ]
          }
        }
      ]
    },
    {
      id: "slide4",
      title: { segments: [{ type: "Text", content: "Applications" }] },
      content: [
        {
          type: "List",
          content: {
            style: { type: "Ordered", content: "Decimal" },
            items: [
              {
                content: [{
                  type: "Paragraph",
                  content: {
                    segments: [
                      { type: "StyledText", text: "Derivatives:", styles: ["Bold"] },
                      { type: "Text", content: " " },
                      { type: "Math", content: { content: mathContent.derivativeDefinition, display_style: false } }
                    ]
                  }
                }]
              },
              {
                content: [{
                  type: "Paragraph",
                  content: {
                    segments: [
                      { type: "StyledText", text: "Integrals:", styles: ["Bold"] },
                      { type: "Text", content: " As limits of Riemann sums" }
                    ]
                  }
                }]
              },
              {
                content: [{
                  type: "Paragraph",
                  content: {
                    segments: [
                      { type: "StyledText", text: "Series:", styles: ["Bold"] },
                      { type: "Text", content: " Infinite sums like " },
                      { type: "Math", content: { content: mathContent.summation, display_style: false } }
                    ]
                  }
                }]
              }
            ]
          }
        }
      ]
    }
  ],
  bibliography: [],
  presentation_config: {
    layout_style: "Presentation",
    interaction_features: ["Animations", "InteractiveControls"],
    target_audience: "Undergraduate",
    formality_level: "SemiFormal",
    animation_config: {
      enabled_animations: ["slide-transition", "fade-in", "zoom"],
      animation_speed: 1.0,
      auto_play: false,
      show_controls: true
    }
  }
};

const resourcePanelDocument: FlexibleDocument = {
  id: "comprehensive-resource-panel",
  title: "Mathematical Formulas Reference Library",
  document_type: "ResourcePanel",
  language: "en",
  version: "1.0",
  body: [
    {
      id: "calculus",
      title: { segments: [{ type: "Text", content: "📊 Calculus" }] },
      content: [
        {
          type: "Grid",
          content: {
            items: [
              {
                content: {
                  type: "Paragraph",
                  content: {
                    segments: [
                      { type: "StyledText", text: "Derivatives", styles: ["Bold"] }
                    ]
                  }
                }
              },
              {
                content: {
                  type: "MathBlock",
                  math: { content: mathContent.derivativeDefinition, display_style: true }
                }
              },
              {
                content: {
                  type: "Paragraph",
                  content: {
                    segments: [
                      { type: "Text", content: "The fundamental definition of a derivative as a limit" }
                    ]
                  }
                }
              }
            ],
            column_template: "1fr 2fr 1fr"
          }
        },
        {
          type: "Grid",
          content: {
            items: [
              {
                content: {
                  type: "Paragraph",
                  content: {
                    segments: [
                      { type: "StyledText", text: "Integration", styles: ["Bold"] }
                    ]
                  }
                }
              },
              {
                content: {
                  type: "MathBlock",
                  math: { content: mathContent.fundamentalTheorem, display_style: true }
                }
              },
              {
                content: {
                  type: "Paragraph",
                  content: {
                    segments: [
                      { type: "Text", content: "Fundamental Theorem of Calculus" }
                    ]
                  }
                }
              }
            ],
            column_template: "1fr 2fr 1fr"
          }
        },
        {
          type: "Grid",
          content: {
            items: [
              {
                content: {
                  type: "Paragraph",
                  content: {
                    segments: [
                      { type: "StyledText", text: "Integration by Parts", styles: ["Bold"] }
                    ]
                  }
                }
              },
              {
                content: {
                  type: "MathBlock",
                  math: { content: mathContent.partsByParts, display_style: true }
                }
              },
              {
                content: {
                  type: "Paragraph",
                  content: {
                    segments: [
                      { type: "Text", content: "Essential technique for product integrals" }
                    ]
                  }
                }
              }
            ],
            column_template: "1fr 2fr 1fr"
          }
        }
      ]
    },
    {
      id: "algebra",
      title: { segments: [{ type: "Text", content: "🔢 Algebra" }] },
      content: [
        {
          type: "Table",
          content: {
            caption: { segments: [{ type: "Text", content: "Essential Algebraic Formulas" }] },
            header_rows: [{
              cells: [
                { content: [{ type: "Paragraph", content: { segments: [{ type: "Text", content: "Formula Name" }] } }], cell_type: "Header" },
                { content: [{ type: "Paragraph", content: { segments: [{ type: "Text", content: "Expression" }] } }], cell_type: "Header" },
                { content: [{ type: "Paragraph", content: { segments: [{ type: "Text", content: "Application" }] } }], cell_type: "Header" }
              ]
            }],
            body_rows: [
              {
                cells: [
                  { content: [{ type: "Paragraph", content: { segments: [{ type: "Text", content: "Quadratic Formula" }] } }], cell_type: "Data" },
                  { content: [{ type: "MathBlock", math: { content: mathContent.quadraticFormula, display_style: false } }], cell_type: "Data" },
                  { content: [{ type: "Paragraph", content: { segments: [{ type: "Text", content: "Solving quadratic equations" }] } }], cell_type: "Data" }
                ]
              },
              {
                cells: [
                  { content: [{ type: "Paragraph", content: { segments: [{ type: "Text", content: "Pythagorean Theorem" }] } }], cell_type: "Data" },
                  { content: [{ type: "MathBlock", math: { content: mathContent.pythagorean, display_style: false } }], cell_type: "Data" },
                  { content: [{ type: "Paragraph", content: { segments: [{ type: "Text", content: "Right triangle relationships" }] } }], cell_type: "Data" }
                ]
              }
            ]
          }
        }
      ]
    },
    {
      id: "complex-analysis",
      title: { segments: [{ type: "Text", content: "🌀 Complex Analysis" }] },
      content: [
        {
          type: "CollapsibleBlock",
          content: {
            summary: [{ type: "Text", content: "Euler's Identity and Related Formulas" }],
            details: [
              {
                type: "MathBlock",
                math: { content: mathContent.eulerIdentity, display_style: true },
                caption: { segments: [{ type: "Text", content: "The most beautiful equation in mathematics" }] }
              },
              {
                type: "MathBlock",
                math: { content: "e^(ix) = cos(x) + i*sin(x)", display_style: true },
                caption: { segments: [{ type: "Text", content: "Euler's Formula" }] }
              },
              {
                type: "Paragraph",
                content: {
                  segments: [
                    { type: "Text", content: "These formulas connect exponential functions, trigonometry, and complex numbers in a profound way." }
                  ]
                }
              }
            ]
          }
        },
        {
          type: "CollapsibleBlock",
          content: {
            summary: [{ type: "Text", content: "Cauchy-Riemann Equations" }],
            details: [
              {
                type: "MathBlock",
                math: { content: mathContent.cauchyRiemann, display_style: true }
              },
              {
                type: "Paragraph",
                content: {
                  segments: [
                    { type: "Text", content: "These equations are necessary and sufficient conditions for a function to be analytic (complex differentiable)." }
                  ]
                }
              }
            ]
          }
        }
      ]
    },
    {
      id: "series",
      title: { segments: [{ type: "Text", content: "∞ Series and Sequences" }] },
      content: [
        {
          type: "Paragraph",
          content: {
            segments: [
              { type: "Text", content: "Important series that appear throughout mathematics:" }
            ]
          }
        },
        {
          type: "List",
          content: {
            style: { type: "Unordered", content: "Disc" },
            items: [
              {
                content: [{
                  type: "Paragraph",
                  content: {
                    segments: [
                      { type: "StyledText", text: "Geometric Series:", styles: ["Bold"] },
                      { type: "Text", content: " " },
                      { type: "Math", content: { content: mathContent.geometricSeries, display_style: false } }
                    ]
                  }
                }]
              },
              {
                content: [{
                  type: "Paragraph",
                  content: {
                    segments: [
                      { type: "StyledText", text: "Taylor Series:", styles: ["Bold"] },
                      { type: "Text", content: " " },
                      { type: "Math", content: { content: mathContent.taylorSeries, display_style: false } }
                    ]
                  }
                }]
              },
              {
                content: [{
                  type: "Paragraph",
                  content: {
                    segments: [
                      { type: "StyledText", text: "Basel Problem:", styles: ["Bold"] },
                      { type: "Text", content: " " },
                      { type: "Math", content: { content: mathContent.summation + " = π²/6", display_style: false } }
                    ]
                  }
                }]
              }
            ]
          }
        }
      ]
    }
  ],
  bibliography: [],
  presentation_config: {
    layout_style: "Sidebar",
    interaction_features: ["ClickableLinks", "HoverTooltips", "ExpandableProofs"],
    target_audience: "Student",
    formality_level: "SemiFormal"
  }
};

const wikiPageDocument: FlexibleDocument = {
  id: "comprehensive-wiki-page",
  title: "Group Theory",
  document_type: "WikiPage",
  language: "en",
  version: "3.2",
  authors: ["Mathematical Community"],
  date_modified: "2024-01-20",
  body: [
    {
      id: "introduction",
      title: { segments: [{ type: "Text", content: "Introduction" }] },
      content: [
        {
          type: "Paragraph",
          content: {
            segments: [
              { type: "StyledText", text: "Group theory", styles: ["Bold"] },
              { type: "Text", content: " is the mathematical study of symmetry. Groups capture the essential properties of transformations that preserve structure, making them fundamental to many areas of mathematics and physics." }
            ]
          }
        },
        {
          type: "StructuredMath",
          content: {
            type: "Definition",
            term_display: [{ type: "Text", content: "Group" }],
            formal_term: { content: "(G, ·)", display_style: false },
            body: [
              {
                type: "Paragraph",
                content: {
                  segments: [
                    { type: "Text", content: "A group is a set G together with a binary operation · that satisfies four axioms:" }
                  ]
                }
              },
              {
                type: "List",
                content: {
                  style: { type: "Ordered", content: "Decimal" },
                  items: [
                    {
                      content: [{
                        type: "Paragraph",
                        content: {
                          segments: [
                            { type: "StyledText", text: "Closure:", styles: ["Bold"] },
                            { type: "Text", content: " For all a, b ∈ G, the result a · b is also in G." }
                          ]
                        }
                      }]
                    },
                    {
                      content: [{
                        type: "Paragraph",
                        content: {
                          segments: [
                            { type: "StyledText", text: "Associativity:", styles: ["Bold"] },
                            { type: "Text", content: " For all a, b, c ∈ G: (a · b) · c = a · (b · c)" }
                          ]
                        }
                      }]
                    },
                    {
                      content: [{
                        type: "Paragraph",
                        content: {
                          segments: [
                            { type: "StyledText", text: "Identity:", styles: ["Bold"] },
                            { type: "Text", content: " There exists an element e ∈ G such that for all a ∈ G: e · a = a · e = a" }
                          ]
                        }
                      }]
                    },
                    {
                      content: [{
                        type: "Paragraph",
                        content: {
                          segments: [
                            { type: "StyledText", text: "Inverse:", styles: ["Bold"] },
                            { type: "Text", content: " For each a ∈ G, there exists a⁻¹ ∈ G such that a · a⁻¹ = a⁻¹ · a = e" }
                          ]
                        }
                      }]
                    }
                  ]
                }
              }
            ]
          }
        }
      ]
    },
    {
      id: "examples",
      title: { segments: [{ type: "Text", content: "Examples" }] },
      content: [
        {
          type: "StructuredMath",
          content: {
            type: "Example",
            label: "ex:integers",
            introduction: [{
              type: "Paragraph",
              content: {
                segments: [
                  { type: "Text", content: "The integers under addition form a group:" }
                ]
              }
            }],
            body: [
              {
                type: "Paragraph",
                content: {
                  segments: [
                    { type: "Text", content: "Consider the set " },
                    { type: "Math", content: { content: "ℤ = {..., -2, -1, 0, 1, 2, ...}", display_style: false } },
                    { type: "Text", content: " with the operation of addition:" }
                  ]
                }
              },
              {
                type: "List",
                content: {
                  style: { type: "Unordered", content: "Disc" },
                  items: [
                    {
                      content: [{
                        type: "Paragraph",
                        content: {
                          segments: [
                            { type: "StyledText", text: "Closure:", styles: ["Bold"] },
                            { type: "Text", content: " The sum of any two integers is an integer" }
                          ]
                        }
                      }]
                    },
                    {
                      content: [{
                        type: "Paragraph",
                        content: {
                          segments: [
                            { type: "StyledText", text: "Associativity:", styles: ["Bold"] },
                            { type: "Text", content: " " },
                            { type: "Math", content: { content: "(a + b) + c = a + (b + c)", display_style: false } }
                          ]
                        }
                      }]
                    },
                    {
                      content: [{
                        type: "Paragraph",
                        content: {
                          segments: [
                            { type: "StyledText", text: "Identity:", styles: ["Bold"] },
                            { type: "Text", content: " 0 is the additive identity" }
                          ]
                        }
                      }]
                    },
                    {
                      content: [{
                        type: "Paragraph",
                        content: {
                          segments: [
                            { type: "StyledText", text: "Inverse:", styles: ["Bold"] },
                            { type: "Text", content: " For each integer a, -a is its additive inverse" }
                          ]
                        }
                      }]
                    }
                  ]
                }
              }
            ]
          }
        }
      ]
    },
    {
      id: "theorems",
      title: { segments: [{ type: "Text", content: "Important Theorems" }] },
      content: [
        {
          type: "StructuredMath",
          content: {
            type: "TheoremLike",
            kind: "Theorem",
            label: "thm:lagrange",
            statement: [
              {
                type: "Paragraph",
                content: {
                  segments: [
                    { type: "StyledText", text: "Lagrange's Theorem.", styles: ["Bold"] },
                    { type: "Text", content: " Let G be a finite group and H be a subgroup of G. Then:" }
                  ]
                }
              },
              {
                type: "MathBlock",
                math: { content: mathContent.lagrangeTheorem, display_style: true }
              },
              {
                type: "Paragraph",
                content: {
                  segments: [
                    { type: "Text", content: "where [G:H] denotes the index of H in G." }
                  ]
                }
              }
            ]
          }
        }
      ]
    }
  ],
  bibliography: [
    { entry_type: "book", fields: [["author", "I.N. Herstein"], ["title", "Topics in Algebra"], ["year", "1975"]] },
    { entry_type: "book", fields: [["author", "J.J. Rotman"], ["title", "An Introduction to the Theory of Groups"], ["year", "1995"]] }
  ],
  presentation_config: {
    layout_style: "SingleColumn",
    interaction_features: ["ClickableLinks", "HoverTooltips"],
    target_audience: "Undergraduate",
    formality_level: "SemiFormal"
  }
};

const textbookDocument: FlexibleDocument = {
  id: "comprehensive-textbook",
  title: "Linear Algebra: Foundations and Applications",
  document_type: "Textbook",
  language: "en",
  version: "4.0",
  authors: ["Dr. Linear Algebra", "Prof. Matrix Theory"],
  date_published: "2024-01-01",
  table_of_contents: {
    title: "Chapter 3: Eigenvalues and Eigenvectors",
    target_id: "chapter3",
    children: [
      { title: "3.1 Definitions", target_id: "definitions", children: [] },
      { title: "3.2 Computing Eigenvalues", target_id: "computing", children: [] },
      { title: "3.3 Applications", target_id: "applications", children: [] }
    ]
  },
  body: [
    {
      id: "chapter3",
      title: { segments: [{ type: "Text", content: "Chapter 3: Eigenvalues and Eigenvectors" }] },
      content: [
        {
          type: "Paragraph",
          content: {
            segments: [
              { type: "Text", content: "In this chapter, we explore one of the most important concepts in linear algebra: eigenvalues and eigenvectors. These concepts have applications ranging from principal component analysis in statistics to quantum mechanics in physics." }
            ]
          }
        }
      ]
    },
    {
      id: "definitions",
      title: { segments: [{ type: "Text", content: "3.1 Definitions" }] },
      content: [
        {
          type: "StructuredMath",
          content: {
            type: "Definition",
            term_display: [{ type: "Text", content: "Eigenvalue and Eigenvector" }],
            label: "def:eigenvalue",
            body: [
              {
                type: "Paragraph",
                content: {
                  segments: [
                    { type: "Text", content: "Let A be an n×n matrix. A scalar λ is called an " },
                    { type: "StyledText", text: "eigenvalue", styles: ["Bold"] },
                    { type: "Text", content: " of A if there exists a non-zero vector v such that:" }
                  ]
                }
              },
              {
                type: "MathBlock",
                math: { content: mathContent.eigenvalue, display_style: true }
              },
              {
                type: "Paragraph",
                content: {
                  segments: [
                    { type: "Text", content: "The vector v is called an " },
                    { type: "StyledText", text: "eigenvector", styles: ["Bold"] },
                    { type: "Text", content: " corresponding to the eigenvalue λ." }
                  ]
                }
              }
            ]
          }
        },
        {
          type: "StructuredMath",
          content: {
            type: "Example",
            label: "ex:simple-eigenvalue",
            body: [
              {
                type: "Paragraph",
                content: {
                  segments: [
                    { type: "Text", content: "Consider the matrix:" }
                  ]
                }
              },
              {
                type: "MathBlock",
                math: { content: "A = [2 1; 0 3]", display_style: true }
              },
              {
                type: "Paragraph",
                content: {
                  segments: [
                    { type: "Text", content: "We can verify that λ = 2 is an eigenvalue with eigenvector v = [1; 0]:" }
                  ]
                }
              },
              {
                type: "MathBlock",
                math: { content: "A·v = [2 1; 0 3]·[1; 0] = [2; 0] = 2·[1; 0] = λ·v", display_style: true }
              }
            ]
          }
        }
      ]
    },
    {
      id: "computing",
      title: { segments: [{ type: "Text", content: "3.2 Computing Eigenvalues" }] },
      content: [
        {
          type: "Paragraph",
          content: {
            segments: [
              { type: "Text", content: "To find eigenvalues, we solve the characteristic equation:" }
            ]
          }
        },
        {
          type: "MathBlock",
          math: { content: "det(A - λI) = 0", display_style: true }
        },
        {
          type: "StructuredMath",
          content: {
            type: "TheoremLike",
            kind: "Theorem",
            label: "thm:characteristic-poly",
            statement: [
              {
                type: "Paragraph",
                content: {
                  segments: [
                    { type: "Text", content: "The eigenvalues of an n×n matrix A are exactly the roots of the characteristic polynomial " },
                    { type: "Math", content: { content: "p(λ) = det(A - λI)", display_style: false } },
                    { type: "Text", content: "." }
                  ]
                }
              }
            ]
          }
        }
      ]
    },
    {
      id: "exercises",
      title: { segments: [{ type: "Text", content: "Exercises" }] },
      content: [
        {
          type: "StructuredMath",
          content: {
            type: "Exercise",
            label: "ex:3.1",
            problem_statement: [
              {
                type: "Paragraph",
                content: {
                  segments: [
                    { type: "Text", content: "Find the eigenvalues and eigenvectors of the matrix:" }
                  ]
                }
              },
              {
                type: "MathBlock",
                math: { content: "A = [4 -2; 1 1]", display_style: true }
              }
            ],
            hints: [{
              summary: [{ type: "Text", content: "Hint" }],
              details: [
                {
                  type: "Paragraph",
                  content: {
                    segments: [
                      { type: "Text", content: "Start by computing the characteristic polynomial " },
                      { type: "Math", content: { content: "det(A - λI)", display_style: false } },
                      { type: "Text", content: "." }
                    ]
                  }
                }
              ]
            }],
            solution: {
              summary: [{ type: "Text", content: "Solution" }],
              details: [
                {
                  type: "Paragraph",
                  content: {
                    segments: [
                      { type: "Text", content: "The characteristic polynomial is:" }
                    ]
                  }
                },
                {
                  type: "MathBlock",
                  math: { content: "det(A - λI) = det([4-λ -2; 1 1-λ]) = (4-λ)(1-λ) + 2 = λ² - 5λ + 6", display_style: true }
                },
                {
                  type: "Paragraph",
                  content: {
                    segments: [
                      { type: "Text", content: "Solving " },
                      { type: "Math", content: { content: "λ² - 5λ + 6 = 0", display_style: false } },
                      { type: "Text", content: " gives eigenvalues λ₁ = 2 and λ₂ = 3." }
                    ]
                  }
                }
              ]
            }
          }
        }
      ]
    }
  ],
  bibliography: [
    { entry_type: "book", fields: [["author", "Gilbert Strang"], ["title", "Linear Algebra and Its Applications"], ["year", "2016"]] }
  ],
  presentation_config: {
    layout_style: "SingleColumn",
    interaction_features: ["ClickableLinks", "ExpandableProofs"],
    target_audience: "Undergraduate",
    formality_level: "SemiFormal"
  }
};

const personalNotesDocument: FlexibleDocument = {
  id: "comprehensive-personal-notes",
  title: "My Notes on Complex Analysis",
  document_type: "PersonalNotes",
  language: "en",
  version: "1.2",
  authors: ["Student"],
  date_modified: "2024-01-20",
  body: [
    {
      id: "main-notes",
      content: [
        {
          type: "Paragraph",
          content: {
            segments: [
              { type: "Text", content: "📝 Study notes for Complex Analysis midterm - Prof. Johnson's class" }
            ]
          }
        },
        {
          type: "AlertBox",
          style: "Note",
          content: [{
            type: "Paragraph",
            content: {
              segments: [
                { type: "StyledText", text: "Remember:", styles: ["Bold"] },
                { type: "Text", content: " Complex numbers have both real and imaginary parts!" }
              ]
            }
          }]
        },
        {
          type: "Paragraph",
          content: {
            segments: [
              { type: "StyledText", text: "Key Formula - Euler's Identity:", styles: ["Bold", "Underline"] }
            ]
          }
        },
        {
          type: "MathBlock",
          math: { content: mathContent.eulerIdentity, display_style: true }
        },
        {
          type: "Paragraph",
          content: {
            segments: [
              { type: "Text", content: "This comes from Euler's formula " },
              { type: "Math", content: { content: "e^(ix) = cos(x) + i*sin(x)", display_style: false } },
              { type: "Text", content: " when x = π." }
            ]
          }
        },
        {
          type: "List",
          content: {
            style: { type: "Unordered", content: "Disc" },
            items: [
              {
                content: [{
                  type: "Paragraph",
                  content: {
                    segments: [
                      { type: "Text", content: "✅ Memorize Cauchy-Riemann equations: " },
                      { type: "Math", content: { content: mathContent.cauchyRiemann, display_style: false } }
                    ]
                  }
                }]
              },
              {
                content: [{
                  type: "Paragraph",
                  content: {
                    segments: [
                      { type: "Text", content: "⚠️ Don't forget: analytic functions are infinitely differentiable!" }
                    ]
                  }
                }]
              },
              {
                content: [{
                  type: "Paragraph",
                  content: {
                    segments: [
                      { type: "Text", content: "🔍 Review residue theorem for the exam" }
                    ]
                  }
                }]
              }
            ]
          }
        },
        {
          type: "AlertBox",
          style: "Warning",
          content: [{
            type: "Paragraph",
            content: {
              segments: [
                { type: "StyledText", text: "Exam tip:", styles: ["Bold"] },
                { type: "Text", content: " Always check if a function satisfies C-R equations before claiming it's analytic!" }
              ]
            }
          }]
        },
        {
          type: "Paragraph",
          content: {
            segments: [
              { type: "StyledText", text: "Practice Problems to Review:", styles: ["Bold"] }
            ]
          }
        },
        {
          type: "List",
          content: {
            style: { type: "Ordered", content: "Decimal" },
            items: [
              {
                content: [{
                  type: "Paragraph",
                  content: {
                    segments: [
                      { type: "Text", content: "Homework 3, Problem 5 (contour integration)" }
                    ]
                  }
                }]
              },
              {
                content: [{
                  type: "Paragraph",
                  content: {
                    segments: [
                      { type: "Text", content: "Quiz 2 (finding residues)" }
                    ]
                  }
                }]
              },
              {
                content: [{
                  type: "Paragraph",
                  content: {
                    segments: [
                      { type: "Text", content: "Textbook examples from Chapter 4" }
                    ]
                  }
                }]
              }
            ]
          }
        }
      ]
    }
  ],
  bibliography: [],
  presentation_config: {
    layout_style: "SingleColumn",
    interaction_features: ["HoverTooltips"],
    target_audience: "Student",
    formality_level: "Conversational"
  }
};

const mathematicianNotesDocument: FlexibleDocument = {
  id: "comprehensive-mathematician-notes",
  title: "Research Notes: Connections Between Algebraic Topology and Number Theory",
  document_type: "MathematicianNotes",
  language: "en",
  version: "3.1",
  authors: ["Prof. Research Mathematician"],
  date_modified: "2024-01-20",
  body: [
    {
      id: "research-outline",
      title: { segments: [{ type: "Text", content: "Research Direction" }] },
      content: [
        {
          type: "Paragraph",
          content: {
            segments: [
              { type: "Text", content: "Investigating the deep connections between étale cohomology and L-functions, following Weil's conjectures and subsequent developments." }
            ]
          }
        },
        {
          type: "StructuredMath",
          content: {
            type: "Definition",
            term_display: [{ type: "Text", content: "L-function" }],
            formal_term: { content: "L(s, χ)", display_style: false },
            body: [
              {
                type: "Paragraph",
                content: {
                  segments: [
                    { type: "Text", content: "For a Dirichlet character χ, the associated L-function is defined by:" }
                  ]
                }
              },
              {
                type: "MathBlock",
                math: { content: "L(s, χ) = ∑(n=1 to ∞) χ(n)/n^s", display_style: true }
              },
              {
                type: "Paragraph",
                content: {
                  segments: [
                    { type: "Text", content: "This generalizes the Riemann zeta function " },
                    { type: "Math", content: { content: mathContent.riemannHypothesis, display_style: false } },
                    { type: "Text", content: " when χ is the trivial character." }
                  ]
                }
              }
            ]
          }
        }
      ]
    },
    {
      id: "conjectures",
      title: { segments: [{ type: "Text", content: "Key Conjectures" }] },
      content: [
        {
          type: "StructuredMath",
          content: {
            type: "TheoremLike",
            kind: "Conjecture",
            label: "conj:generalized-riemann",
            statement: [
              {
                type: "Paragraph",
                content: {
                  segments: [
                    { type: "StyledText", text: "Generalized Riemann Hypothesis.", styles: ["Bold"] },
                    { type: "Text", content: " For any Dirichlet character χ, all non-trivial zeros of L(s, χ) have real part 1/2." }
                  ]
                }
              }
            ]
          }
        },
        {
          type: "Paragraph",
          content: {
            segments: [
              { type: "StyledText", text: "Progress Notes:", styles: ["Bold"] },
              { type: "Text", content: " The connection to étale cohomology through the Weil conjectures (now theorems, proven by Deligne) suggests a geometric approach to understanding these analytic properties." }
            ]
          }
        }
      ]
    },
    {
      id: "calculations",
      title: { segments: [{ type: "Text", content: "Recent Calculations" }] },
      content: [
        {
          type: "Paragraph",
          content: {
            segments: [
              { type: "Text", content: "Working on explicit calculations for elliptic curves over finite fields. The Hasse bound gives us:" }
            ]
          }
        },
        {
          type: "MathBlock",
          math: { content: "|E(𝔽_q)| - (q + 1)| ≤ 2√q", display_style: true }
        },
        {
          type: "Paragraph",
          content: {
            segments: [
              { type: "Text", content: "This connects to the eigenvalues of Frobenius acting on H¹_ét(E, ℚ_ℓ), which have absolute value √q by Weil's theorems." }
            ]
          }
        }
      ]
    },
    {
      id: "open-questions",
      title: { segments: [{ type: "Text", content: "Open Questions" }] },
      content: [
        {
          type: "List",
          content: {
            style: { type: "Ordered", content: "Decimal" },
            items: [
              {
                content: [{
                  type: "Paragraph",
                  content: {
                    segments: [
                      { type: "Text", content: "Can we extend the Langlands correspondence to higher dimensional varieties over function fields?" }
                    ]
                  }
                }]
              },
              {
                content: [{
                  type: "Paragraph",
                  content: {
                    segments: [
                      { type: "Text", content: "What is the precise relationship between motivic cohomology and special values of L-functions?" }
                    ]
                  }
                }]
              },
              {
                content: [{
                  type: "Paragraph",
                  content: {
                    segments: [
                      { type: "Text", content: "How do p-adic L-functions relate to the étale cohomology of Shimura varieties?" }
                    ]
                  }
                }]
              }
            ]
          }
        }
      ]
    }
  ],
  bibliography: [
    { entry_type: "article", fields: [["author", "P. Deligne"], ["title", "La conjecture de Weil"], ["journal", "Publ. Math. IHES"], ["year", "1974"]] },
    { entry_type: "book", fields: [["author", "J.-P. Serre"], ["title", "Local Fields"], ["year", "1979"]] }
  ],
  presentation_config: {
    layout_style: "SingleColumn",
    interaction_features: ["ClickableLinks", "ExpandableProofs"],
    target_audience: "Expert",
    formality_level: "FullyFormal"
  }
};

const studyNotesDocument: FlexibleDocument = {
  id: "comprehensive-study-notes",
  title: "Calculus Study Guide: Limits and Derivatives",
  document_type: "StudyNotes",
  language: "en",
  version: "2.0",
  authors: ["Study Group"],
  date_modified: "2024-01-18",
  body: [
    {
      id: "limits-review",
      title: { segments: [{ type: "Text", content: "📚 Limits Review" }] },
      content: [
        {
          type: "AlertBox",
          style: "Tip",
          content: [{
            type: "Paragraph",
            content: {
              segments: [
                { type: "StyledText", text: "Study Tip:", styles: ["Bold"] },
                { type: "Text", content: " Always check if you can substitute directly before using L'Hôpital's rule!" }
              ]
            }
          }]
        },
        {
          type: "Paragraph",
          content: {
            segments: [
              { type: "StyledText", text: "Key Limit:", styles: ["Bold"] }
            ]
          }
        },
        {
          type: "MathBlock",
          math: { content: mathContent.limit, display_style: true }
        },
        {
          type: "Paragraph",
          content: {
            segments: [
              { type: "StyledText", text: "Why is this important?", styles: ["Bold"] },
              { type: "Text", content: " This limit appears when finding the derivative of sin(x)!" }
            ]
          }
        },
        {
          type: "Paragraph",
          content: {
            segments: [
              { type: "StyledText", text: "L'Hôpital's Rule:", styles: ["Bold"] }
            ]
          }
        },
        {
          type: "MathBlock",
          math: { content: mathContent.lhopital, display_style: true }
        },
        {
          type: "AlertBox",
          style: "Warning",
          content: [{
            type: "Paragraph",
            content: {
              segments: [
                { type: "StyledText", text: "Remember:", styles: ["Bold"] },
                { type: "Text", content: " L'Hôpital's rule only works for indeterminate forms like 0/0 or ∞/∞!" }
              ]
            }
          }]
        }
      ]
    },
    {
      id: "derivatives-review",
      title: { segments: [{ type: "Text", content: "📈 Derivatives Review" }] },
      content: [
        {
          type: "Paragraph",
          content: {
            segments: [
              { type: "StyledText", text: "Definition of Derivative:", styles: ["Bold"] }
            ]
          }
        },
        {
          type: "MathBlock",
          math: { content: mathContent.derivativeDefinition, display_style: true }
        },
        {
          type: "Paragraph",
          content: {
            segments: [
              { type: "StyledText", text: "Common Derivatives to Memorize:", styles: ["Bold"] }
            ]
          }
        },
        {
          type: "Table",
          content: {
            caption: { segments: [{ type: "Text", content: "Basic Derivative Formulas" }] },
            header_rows: [{
              cells: [
                { content: [{ type: "Paragraph", content: { segments: [{ type: "Text", content: "Function" }] } }], cell_type: "Header" },
                { content: [{ type: "Paragraph", content: { segments: [{ type: "Text", content: "Derivative" }] } }], cell_type: "Header" }
              ]
            }],
            body_rows: [
              {
                cells: [
                  { content: [{ type: "MathBlock", math: { content: "x^n", display_style: false } }], cell_type: "Data" },
                  { content: [{ type: "MathBlock", math: { content: "nx^(n-1)", display_style: false } }], cell_type: "Data" }
                ]
              },
              {
                cells: [
                  { content: [{ type: "MathBlock", math: { content: "sin(x)", display_style: false } }], cell_type: "Data" },
                  { content: [{ type: "MathBlock", math: { content: "cos(x)", display_style: false } }], cell_type: "Data" }
                ]
              },
              {
                cells: [
                  { content: [{ type: "MathBlock", math: { content: "cos(x)", display_style: false } }], cell_type: "Data" },
                  { content: [{ type: "MathBlock", math: { content: "-sin(x)", display_style: false } }], cell_type: "Data" }
                ]
              },
              {
                cells: [
                  { content: [{ type: "MathBlock", math: { content: "e^x", display_style: false } }], cell_type: "Data" },
                  { content: [{ type: "MathBlock", math: { content: "e^x", display_style: false } }], cell_type: "Data" }
                ]
              }
            ]
          }
        }
      ]
    },
    {
      id: "practice-problems",
      title: { segments: [{ type: "Text", content: "💪 Practice Problems" }] },
      content: [
        {
          type: "Paragraph",
          content: {
            segments: [
              { type: "StyledText", text: "Problem 1:", styles: ["Bold"] },
              { type: "Text", content: " Find the limit:" }
            ]
          }
        },
        {
          type: "MathBlock",
          math: { content: "lim(x→0) (sin(3x))/(2x)", display_style: true }
        },
        {
          type: "CollapsibleBlock",
          content: {
            summary: [{ type: "Text", content: "💡 Solution" }],
            details: [
              {
                type: "Paragraph",
                content: {
                  segments: [
                    { type: "Text", content: "We can rewrite this as:" }
                  ]
                }
              },
              {
                type: "MathBlock",
                math: { content: "lim(x→0) (sin(3x))/(2x) = (3/2) · lim(x→0) (sin(3x))/(3x)", display_style: true }
              },
              {
                type: "Paragraph",
                content: {
                  segments: [
                    { type: "Text", content: "Using the standard limit " },
                    { type: "Math", content: { content: "lim(u→0) sin(u)/u = 1", display_style: false } },
                    { type: "Text", content: " with u = 3x, we get 3/2." }
                  ]
                }
              }
            ]
          }
        },
        {
          type: "Paragraph",
          content: {
            segments: [
              { type: "StyledText", text: "Problem 2:", styles: ["Bold"] },
              { type: "Text", content: " Find the derivative of " },
              { type: "Math", content: { content: "f(x) = x² sin(x)", display_style: false } }
            ]
          }
        },
        {
          type: "CollapsibleBlock",
          content: {
            summary: [{ type: "Text", content: "💡 Solution" }],
            details: [
              {
                type: "Paragraph",
                content: {
                  segments: [
                    { type: "Text", content: "Using the product rule:" }
                  ]
                }
              },
              {
                type: "MathBlock",
                math: { content: "f'(x) = (x²)' sin(x) + x² (sin(x))' = 2x sin(x) + x² cos(x)", display_style: true }
              }
            ]
          }
        }
      ]
    }
  ],
  bibliography: [],
  presentation_config: {
    layout_style: "SingleColumn",
    interaction_features: ["HoverTooltips", "ExpandableProofs"],
    target_audience: "Undergraduate",
    formality_level: "SemiFormal"
  }
};

const comparisonPageDocument: FlexibleDocument = {
  id: "comprehensive-comparison-page",
  title: "Comparison: Classical vs. Modern Group Theory Approaches",
  document_type: "ComparisonPage",
  language: "en",
  version: "1.0",
  body: [
    {
      id: "comparison",
      title: { segments: [{ type: "Text", content: "Classical vs. Modern Approaches" }] },
      content: [
        {
          type: "SideBySideLayout",
          content: {
            left_panel: {
              id: "classical-approach",
              title: { segments: [{ type: "Text", content: "🏛️ Classical Approach" }] },
              content: [
                {
                  type: "Paragraph",
                  content: {
                    segments: [
                      { type: "StyledText", text: "Historical Development:", styles: ["Bold"] },
                      { type: "Text", content: " Group theory emerged from studying polynomial equations and geometric symmetries." }
                    ]
                  }
                },
                {
                  type: "StructuredMath",
                  content: {
                    type: "Definition",
                    term_display: [{ type: "Text", content: "Classical Group Definition" }],
                    body: [
                      {
                        type: "Paragraph",
                        content: {
                          segments: [
                            { type: "Text", content: "A group was originally defined as a set of transformations closed under composition, with:" }
                          ]
                        }
                      },
                      {
                        type: "List",
                        content: {
                          style: { type: "Unordered", content: "Disc" },
                          items: [
                            {
                              content: [{
                                type: "Paragraph",
                                content: {
                                  segments: [
                                    { type: "Text", content: "Identity transformation" }
                                  ]
                                }
                              }]
                            },
                            {
                              content: [{
                                type: "Paragraph",
                                content: {
                                  segments: [
                                    { type: "Text", content: "Inverse transformations" }
                                  ]
                                }
                              }]
                            },
                            {
                              content: [{
                                type: "Paragraph",
                                content: {
                                  segments: [
                                    { type: "Text", content: "Associative composition" }
                                  ]
                                }
                              }]
                            }
                          ]
                        }
                      }
                    ]
                  }
                },
                {
                  type: "Paragraph",
                  content: {
                    segments: [
                      { type: "StyledText", text: "Key Features:", styles: ["Bold"] }
                    ]
                  }
                },
                {
                  type: "List",
                  content: {
                    style: { type: "Unordered", content: "Disc" },
                    items: [
                      {
                        content: [{
                          type: "Paragraph",
                          content: {
                            segments: [
                              { type: "Text", content: "Concrete representations (matrices, permutations)" }
                            ]
                          }
                        }]
                      },
                      {
                        content: [{
                          type: "Paragraph",
                          content: {
                            segments: [
                              { type: "Text", content: "Focus on specific examples (S_n, GL_n, etc.)" }
                            ]
                          }
                        }]
                      },
                      {
                        content: [{
                          type: "Paragraph",
                          content: {
                            segments: [
                              { type: "Text", content: "Computational methods" }
                            ]
                          }
                        }]
                      }
                    ]
                  }
                }
              ],
              panel_role: "ComparisonLeft"
            },
            right_panel: {
              id: "modern-approach",
              title: { segments: [{ type: "Text", content: "🔬 Modern Approach" }] },
              content: [
                {
                  type: "Paragraph",
                  content: {
                    segments: [
                      { type: "StyledText", text: "Abstract Development:", styles: ["Bold"] },
                      { type: "Text", content: " Modern group theory emphasizes abstract structure and categorical relationships." }
                    ]
                  }
                },
                {
                  type: "StructuredMath",
                  content: {
                    type: "Definition",
                    term_display: [{ type: "Text", content: "Abstract Group Definition" }],
                    body: [
                      {
                        type: "Paragraph",
                        content: {
                          segments: [
                            { type: "Text", content: "A group (G, ·) is a set G with a binary operation · satisfying:" }
                          ]
                        }
                      },
                      {
                        type: "MathBlock",
                        math: { content: mathContent.groupAxioms, display_style: true }
                      }
                    ]
                  }
                },
                {
                  type: "Paragraph",
                  content: {
                    segments: [
                      { type: "StyledText", text: "Key Features:", styles: ["Bold"] }
                    ]
                  }
                },
                {
                  type: "List",
                  content: {
                    style: { type: "Unordered", content: "Disc" },
                    items: [
                      {
                        content: [{
                          type: "Paragraph",
                          content: {
                            segments: [
                              { type: "Text", content: "Abstract axiomatization" }
                            ]
                          }
                        }]
                      },
                      {
                        content: [{
                          type: "Paragraph",
                          content: {
                            segments: [
                              { type: "Text", content: "Category theory connections" }
                            ]
                          }
                        }]
                      },
                      {
                        content: [{
                          type: "Paragraph",
                          content: {
                            segments: [
                              { type: "Text", content: "Homological methods" }
                            ]
                          }
                        }]
                      }
                    ]
                  }
                }
              ],
              panel_role: "ComparisonRight"
            },
            sync_scrolling: true,
            highlight_correspondence: true
          }
        },
        {
          type: "Paragraph",
          content: {
            segments: [
              { type: "StyledText", text: "Synthesis:", styles: ["Bold"] },
              { type: "Text", content: " Modern mathematics benefits from both approaches - concrete examples provide intuition while abstract methods reveal deep structural connections." }
            ]
          }
        }
      ]
    }
  ],
  bibliography: [],
  presentation_config: {
    layout_style: "TwoColumn",
    interaction_features: ["HighlightCorrespondence", "ClickableLinks"],
    target_audience: "Graduate",
    formality_level: "SemiFormal"
  }
};

const typeMappingDisplayDocument: FlexibleDocument = {
  id: "comprehensive-type-mapping-display",
  title: "Type Theory: Functions and Their Types",
  document_type: "TypeMappingDisplay",
  language: "en",
  version: "1.0",
  body: [
    {
      id: "type-mappings",
      title: { segments: [{ type: "Text", content: "Function Type Mappings" }] },
      content: [
        {
          type: "AnnotationOverlay",
          content: {
            base_content: [
              {
                type: "MathBlock",
                math: { content: "f: ℕ → ℕ", display_style: true }
              },
              {
                type: "MathBlock",
                math: { content: "f(x) = x²", display_style: true }
              }
            ],
            annotations: [
              {
                id: "domain-annotation",
                target_selector: "ℕ",
                annotation_content: [
                  { type: "Text", content: "Domain: Natural numbers" }
                ],
                annotation_type: "TypeInfo",
                styling: {
                  color: "#2563eb",
                  background_color: "#eff6ff"
                }
              },
              {
                id: "codomain-annotation", 
                target_selector: "→ ℕ",
                annotation_content: [
                  { type: "Text", content: "Codomain: Natural numbers" }
                ],
                annotation_type: "TypeInfo",
                styling: {
                  color: "#dc2626",
                  background_color: "#fef2f2"
                }
              }
            ],
            overlay_style: "Tooltip"
          }
        },
        {
          type: "Paragraph",
          content: {
            segments: [
              { type: "Text", content: "This function maps each natural number to its square. The type signature " },
              { type: "Math", content: { content: "ℕ → ℕ", display_style: false } },
              { type: "Text", content: " tells us both the input and output types." }
            ]
          }
        },
        {
          type: "StructuredMath",
          content: {
            type: "Definition",
            term_display: [{ type: "Text", content: "Function Type" }],
            formal_term: { content: "A → B", display_style: false },
            body: [
              {
                type: "Paragraph",
                content: {
                  segments: [
                    { type: "Text", content: "A function type " },
                    { type: "Math", content: { content: "A → B", display_style: false } },
                    { type: "Text", content: " represents the set of all functions from type A to type B." }
                  ]
                }
              }
            ]
          }
        },
        {
          type: "Table",
          content: {
            caption: { segments: [{ type: "Text", content: "Common Function Types" }] },
            header_rows: [{
              cells: [
                { content: [{ type: "Paragraph", content: { segments: [{ type: "Text", content: "Function" }] } }], cell_type: "Header" },
                { content: [{ type: "Paragraph", content: { segments: [{ type: "Text", content: "Type" }] } }], cell_type: "Header" },
                { content: [{ type: "Paragraph", content: { segments: [{ type: "Text", content: "Description" }] } }], cell_type: "Header" }
              ]
            }],
            body_rows: [
              {
                cells: [
                  { content: [{ type: "MathBlock", math: { content: "sin", display_style: false } }], cell_type: "Data" },
                  { content: [{ type: "MathBlock", math: { content: "ℝ → ℝ", display_style: false } }], cell_type: "Data" },
                  { content: [{ type: "Paragraph", content: { segments: [{ type: "Text", content: "Sine function" }] } }], cell_type: "Data" }
                ]
              },
              {
                cells: [
                  { content: [{ type: "MathBlock", math: { content: "length", display_style: false } }], cell_type: "Data" },
                  { content: [{ type: "MathBlock", math: { content: "String → ℕ", display_style: false } }], cell_type: "Data" },
                  { content: [{ type: "Paragraph", content: { segments: [{ type: "Text", content: "String length function" }] } }], cell_type: "Data" }
                ]
              }
            ]
          }
        }
      ]
    }
  ],
  bibliography: [],
  presentation_config: {
    layout_style: "SingleColumn",
    interaction_features: ["TypeAnnotations", "HoverTooltips"],
    target_audience: "Graduate",
    formality_level: "SemiFormal"
  }
};

const transformationMappingDocument: FlexibleDocument = {
  id: "comprehensive-transformation-mapping",
  title: "Linear Transformations: Matrix Representations",
  document_type: "TransformationMapping",
  language: "en",
  version: "1.0",
  body: [
    {
      id: "transformation-mapping",
      title: { segments: [{ type: "Text", content: "From Abstract to Concrete" }] },
      content: [
        {
          type: "SideBySideLayout",
          content: {
            left_panel: {
              id: "abstract-transformation",
              title: { segments: [{ type: "Text", content: "Abstract Linear Transformation" }] },
              content: [
                {
                  type: "StructuredMath",
                  content: {
                    type: "Definition",
                    term_display: [{ type: "Text", content: "Linear Transformation" }],
                    formal_term: { content: "T: V → W", display_style: false },
                    body: [
                      {
                        type: "Paragraph",
                        content: {
                          segments: [
                            { type: "Text", content: "A function T between vector spaces V and W satisfying:" }
                          ]
                        }
                      },
                      {
                        type: "List",
                        content: {
                          style: { type: "Ordered", content: "Decimal" },
                          items: [
                            {
                              content: [{
                                type: "MathBlock",
                                math: { content: "T(u + v) = T(u) + T(v)", display_style: false }
                              }]
                            },
                            {
                              content: [{
                                type: "MathBlock",
                                math: { content: "T(cu) = cT(u)", display_style: false }
                              }]
                            }
                          ]
                        }
                      }
                    ]
                  }
                },
                {
                  type: "Paragraph",
                  content: {
                    segments: [
                      { type: "StyledText", text: "Properties:", styles: ["Bold"] }
                    ]
                  }
                },
                {
                  type: "List",
                  content: {
                    style: { type: "Unordered", content: "Disc" },
                    items: [
                      {
                        content: [{
                          type: "Paragraph",
                          content: {
                            segments: [
                              { type: "Text", content: "Preserves vector addition" }
                            ]
                          }
                        }]
                      },
                      {
                        content: [{
                          type: "Paragraph",
                          content: {
                            segments: [
                              { type: "Text", content: "Preserves scalar multiplication" }
                            ]
                          }
                        }]
                      },
                      {
                        content: [{
                          type: "Paragraph",
                          content: {
                            segments: [
                              { type: "Text", content: "Maps zero vector to zero vector" }
                            ]
                          }
                        }]
                      }
                    ]
                  }
                }
              ],
              panel_role: "SourceTheory"
            },
            right_panel: {
              id: "matrix-representation",
              title: { segments: [{ type: "Text", content: "Matrix Representation" }] },
              content: [
                {
                  type: "Paragraph",
                  content: {
                    segments: [
                      { type: "Text", content: "Every linear transformation " },
                      { type: "Math", content: { content: "T: ℝⁿ → ℝᵐ", display_style: false } },
                      { type: "Text", content: " can be represented by an m×n matrix A:" }
                    ]
                  }
                },
                {
                  type: "MathBlock",
                  math: { content: "T(x) = Ax", display_style: true }
                },
                {
                  type: "Paragraph",
                  content: {
                    segments: [
                      { type: "StyledText", text: "Construction:", styles: ["Bold"] },
                      { type: "Text", content: " The columns of A are the images of the standard basis vectors." }
                    ]
                  }
                },
                {
                  type: "MathBlock",
                  math: { content: "A = [T(e₁) | T(e₂) | ... | T(eₙ)]", display_style: true }
                },
                {
                  type: "StructuredMath",
                  content: {
                    type: "Example",
                    body: [
                      {
                        type: "Paragraph",
                        content: {
                          segments: [
                            { type: "Text", content: "Rotation by 90° counterclockwise:" }
                          ]
                        }
                      },
                      {
                        type: "MathBlock",
                        math: { content: "A = [0 -1; 1 0]", display_style: true }
                      },
                      {
                        type: "Paragraph",
                        content: {
                          segments: [
                            { type: "Text", content: "This maps " },
                            { type: "Math", content: { content: "(1,0) → (0,1)", display_style: false } },
                            { type: "Text", content: " and " },
                            { type: "Math", content: { content: "(0,1) → (-1,0)", display_style: false } }
                          ]
                        }
                      }
                    ]
                  }
                }
              ],
              panel_role: "TargetTheory"
            },
            sync_scrolling: true,
            highlight_correspondence: true
          }
        }
      ]
    }
  ],
  bibliography: [],
  presentation_config: {
    layout_style: "TwoColumn",
    interaction_features: ["HighlightCorrespondence", "Animations"],
    target_audience: "Undergraduate",
    formality_level: "SemiFormal"
  }
};

const interactivePlaygroundDocument: FlexibleDocument = {
  id: "comprehensive-interactive-playground",
  title: "Function Explorer: Interactive Mathematical Playground",
  document_type: "InteractivePlayground",
  language: "en",
  version: "1.0",
  body: [
    {
      id: "function-explorer",
      title: { segments: [{ type: "Text", content: "🎮 Function Explorer" }] },
      content: [
        {
          type: "Paragraph",
          content: {
            segments: [
              { type: "Text", content: "Explore how parameters affect the quadratic function " },
              { type: "Math", content: { content: "f(x) = ax² + bx + c", display_style: false } },
              { type: "Text", content: ". Adjust the controls below to see the effects!" }
            ]
          }
        },
        {
          type: "InteractiveControls",
          content: {
            controls: [
              {
                id: "param-a",
                label: "Coefficient a",
                control_type: { type: "Slider", min: -5.0, max: 5.0, step: 0.1 },
                parameter_name: "a",
                default_value: "1.0",
                description: "Controls the opening direction and width of the parabola"
              },
              {
                id: "param-b",
                label: "Coefficient b", 
                control_type: { type: "Slider", min: -10.0, max: 10.0, step: 0.1 },
                parameter_name: "b",
                default_value: "0.0",
                description: "Controls the horizontal shift of the vertex"
              },
              {
                id: "param-c",
                label: "Coefficient c",
                control_type: { type: "Slider", min: -10.0, max: 10.0, step: 0.1 },
                parameter_name: "c",
                default_value: "0.0",
                description: "Controls the vertical shift (y-intercept)"
              },
              {
                id: "show-vertex",
                label: "Show Vertex",
                control_type: "Toggle",
                parameter_name: "show_vertex",
                default_value: "true",
                description: "Display the vertex of the parabola"
              },
              {
                id: "show-roots",
                label: "Show Roots",
                control_type: "Toggle", 
                parameter_name: "show_roots",
                default_value: "false",
                description: "Display the x-intercepts when they exist"
              }
            ],
            target_content_ids: ["quadratic-visualization"],
            layout: { type: "Grid", columns: 2 }
          }
        },
        {
          type: "InteractiveDiagram",
          content: {
            diagram_type_id: "function_plot",
            data: `{
              "function": "quadratic",
              "parameters": ["a", "b", "c"],
              "domain": [-10, 10],
              "range": [-10, 10],
              "grid": true,
              "axes": true
            }`,
            caption: { segments: [{ type: "Text", content: "Interactive quadratic function visualization" }] }
          }
        },
        {
          type: "Paragraph",
          content: {
            segments: [
              { type: "StyledText", text: "Key Observations:", styles: ["Bold"] }
            ]
          }
        },
        {
          type: "List",
          content: {
            style: { type: "Unordered", content: "Disc" },
            items: [
              {
                content: [{
                  type: "Paragraph",
                  content: {
                    segments: [
                      { type: "StyledText", text: "Parameter a:", styles: ["Bold"] },
                      { type: "Text", content: " When a > 0, parabola opens upward; when a < 0, opens downward" }
                    ]
                  }
                }]
              },
              {
                content: [{
                  type: "Paragraph",
                  content: {
                    segments: [
                      { type: "StyledText", text: "Parameter c:", styles: ["Bold"] },
                      { type: "Text", content: " Directly controls the y-intercept of the function" }
                    ]
                  }
                }]
              },
              {
                content: [{
                  type: "Paragraph",
                  content: {
                    segments: [
                      { type: "StyledText", text: "Vertex formula:", styles: ["Bold"] },
                      { type: "Text", content: " The vertex occurs at " },
                      { type: "Math", content: { content: "x = -b/(2a)", display_style: false } }
                    ]
                  }
                }]
              }
            ]
          }
        },
        {
          type: "AlertBox",
          style: "Tip",
          content: [{
            type: "Paragraph",
            content: {
              segments: [
                { type: "StyledText", text: "Try this:", styles: ["Bold"] },
                { type: "Text", content: " Set a = 1, b = 0, c = 0 to see the basic parabola y = x². Then experiment with different values!" }
              ]
            }
          }]
        },
        {
          type: "MathBlock",
          math: { content: mathContent.quadraticFormula, display_style: true },
          caption: { segments: [{ type: "Text", content: "Use this formula to find the roots when they exist" }] }
        }
      ]
    }
  ],
  bibliography: [],
  presentation_config: {
    layout_style: "Dashboard",
    interaction_features: ["ParameterAdjustment", "InteractiveControls", "Animations"],
    target_audience: "HighSchool",
    formality_level: "Intuitive"
  }
};

// Document registry mapping types to their implementations
const documentRegistry: Record<DocumentType, FlexibleDocument> = {
  "ScientificPaper": scientificPaperDocument,
  "BlogPost": blogPostDocument,
  "TooltipSummary": tooltipSummaryDocument,
  "AnimatedPresentation": animatedPresentationDocument,
  "ResourcePanel": resourcePanelDocument,
  "WikiPage": wikiPageDocument,
  "Textbook": textbookDocument,
  "PersonalNotes": personalNotesDocument,
  "MathematicianNotes": mathematicianNotesDocument,
  "StudyNotes": studyNotesDocument,
  "ComparisonPage": comparisonPageDocument,
  "TypeMappingDisplay": typeMappingDisplayDocument,
  "TransformationMapping": transformationMappingDocument,
  "InteractivePlayground": interactivePlaygroundDocument
};

const FlexibleDocumentDemo: React.FC = () => {
  const [selectedType, setSelectedType] = useState<DocumentType>('ScientificPaper');
  const [sampleDocument, setSampleDocument] = useState<FlexibleDocument>(
    documentRegistry['ScientificPaper']
  );

  const documentTypes: DocumentType[] = [
    'ScientificPaper',
    'BlogPost',
    'TooltipSummary',
    'AnimatedPresentation',
    'ResourcePanel',
    'WikiPage',
    'Textbook',
    'PersonalNotes',
    'MathematicianNotes',
    'StudyNotes',
    'ComparisonPage',
    'TypeMappingDisplay',
    'TransformationMapping',
    'InteractivePlayground'
  ];

  const handleTypeChange = (type: DocumentType) => {
    setSelectedType(type);
    setSampleDocument(documentRegistry[type]);
  };

  return (
    <div className={styles.demoContainer}>
      <div className={styles.demoHeader}>
        <h1>FlexibleDocumentRenderer Demo</h1>
        <p>Select different document types to see comprehensive, full-page examples with rich mathematical content rendered using turn-math integration.</p>
        
        <div className={styles.typeSelector}>
          <label htmlFor="document-type">Document Type:</label>
          <select
            id="document-type"
            value={selectedType}
            onChange={(e) => handleTypeChange(e.target.value as DocumentType)}
            className={styles.typeSelect}
          >
            {documentTypes.map(type => (
              <option key={type} value={type}>
                {type.replace(/([A-Z])/g, ' $1').trim()}
              </option>
            ))}
          </select>
        </div>

        <div className={styles.documentInfo}>
          <h3>Current Configuration:</h3>
          <ul>
            <li><strong>Type:</strong> {selectedType}</li>
            <li><strong>Layout:</strong> {sampleDocument.presentation_config?.layout_style || 'Default'}</li>
            <li><strong>Audience:</strong> {sampleDocument.presentation_config?.target_audience || 'General'}</li>
            <li><strong>Formality:</strong> {sampleDocument.presentation_config?.formality_level || 'Standard'}</li>
            <li><strong>Features:</strong> {sampleDocument.presentation_config?.interaction_features?.join(', ') || 'Basic'}</li>
          </ul>
        </div>
      </div>

      <div className={styles.demoContent}>
        <FlexibleDocumentRenderer 
          document={sampleDocument}
          className={styles.demoDocument}
        />
      </div>
    </div>
  );
};

export default FlexibleDocumentDemo; 