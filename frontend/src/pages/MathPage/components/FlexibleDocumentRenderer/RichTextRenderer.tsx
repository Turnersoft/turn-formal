import React from 'react';
import { RichTextSegment, TextStyle, LinkTarget } from './types';
import { MathRenderer } from './MathRenderer.tsx';
import styles from './styles.module.css';

interface RichTextRendererProps {
  segments: RichTextSegment[];
  className?: string;
}

export const RichTextRenderer: React.FC<RichTextRendererProps> = ({ 
  segments, 
  className 
}) => {
  return (
    <span className={className}>
      {segments.map((segment, index) => (
        <RichTextSegmentRenderer key={index} segment={segment} />
      ))}
    </span>
  );
};

interface RichTextSegmentRendererProps {
  segment: RichTextSegment;
}

const RichTextSegmentRenderer: React.FC<RichTextSegmentRendererProps> = ({ segment }) => {
  switch (segment.type) {
    case 'Text':
      return <span>{segment.content}</span>;
      
    case 'StyledText':
      return (
        <span 
          className={getStyleClasses(segment.styles)}
          style={getInlineStyles(segment.styles)}
        >
          {segment.text}
        </span>
      );
      
    case 'Math':
      return <MathRenderer math={segment.content} inline={true} />;
      
    case 'Link':
      return (
        <LinkRenderer 
          target={segment.target} 
          tooltip={segment.tooltip}
        >
          <RichTextRenderer segments={segment.content} />
        </LinkRenderer>
      );
      
    case 'FootnoteReference':
      return (
        <sup>
          <a 
            href={`#footnote-${segment.content}`} 
            className={styles.footnoteRef}
            id={`footnote-ref-${segment.content}`}
          >
            {segment.content}
          </a>
        </sup>
      );
      
    case 'CodeInline':
      return (
        <code className={styles.inlineCode}>
          {segment.content}
        </code>
      );
      
    default:
      // Handle unknown segment types gracefully
      console.warn('Unknown segment type:', segment);
      return <span>[Unknown segment type]</span>;
  }
};

// Helper function to convert TextStyle array to CSS classes
function getStyleClasses(textStyles: TextStyle[]): string {
  const classes: string[] = [];
  
  textStyles.forEach(style => {
    if (typeof style === 'string') {
      switch (style) {
        case 'Bold':
          classes.push(styles.bold);
          break;
        case 'Italic':
          classes.push(styles.italic);
          break;
        case 'Underline':
          classes.push(styles.underline);
          break;
        case 'Strikethrough':
          classes.push(styles.strikethrough);
          break;
        case 'Superscript':
          classes.push(styles.superscript);
          break;
        case 'Subscript':
          classes.push(styles.subscript);
          break;
        default:
          console.warn('Unknown text style:', style);
      }
    }
    // Complex styles (with properties) are handled in getInlineStyles
  });
  
  return classes.join(' ');
}

// Helper function to convert TextStyle array to inline CSS styles
function getInlineStyles(textStyles: TextStyle[]): React.CSSProperties {
  const inlineStyles: React.CSSProperties = {};
  
  textStyles.forEach(style => {
    if (typeof style === 'object' && style !== null && 'type' in style) {
      switch (style.type) {
        case 'Color':
          inlineStyles.color = style.content;
          break;
        case 'BackgroundColor':
          inlineStyles.backgroundColor = style.content;
          break;
        case 'FontSize':
          inlineStyles.fontSize = style.content;
          break;
        case 'FontFamily':
          inlineStyles.fontFamily = style.content;
          break;
        default:
          console.warn('Unknown style type:', (style as any).type);
      }
    }
  });
  
  return inlineStyles;
}

// Link renderer component
interface LinkRendererProps {
  target: LinkTarget;
  tooltip?: string;
  children: React.ReactNode;
}

const LinkRenderer: React.FC<LinkRendererProps> = ({ target, tooltip, children }) => {
  const handleClick = (e: React.MouseEvent) => {
    // Handle different link types
    switch (target.type) {
      case 'Url':
        // Let browser handle normally
        break;
        
      case 'InternalPageId':
        e.preventDefault();
        // Navigate to internal page
        console.log('Navigate to internal page:', target.content);
        break;
        
      case 'DefinitionId':
        e.preventDefault();
        // Show definition tooltip or navigate
        console.log('Show definition:', target.term_id);
        break;
        
      case 'TheoremId':
        e.preventDefault();
        // Navigate to theorem
        console.log('Navigate to theorem:', target.content);
        break;
        
      case 'AnimationTrigger':
        e.preventDefault();
        // Trigger animation
        console.log('Trigger animation:', target.animation_id);
        break;
        
      default:
        e.preventDefault();
        console.log('Unhandled link type:', target);
    }
  };

  const getHref = (): string => {
    switch (target.type) {
      case 'Url':
        return target.content;
      case 'InternalPageId':
        return `#${target.content}`;
      case 'DefinitionId':
        return `#definition-${target.term_id}`;
      case 'TheoremId':
        return `#theorem-${target.content}`;
      default:
        return '#';
    }
  };

  const getLinkClassName = (): string => {
    const baseClass = styles.link;
    switch (target.type) {
      case 'Url':
        return `${baseClass} ${styles.externalLink}`;
      case 'DefinitionId':
        return `${baseClass} ${styles.definitionLink}`;
      case 'TheoremId':
        return `${baseClass} ${styles.theoremLink}`;
      default:
        return baseClass;
    }
  };

  return (
    <a
      href={getHref()}
      className={getLinkClassName()}
      onClick={handleClick}
      title={tooltip}
      data-link-type={target.type}
    >
      {children}
    </a>
  );
};

export default RichTextRenderer; 