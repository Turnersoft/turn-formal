import { useState, useEffect } from "react";
import { Theorem, ProofStep, MathNode } from "../../models/math";
import '@shoelace-style/shoelace/dist/components/icon/icon.js';
import styles from "./theorem_detail.module.css";

type TheoremDetailProps = {
  theorem: Theorem;
  index: number;
};

/**
 * Display a theorem with its details
 */
export default function TheoremDetail({ theorem, index }: TheoremDetailProps) {
  const [expanded, setExpanded] = useState(false);
  const [extractedProofSteps, setExtractedProofSteps] = useState<(ProofStep | MathNode | string)[]>([]);

  // Extract proof steps whenever the theorem or expanded state changes
  useEffect(() => {
    if (expanded && theorem) {
      try {
        let proofSteps: (ProofStep | MathNode | string)[] = [];

        // First check if we have proof steps directly on the theorem
        if (theorem.proof_steps && theorem.proof_steps.length > 0) {
          proofSteps = theorem.proof_steps;
        } 
        // Then check if we have them in the nested content structure
        else if (theorem.content?.Theorem?.proof_steps) {
          proofSteps = theorem.content.Theorem.proof_steps;
        }
        // Then check for the deeply nested path
        else if (
          theorem.content?.Theorem?.initial_proof_state?.content?.Theorem?.proof_steps
        ) {
          proofSteps = theorem.content.Theorem.initial_proof_state.content.Theorem.proof_steps;
        }

        setExtractedProofSteps(proofSteps);
      } catch (error: any) {
        console.error("Error extracting proof steps:", error);
      }
    }
  }, [theorem, expanded]);

  /**
   * Get the theorem statement
   */
  const getTheoremStatement = (): string => {
    if (!theorem) return "";

    // If the theorem has a statement, use that
    if (theorem.statement) {
      return theorem.statement;
    }
    
    // Check in nested structure
    if (theorem.content?.Theorem?.initial_proof_state?.content?.ProofState?.statement?.content?.Text) {
      return theorem.content.Theorem.initial_proof_state.content.ProofState.statement.content.Text;
    }

    // Otherwise, use the name
    return theorem.name || "Unnamed theorem";
  };

  /**
   * Render a proof step with formatted content
   */
  const renderProofStep = (step: ProofStep | MathNode | string, _stepIndex: number) => {
    // For string steps, just render them directly
    if (typeof step === 'string') {
      return (
        <div>
          <div className={styles.stepContent}>{step}</div>
        </div>
      );
    }
    
    // Handle ProofStep type
    if ('description' in step) {
      return (
        <div>
          <div className={styles.stepContent}>{step.description}</div>
        </div>
      );
    }
    
    // Handle MathNode type
    if ('content' in step) {
      // Extract statement text
      let statementText = '';
      if (step.content.Text) {
        statementText = step.content.Text;
      } else if (step.content.ProofState?.statement?.content?.Text) {
        statementText = step.content.ProofState.statement.content.Text;
      }
      
      return (
        <div>
          {statementText && <div className={styles.stepContent}>{statementText}</div>}
          {!statementText && (
            <div className={styles.stepContent}>
              {JSON.stringify(step).substring(0, 100) + '...'}
            </div>
          )}
        </div>
      );
    }
    
    // Default case
    return <div className={styles.stepContent}>{JSON.stringify(step).substring(0, 100) + '...'}</div>;
  };

  /**
   * Handle expanding the theorem
   */
  const handleExpandClick = () => {
    setExpanded(!expanded);
  };

  return (
    <div className={styles.theoremContainer}>
      <div className={styles.theoremHeader} onClick={handleExpandClick}>
        <div className={styles.expandIcon}>
          {expanded ? 
            <sl-icon name="chevron-down"></sl-icon> : 
            <sl-icon name="chevron-right"></sl-icon>
          }
                </div>
        <div className={styles.theoremTitle}>
          <strong>{theorem.name || `Theorem ${index + 1}`}:</strong>{" "}
          {getTheoremStatement()}
                  </div>
                  </div>

      {expanded && (
        <div className={styles.theoremDetails}>
          {theorem.description && (
            <div className={styles.theoremDescription}>{theorem.description}</div>
          )}

          {extractedProofSteps.length > 0 ? (
            <div className={styles.proofSteps}>
              <h4>Proof Steps:</h4>
              <ol>
                {extractedProofSteps.map((step, stepIndex) => (
                  <li key={stepIndex} className={styles.proofStep}>
                    {renderProofStep(step, stepIndex)}
              </li>
            ))}
          </ol>
        </div>
          ) : (
            <div className={styles.noProofSteps}>
              No formal proof steps available for this theorem
        </div>
      )}
        </div>
      )}
    </div>
  );
}
