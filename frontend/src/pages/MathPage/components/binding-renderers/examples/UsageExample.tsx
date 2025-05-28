import React, { useEffect, useState } from 'react';
import type { MathematicalContent } from '../../turn-render/bindings/MathematicalContent';
import type { MathNode } from '../../turn-render/bindings/MathNode';
import { DocumentRenderer } from '../core/DocumentRenderer';
import { renderMathNode } from '../../turn-render/turn-math';
import styles from './UsageExample.module.css';

/**
 * Example component demonstrating the new binding-based architecture
 * 
 * Flow: Services → Bindings → Components
 */
export const UsageExample: React.FC = () => {
  const [content, setContent] = useState<MathematicalContent | null>(null);
  const [mathNodes, setMathNodes] = useState<MathNode[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, _setError] = useState<string | null>(null);

  useEffect(() => {
    loadExampleContent();
  }, []);

  const loadExampleContent = async () => {
    // Simulate loading - in real implementation, this would use the service
    // that returns exact binding types from the unified export JSON
    setTimeout(() => {
      setContent(createSampleMathDocument());
      setMathNodes(createSampleMathNodes());
      setLoading(false);
    }, 500);
  };

  const createSampleMathNodes = (): MathNode[] => {
    // Create example math nodes using exact binding structure
    return [
      {
        id: "example-identifier",
        content: {
          Identifier: {
            body: "x",
            pre_script: null,
            mid_script: null,
            post_script: null,
            primes: 0,
            is_function: false
          }
        }
      },
      {
        id: "example-power",
        content: {
          Power: {
            base: {
              id: "base",
              content: { Identifier: { body: "a", pre_script: null, mid_script: null, post_script: null, primes: 0, is_function: false } }
            },
            exponent: {
              id: "exp", 
              content: { Quantity: { number: "2", unit: null } }
            }
          }
        }
      },
      {
        id: "example-relationship",
        content: {
          Relationship: {
            lhs: {
              id: "lhs",
              content: { Identifier: { body: "H", pre_script: null, mid_script: null, post_script: null, primes: 0, is_function: false } }
            },
            rhs: {
              id: "rhs", 
              content: { Identifier: { body: "G", pre_script: null, mid_script: null, post_script: null, primes: 0, is_function: false } }
            },
            operator: "IsSubgroupOf"
          }
        }
      }
    ];
  };

  if (loading) {
    return <div className={styles.loading}>Loading mathematical content...</div>;
  }

  if (error) {
    return <div className={styles.error}>Error: {error}</div>;
  }

  return (
    <div className={styles.container}>
      <header className={styles.header}>
        <h1>Binding-Based Component Architecture Demo</h1>
        <p>
          This example shows how components directly use auto-generated Rust binding types,
          creating a type-safe flow from services to rendering.
        </p>
      </header>

      <section className={styles.section}>
        <h2>Full Document Rendering</h2>
        <p>
          The <code>DocumentRenderer</code> directly accepts <code>MathematicalContent</code> binding type:
        </p>
        <div className={styles.documentExample}>
          {content && <DocumentRenderer content={content} />}
        </div>
      </section>

      <section className={styles.section}>
        <h2>Individual Math Node Rendering</h2>
        <p>
          Each <code>MathNodeRenderer</code> directly accepts <code>MathNode</code> binding type:
        </p>
        <div className={styles.mathExamples}>
          {mathNodes.map((node, index) => (
            <div key={index} className={styles.mathExample}>
              <div className={styles.mathRender}>
                {renderMathNode(node)}
              </div>
              <div className={styles.mathDebug}>
                <details>
                  <summary>Binding Data</summary>
                  <pre>{JSON.stringify(node, null, 2)}</pre>
                </details>
              </div>
            </div>
          ))}
        </div>
      </section>

      <section className={styles.section}>
        <h2>Architecture Benefits</h2>
        <ul className={styles.benefits}>
          <li>✅ <strong>Type Safety:</strong> Components guaranteed to match Rust types</li>
          <li>✅ <strong>Auto-Updates:</strong> Rust changes automatically update TypeScript</li>
          <li>✅ <strong>No Duplication:</strong> Single source of truth for type definitions</li>
          <li>✅ <strong>Service Integration:</strong> Services provide exact binding types</li>
          <li>✅ <strong>Performance:</strong> No runtime type transformation</li>
        </ul>
      </section>
    </div>
  );
};

// Helper function to create sample MathematicalContent with exact binding structure
function createSampleMathDocument(title: string = "Group Theory Basics"): MathematicalContent {
  return {
    id: `sample-${title.toLowerCase().replace(/\s+/g, '-')}`,
    content_type: {
      ScientificPaper: {
        title,
        paper_type: "Research",
        venue: "Mathematical Foundations",
        peer_reviewed: true,
        content_metadata: {
          language: "en-US",
          version: "1.0",
          created_at: null,
          last_modified: null,
          content_hash: null
        },
        academic_metadata: {
          authors: ["Turn Formal System"],
          date_published: null,
          date_modified: null,
          venue: "Mathematical Foundations",
          doi: null,
          keywords: ["group-theory", "mathematics"]
        },
        structure: {
          abstract_content: {
            id: "abstract",
            title: null,
            content: [
              {
                Paragraph: {
                  segments: [
                    { Text: "An introduction to group theory fundamentals." }
                  ],
                  alignment: null
                }
              }
            ],
            metadata: [],
            display_options: null
          },
          table_of_contents: null,
          body: [
            {
              id: "main-section",
              title: {
                segments: [{ Text: "Definition" }],
                alignment: null
              },
              content: [
                {
                  MathBlock: {
                    math: {
                      id: "group-def",
                      content: {
                        Text: "A group (G, ∘) is a set G with a binary operation ∘"
                      }
                    },
                    label: "Definition 1",
                    caption: null
                  }
                }
              ],
              metadata: [],
              display_options: null
            }
          ],
          footnotes: [],
          glossary: [],
          bibliography: []
        },
        relationships: {
          parent_documents: [],
          child_documents: [],
          related_concepts: [],
          cross_references: [],
          dependency_graph: null
        }
      }
    }
  };
}

export default UsageExample; 