import React from 'react';
import type { RichTextSegment } from '../../turn-render/bindings/RichTextSegment';
import { LinkRenderer } from './LinkRenderer';

interface RichTextRendererProps {
  segments: RichTextSegment[];
  className?: string;
}

/**
 * RichTextRenderer that handles all types of RichTextSegment
 * including Text, StyledText, Math, Link, FootnoteReference, and CodeInline
 */
export const RichTextRenderer: React.FC<RichTextRendererProps> = ({ 
  segments, 
  className 
}) => {
  const renderSegment = (segment: RichTextSegment, index: number) => {
    if ('Text' in segment) {
      return <span key={index}>{segment.Text}</span>;
    }
    
    if ('StyledText' in segment) {
      // Apply text styles
      const styles = segment.StyledText.styles;
      let className = '';
      let style: React.CSSProperties = {};
      
      styles.forEach(styleType => {
        if (styleType === 'Bold') {
          style.fontWeight = 'bold';
        } else if (styleType === 'Italic') {
          style.fontStyle = 'italic';
        } else if (styleType === 'Underline') {
          style.textDecoration = 'underline';
        } else if (styleType === 'Strikethrough') {
          style.textDecoration = 'line-through';
        } else if (styleType === 'Subscript') {
          style.verticalAlign = 'sub';
          style.fontSize = '0.8em';
        } else if (styleType === 'Superscript') {
          style.verticalAlign = 'super';
          style.fontSize = '0.8em';
        } else if (typeof styleType === 'object') {
          // Handle object-based styles
          if ('Color' in styleType) {
            style.color = styleType.Color;
          } else if ('BackgroundColor' in styleType) {
            style.backgroundColor = styleType.BackgroundColor;
          } else if ('FontSize' in styleType) {
            style.fontSize = styleType.FontSize;
          } else if ('FontFamily' in styleType) {
            style.fontFamily = styleType.FontFamily;
          }
        }
      });
      
      return (
        <span key={index} style={style}>
          {segment.StyledText.text}
        </span>
      );
    }
    
    if ('Math' in segment) {
      // Render math content - for now just display the math as text
      // TODO: Integrate with proper math renderer
      return (
        <span key={index} className="math-inline" style={{ fontFamily: 'serif', fontStyle: 'italic' }}>
          {renderMathNode(segment.Math)}
        </span>
      );
    }
    
    if ('Link' in segment) {
      return (
        <LinkRenderer
          key={index}
          content={segment.Link.content}
          target={segment.Link.target}
          tooltip={segment.Link.tooltip}
        />
      );
    }
    
    if ('FootnoteReference' in segment) {
      return (
        <sup key={index} className="footnote-ref">
          <a href={`#footnote-${segment.FootnoteReference}`}>
            [{segment.FootnoteReference}]
          </a>
        </sup>
      );
    }
    
    if ('CodeInline' in segment) {
      return (
        <code key={index} style={{
          backgroundColor: '#f5f5f5',
          padding: '2px 4px',
          borderRadius: '3px',
          fontFamily: 'monospace',
          fontSize: '0.9em'
        }}>
          {segment.CodeInline}
        </code>
      );
    }
    
    // Fallback for unknown segment types
    return <span key={index}>[Unknown segment type]</span>;
  };

  // Simple math node renderer - this should be replaced with proper math rendering
  const renderMathNode = (mathNode: any): string => {
    if (mathNode.content) {
      if ('Text' in mathNode.content) {
        return mathNode.content.Text;
      }
      if ('Identifier' in mathNode.content) {
        const id = mathNode.content.Identifier;
        let result = id.body || '';
        if (id.post_script && 'content' in id.post_script) {
          if ('Text' in id.post_script.content) {
            result += `_${id.post_script.content.Text}`;
          } else if ('Quantity' in id.post_script.content) {
            result += `_${id.post_script.content.Quantity.number}`;
          }
        }
        return result;
      }
    }
    return '[Math]';
  };

  return (
    <span className={className}>
      {segments.map((segment, index) => renderSegment(segment, index))}
    </span>
  );
};

export default RichTextRenderer; 