import React from "react";
import { MathRender } from "../math_render/mathrender";
import styles from "./math_formula.module.scss";

interface MathFormulaProps {
  formula: string;
}

/**
 * Component to render mathematical formulas using MathRender
 */
const MathFormula: React.FC<MathFormulaProps> = ({ formula }) => {
  return (
    <div className={styles.mathFormula}>
      <MathRender formula={formula} />
    </div>
  );
};

export default MathFormula;
