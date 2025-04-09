import React, { useEffect } from "react";
import styles from "./MathContent.module.scss";
import {
  MathDefinition,
  MathTheorem,
  ProofStep,
} from "../../../../types/mathContent";

interface MathContentProps {
  content: any;
  loading: boolean;
  error: string | null;
  onNavigate: (path: string) => void;
}

// Sample data for development, will be replaced with actual data from Rust
const sampleData = {
  definitions: [
    {
      id: "group",
      name: "Group",
      description:
        "A group is a set G equipped with a binary operation that combines any two elements to form a third element.",
      type: "struct",
      fields: [
        {
          name: "base_set",
          type: "Set",
          description: "The underlying set",
        },
        {
          name: "operation",
          type: "GroupOperation",
          description: "The binary operation",
        },
      ],
    },
  ],
  theorems: [
    {
      id: "inverse_uniqueness",
      name: "Group Inverse Uniqueness",
      statement:
        "For all elements g in a group G, if g*h₁ = e and g*h₂ = e, then h₁ = h₂",
      description: "This theorem proves that inverses in a group are unique",
      proofSteps: [
        {
          id: "step_1",
          description: "Assume g*h₁ = e and g*h₂ = e",
          formula: "g*h₁ = e, g*h₂ = e",
        },
        {
          id: "step_2",
          description: "Multiply the first equation by h₂ on the left",
          formula: "h₂*(g*h₁) = h₂*e",
        },
      ],
    },
  ],
};

const MathContentComponent: React.FC<MathContentProps> = ({
  content,
  loading,
  error,
  onNavigate,
}) => {
  // Render MathJax whenever the content changes
  useEffect(() => {
    if (!loading && window.MathJax) {
      window.MathJax.typeset();
    }
  }, [content, loading]);

  // Function to render a definition
  const renderDefinition = (definition: MathDefinition) => {
    return (
      <div
        key={definition.id}
        className={`${styles.mathCard} ${styles.definition}`}
      >
        <h2>{definition.name}</h2>
        <div className={styles.description}>{definition.description}</div>

        {definition.type === "enum" && definition.variants && (
          <div className={styles.enumVariants}>
            <h3>Variants</h3>
            <ul>
              {definition.variants.map((variant) => (
                <li key={variant.id}>
                  <strong>{variant.name}</strong> - {variant.description}
                </li>
              ))}
            </ul>
          </div>
        )}

        {definition.fields && (
          <div className={styles.structFields}>
            <h3>Fields</h3>
            <table>
              <thead>
                <tr>
                  <th>Name</th>
                  <th>Type</th>
                  <th>Description</th>
                </tr>
              </thead>
              <tbody>
                {definition.fields.map((field, index) => (
                  <tr key={index}>
                    <td>
                      <code>{field.name}</code>
                    </td>
                    <td>
                      <code>{field.type}</code>
                    </td>
                    <td>{field.description}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </div>
    );
  };

  // Function to render a theorem
  const renderTheorem = (theorem: MathTheorem) => {
    return (
      <div key={theorem.id} className={`${styles.mathCard} ${styles.theorem}`}>
        <h2>{theorem.name}</h2>
        <div className={styles.statement}>
          <strong>Statement:</strong>{" "}
          <span dangerouslySetInnerHTML={{ __html: theorem.statement }} />
        </div>
        <p>{theorem.description}</p>

        <div className={styles.proofSection}>
          <h3>Proof</h3>
          <ol className={styles.proofSteps}>
            {theorem.proofSteps.map((step: ProofStep) => (
              <li key={step.id} className={styles.proofStep}>
                <div className={styles.stepDescription}>{step.description}</div>
                {step.formula && (
                  <div className={styles.formula}>
                    {"\\(" + step.formula + "\\)"}
                  </div>
                )}
                {step.justification && (
                  <div className={styles.justification}>
                    <em>Justification:</em> {step.justification}
                  </div>
                )}
              </li>
            ))}
          </ol>
        </div>

        {theorem.tags && theorem.tags.length > 0 && (
          <div className={styles.tags}>
            {theorem.tags.map((tag) => (
              <span key={tag} className={styles.tag}>
                {tag}
              </span>
            ))}
          </div>
        )}
      </div>
    );
  };

  if (loading) {
    return (
      <div className={styles.loadingIndicator}>
        Loading mathematics content...
      </div>
    );
  }

  if (error) {
    return (
      <div className={styles.errorMessage}>
        {error}
        <button onClick={() => onNavigate("/")}>Go to Home</button>
      </div>
    );
  }

  // Use the actual content if available, otherwise use the sample data
  const mathContent = content || sampleData;

  return (
    <div className={styles.contentArea}>
      <div className={styles.contentHeader}>
        <h1>Mathematics Content</h1>
      </div>

      <div className={styles.mainContent}>
        <div className={styles.contentGrid}>
          {mathContent.theorems && mathContent.theorems.length > 0 ? (
            <>
              <h2 className={styles.sectionHeader}>Theorems</h2>
              {mathContent.theorems.map((theorem: MathTheorem) =>
                renderTheorem(theorem)
              )}
            </>
          ) : null}

          {mathContent.definitions && mathContent.definitions.length > 0 ? (
            <>
              <h2 className={styles.sectionHeader}>Definitions</h2>
              {mathContent.definitions.map((definition: MathDefinition) =>
                renderDefinition(definition)
              )}
            </>
          ) : null}

          {(!mathContent.theorems || mathContent.theorems.length === 0) &&
            (!mathContent.definitions ||
              mathContent.definitions.length === 0) && (
              <div className={styles.emptyState}>
                No content found in this directory. Please select another
                folder.
              </div>
            )}
        </div>
      </div>
    </div>
  );
};

export default MathContentComponent;
