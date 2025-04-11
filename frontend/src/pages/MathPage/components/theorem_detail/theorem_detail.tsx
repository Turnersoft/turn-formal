import React from "react";
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
  return (
    <div className={styles.theorem}>
      <h3>{theorem.name}</h3>

      {theorem.statement && (
        <p className={styles.statement}>
          <strong>Statement:</strong>{" "}
          <span>{renderMathText(theorem.statement)}</span>
        </p>
      )}

      {theorem.description && (
        <p className={styles.description}>
          {renderMathText(theorem.description)}
        </p>
      )}

      {theorem.proof_steps && theorem.proof_steps.length > 0 && (
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
    </div>
  );
};

export default TheoremDetail;
