import React from 'react';
import { MathNode } from './types';
import { MathSpan } from '../turn-render/turn-math';
import { TurnTextLineNode } from '../turn-render/bindings/TurnTextLineNode';
import { MathNode as TurnMathNode } from '../turn-render/bindings/MathNode';
import styles from './styles.module.css';

interface MathRendererProps {
  math: MathNode;
  inline?: boolean;
  className?: string;
}

export const MathRenderer: React.FC<MathRendererProps> = ({ 
  math, 
  inline = false, 
  className 
}) => {
  const mathClass = `${styles.math} ${inline ? styles.mathInline : styles.mathBlock} ${className || ''}`;
  
  // Simple math node creation for basic mathematical text
  const createSimpleMathNode = (content: string): TurnMathNode => {
    return {
      id: 'math-' + Math.random().toString(36).substr(2, 9),
      content: {
        String: content
      }
    };
  };

  // Convert to TurnTextLineNode for MathSpan
  const convertToTurnTextLine = (content: string): TurnTextLineNode[] => {
    // For simple mathematical content, we'll use the String variant
    // This provides basic rendering through turn-math without complex parsing
    if (content && content.trim()) {
      // Check if content looks like mathematical notation
      const hasMathSymbols = /[∫∑∏√±×÷≤≥≠≈∞π∂∇∀∃∈∉⊂⊃∪∩∧∨¬→↔⇒⇔]/.test(content);
      const hasGreekLetters = /[αβγδεζηθικλμνξοπρστυφχψω]/.test(content);
      const hasSupSub = /[_^]/.test(content);
      const hasParentheses = /[()[\]{}]/.test(content);
      
      if (hasMathSymbols || hasGreekLetters || hasSupSub || hasParentheses) {
        // Create a Math node for mathematical content
        const mathNode = createSimpleMathNode(content);
        const mathLineNode: TurnTextLineNode = {
          Math: [mathNode, ''] // Math content with empty explanation
        };
        return [mathLineNode];
      } else {
        // For non-mathematical text, use Phrase
        const phraseLineNode: TurnTextLineNode = {
          Phrase: content
        };
        return [phraseLineNode];
      }
    }
    
    // Fallback to empty phrase
    const phraseLineNode: TurnTextLineNode = {
      Phrase: content || ''
    };
    return [phraseLineNode];
  };

  // Main rendering logic
  if (math.content && math.content.trim()) {
    try {
      const turnTextLines = convertToTurnTextLine(math.content);
      
      return (
        <span 
          className={mathClass}
          data-math-content={math.content}
          data-display-style={math.display_style}
        >
          <MathSpan spanData={turnTextLines} />
        </span>
      );
    } catch (error) {
      console.warn('Failed to render with turn-math, falling back to text:', error);
    }
  }
  
  // Fallback to simple text rendering
  return (
    <span 
      className={mathClass}
      data-math-content={math.content}
      data-display-style={math.display_style}
    >
      {math.content}
    </span>
  );
}; 