import React, { useState } from "react";
import styles from "./theorem_detail.module.scss";
import { Theorem, ProofStep } from "../../models/math";
import MathFormula from "../math_formula/math_formula";
import { renderMathText } from "../../utils/formatters";

interface TheoremDetailProps {
  theorem: Theorem;
}

/**
 * Component for rendering a detailed view of a theorem
 */
const TheoremDetail: React.FC<TheoremDetailProps> = ({ theorem }) => {
  const [expanded, setExpanded] = useState(false);
  
  // Function to format the statement for better readability
  const formatStatement = (statement: string): string => {
    if (!statement) return "";
    
    // If it's a long complex statement from the theorem export, try to simplify it
    if (statement.length > 200 && statement.includes("GroupOperation")) {
      // Extract key parts from the complex statement
      if (statement.includes("Group Inverse")) {
        return "For any element g in a group, its inverse is unique.";
      } else if (statement.includes("Identity Element")) {
        return "In a group, the identity element is unique.";
      } else if (statement.includes("Inverse Product Rule")) {
        return "For elements a and b in a group, (a·b)⁻¹ = b⁻¹·a⁻¹";
      } else if (statement.includes("Abelian")) {
        return "A group is abelian if and only if (a·b)·(a·b) = (a·a)·(b·b) for all a,b in the group.";
      } else if (statement.includes("Lagrange")) {
        return "If H is a subgroup of a finite group G, then the order of H divides the order of G.";
      }
      
      // Generic simplification for other complex statements
      return "A theorem about group properties.";
    }
    
    return statement;
  };

  return (
    <div className={styles.theorem}>
      <h3 onClick={() => setExpanded(!expanded)} className={styles.theoremTitle}>
        {theorem.name}
        {theorem.id && <span className={styles.theoremId}>(ID: {theorem.id})</span>}
      </h3>

      {theorem.statement && (
        <p className={styles.statement}>
          <strong>Statement:</strong>{" "}
          <span>{renderMathText(formatStatement(theorem.statement))}</span>
        </p>
      )}

      {theorem.description && (
        <p className={styles.description}>
          {renderMathText(theorem.description)}
        </p>
      )}

      {expanded && theorem.proof_steps && theorem.proof_steps.length > 0 && (
        <div className={styles.proof}>
          <h4>Proof</h4>
          <ol className={styles.proofSteps}>
            {theorem.proof_steps.map((step: ProofStep, idx: number) => (
              <li key={idx} className={styles.proofStep}>
                <div className={styles.stepDescription}>
                  {renderMathText(step.description)}
                </div>
                {step.formula && (
                  <div className={styles.stepFormula}>
                    <MathFormula formula={step.formula} />
                  </div>
                )}
                {step.justification && (
                  <div className={styles.stepJustification}>
                    <em>Justification: </em>
                    {renderMathText(step.justification)}
                  </div>
                )}
              </li>
            ))}
          </ol>
        </div>
      )}

      {theorem.tags && theorem.tags.length > 0 && (
        <div className={styles.tags}>
          {theorem.tags.map((tag: string, idx: number) => (
            <span key={idx} className={styles.tag}>
              {tag}
            </span>
          ))}
        </div>
      )}
      
      {expanded && (
        <div className={styles.showOriginal}>
          <button 
            className={styles.showOriginalButton}
            onClick={(e) => {
              e.stopPropagation();
              alert(theorem.statement);
            }}
          >
            Show Original Statement
          </button>
        </div>
      )}
    </div>
  );
};

export default TheoremDetail;
