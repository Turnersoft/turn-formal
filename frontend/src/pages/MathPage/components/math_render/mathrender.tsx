import React from "react";
import { MathJaxNode } from "@yozora/react-mathjax";
import styles from "./mathrender.module.scss";

interface MathRenderProps {
  formula: string;
  inline?: boolean;
}

/**
 * Component for rendering mathematical expressions using MathJax
 * @param formula - The LaTeX formula to render
 * @param inline - Whether to render inline math (default: false)
 */
export const MathRender: React.FC<MathRenderProps> = ({
  formula,
  inline = false,
}) => {
  // Check if the formula is already wrapped in delimiters
  const hasDelimiters =
    formula.startsWith("$$") ||
    formula.startsWith("\\[") ||
    formula.startsWith("\\(") ||
    formula.startsWith("$");

  // Strip delimiters if present to avoid double rendering
  let cleanFormula = formula;
  if (hasDelimiters) {
    if (formula.startsWith("$$") && formula.endsWith("$$")) {
      cleanFormula = formula.slice(2, -2);
    } else if (formula.startsWith("\\[") && formula.endsWith("\\]")) {
      cleanFormula = formula.slice(2, -2);
    } else if (formula.startsWith("\\(") && formula.endsWith("\\)")) {
      cleanFormula = formula.slice(2, -2);
    } else if (formula.startsWith("$") && formula.endsWith("$")) {
      cleanFormula = formula.slice(1, -1);
    }
  }

  return (
    <div className={inline ? styles.inlineMath : styles.displayMath}>
      <MathJaxNode inline={inline} formula={cleanFormula} />
    </div>
  );
};
