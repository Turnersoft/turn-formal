import React, { useEffect, useState } from "react";
import { MathNode } from "../turn-render/bindings/MathNode";
import { MathSpan } from "../turn-render/turn-math";
import styles from "./theorem_renderer.module.scss";

// Define the type required by MathSpan based on the errors
type TurnTextLineNode =
  | "Empty"
  | { Math: [MathNode, string] }
  | { Phrase: string }
  | { Comment: string }
  | { Latex: string }
  | { PageLink: string }
  | { Image: string };

interface TheoremProps {
  id: string;
  name: string;
  statement: MathNode;
  proof?: ProofStep[];
}

interface ProofStep {
  id: string;
  content: MathNode;
}

const TheoremRenderer: React.FC<TheoremProps> = ({
  id,
  name,
  statement,
  proof,
}) => {
  const [expanded, setExpanded] = useState(false);

  // Convert MathNode to a format suitable for MathSpan
  const convertNodeForMathSpan = (node: MathNode): TurnTextLineNode[] => {
    // Create a proper tuple with MathNode and empty string for explanation
    return [{ Math: [node, ""] as [MathNode, string] }];
  };

  return (
    <div className={styles.theoremContainer}>
      <div className={styles.theoremHeader}>
        <h3>{name}</h3>
        <span className={styles.theoremId}>(ID: {id})</span>
      </div>

      <div className={styles.theoremStatement}>
        <div className={styles.theoremLabel}>Statement:</div>
        <div style={{ padding: "10px" }}>
          <MathSpan spanData={convertNodeForMathSpan(statement)} />
        </div>
      </div>

      {proof && proof.length > 0 && (
        <div className={styles.proofSection}>
          <button
            className={styles.proofToggle}
            onClick={() => setExpanded(!expanded)}
          >
            {expanded ? "Hide Proof" : "Show Proof"}
          </button>

          {expanded && (
            <div className={styles.proofSteps}>
              {proof.map((step, index) => (
                <div key={step.id} className={styles.proofStep}>
                  <div className={styles.proofStepNumber}>{index + 1}.</div>
                  <div className={styles.proofStepContent}>
                    <div style={{ margin: "5px 0" }}>
                      <MathSpan
                        spanData={convertNodeForMathSpan(step.content)}
                      />
                    </div>
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>
      )}
    </div>
  );
};

// Example component showing how the TheoremRenderer would be used
export const TheoremCollection: React.FC = () => {
  const [theorems, setTheorems] = useState<TheoremProps[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    // In a real implementation, this would fetch theorems from the API
    // For now, we'll use a placeholder example
    const fetchTheorems = async () => {
      try {
        // Example placeholder data - in a real implementation,
        // this would come from our Rust converter
        const exampleTheorem: TheoremProps = {
          id: "pythagorean",
          name: "Pythagorean Theorem",
          statement: {
            id: "stmt_1",
            content: {
              Relationship: {
                lhs: {
                  id: "lhs_1",
                  content: {
                    Power: {
                      base: {
                        id: "base_1",
                        content: {
                          Identifier: {
                            body: "c",
                            is_function: false,
                            post_script: null,
                            pre_script: null,
                            primes: 0,
                            mid_script: null,
                          },
                        },
                      },
                      exponent: {
                        id: "exp_1",
                        content: {
                          Text: "2",
                        },
                      },
                    },
                  },
                },
                rhs: {
                  id: "rhs_1",
                  content: {
                    Additions: {
                      terms: [
                        [
                          "Addition",
                          {
                            id: "term_1",
                            content: {
                              Power: {
                                base: {
                                  id: "base_2",
                                  content: {
                                    Identifier: {
                                      body: "a",
                                      is_function: false,
                                      post_script: null,
                                      pre_script: null,
                                      primes: 0,
                                      mid_script: null,
                                    },
                                  },
                                },
                                exponent: {
                                  id: "exp_2",
                                  content: {
                                    Text: "2",
                                  },
                                },
                              },
                            },
                          },
                        ],
                        [
                          "Addition",
                          {
                            id: "term_2",
                            content: {
                              Power: {
                                base: {
                                  id: "base_3",
                                  content: {
                                    Identifier: {
                                      body: "b",
                                      is_function: false,
                                      post_script: null,
                                      pre_script: null,
                                      primes: 0,
                                      mid_script: null,
                                    },
                                  },
                                },
                                exponent: {
                                  id: "exp_3",
                                  content: {
                                    Text: "2",
                                  },
                                },
                              },
                            },
                          },
                        ],
                      ],
                    },
                  },
                },
                operator: "Equal",
              },
            },
          },
          proof: [
            {
              id: "step_1",
              content: {
                id: "content_1",
                content: {
                  Text: "Consider a right triangle with sides a, b and hypotenuse c.",
                },
              },
            },
            {
              id: "step_2",
              content: {
                id: "content_2",
                content: {
                  Relationship: {
                    lhs: {
                      id: "lhs_2",
                      content: {
                        Power: {
                          base: {
                            id: "base_4",
                            content: {
                              Identifier: {
                                body: "c",
                                is_function: false,
                                post_script: null,
                                pre_script: null,
                                primes: 0,
                                mid_script: null,
                              },
                            },
                          },
                          exponent: {
                            id: "exp_4",
                            content: {
                              Text: "2",
                            },
                          },
                        },
                      },
                    },
                    rhs: {
                      id: "rhs_2",
                      content: {
                        Additions: {
                          terms: [
                            [
                              "Addition",
                              {
                                id: "term_3",
                                content: {
                                  Power: {
                                    base: {
                                      id: "base_5",
                                      content: {
                                        Identifier: {
                                          body: "a",
                                          is_function: false,
                                          post_script: null,
                                          pre_script: null,
                                          primes: 0,
                                          mid_script: null,
                                        },
                                      },
                                    },
                                    exponent: {
                                      id: "exp_5",
                                      content: {
                                        Text: "2",
                                      },
                                    },
                                  },
                                },
                              },
                            ],
                            [
                              "Addition",
                              {
                                id: "term_4",
                                content: {
                                  Power: {
                                    base: {
                                      id: "base_6",
                                      content: {
                                        Identifier: {
                                          body: "b",
                                          is_function: false,
                                          post_script: null,
                                          pre_script: null,
                                          primes: 0,
                                          mid_script: null,
                                        },
                                      },
                                    },
                                    exponent: {
                                      id: "exp_6",
                                      content: {
                                        Text: "2",
                                      },
                                    },
                                  },
                                },
                              },
                            ],
                          ],
                        },
                      },
                    },
                    operator: "Equal",
                  },
                },
              },
            },
          ],
        };

        setTheorems([exampleTheorem]);
        setLoading(false);
      } catch (err) {
        console.error("Error fetching theorems:", err);
        setError("Failed to load theorems");
        setLoading(false);
      }
    };

    fetchTheorems();
  }, []);

  if (loading) {
    return <div>Loading theorems...</div>;
  }

  if (error) {
    return <div className={styles.error}>{error}</div>;
  }

  return (
    <div className={styles.theoremsContainer}>
      <h2>Mathematical Theorems</h2>
      {theorems.map((theorem) => (
        <TheoremRenderer
          key={theorem.id}
          id={theorem.id}
          name={theorem.name}
          statement={theorem.statement}
          proof={theorem.proof}
        />
      ))}
    </div>
  );
};

export default TheoremRenderer;
