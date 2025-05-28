import React from 'react';
import { renderMathNode } from '../../turn-render/turn-math';
import { cleanSegmentText } from '../../../../../utils/mathNotationCleaner';
import styles from './SectionContentRenderer.module.css';

// Type definitions based on the JSON structure
interface ParagraphSegment {
  Text?: string;
  MathInline?: any;
  MathBlock?: any;
}

interface ParagraphNode {
  segments: ParagraphSegment[];
  alignment?: string | null;
}

interface SubSectionNode {
  id: string;
  title?: {
    segments: ParagraphSegment[];
    alignment?: string | null;
  };
  content: SectionContentNode[];
  metadata: Array<[string, string]>;
  display_options?: any;
}

interface SectionContentNode {
  Paragraph?: ParagraphNode;
  SubSection?: SubSectionNode;
  MathBlock?: {
    math: any; // MathNode type
    label?: string;
    caption?: string;
  };
  List?: {
    items: Array<{
      content: SectionContentNode[];
    }>;
    style?: string;
  };
  Table?: {
    rows: Array<{
      cells: Array<{
        content: SectionContentNode[];
        span?: number;
      }>;
    }>;
    headers?: Array<{
      content: SectionContentNode[];
    }>;
  };
  StructuredMath?: {
    Definition?: {
      term_display: Array<{ Text?: string }>;
      formal_term?: any; // MathNode type
      label?: string;
      body: SectionContentNode[];
      abstraction_meta?: {
        level?: number;
        source_template_id?: string;
        specified_parameters?: string[];
        universally_quantified_properties?: string[];
      };
      selectable_properties?: Array<{
        name: string;
        current_variant: string;
        all_variants: string[];
        description?: string;
      }>;
    };
    Theorem?: {
      kind: string;
      label?: string;
      statement: SectionContentNode[];
      proof?: {
        steps: SectionContentNode[];
      };
    };
    Example?: {
      title?: string;
      content: SectionContentNode[];
    };
  };
}

interface SectionNode {
  id: string;
  title?: {
    segments: ParagraphSegment[];
    alignment?: string | null;
  };
  content: SectionContentNode[];
  metadata: Array<[string, string]>;
  display_options?: any;
}

interface SectionContentRendererProps {
  sections: SectionNode[];
  className?: string;
}

export const SectionContentRenderer: React.FC<SectionContentRendererProps> = ({ 
  sections, 
  className = '' 
}) => {
  return (
    <div className={`${styles.sectionsContainer} ${className}`}>
      {sections.map((section, index) => (
        <SectionRenderer key={section.id || index} section={section} />
      ))}
    </div>
  );
};

const SectionRenderer: React.FC<{ section: SectionNode }> = ({ section }) => {
  const getMetadata = (key: string): string | undefined => {
    return section.metadata?.find(([k]) => k === key)?.[1];
  };

  const sectionLevel = getMetadata('abstraction_level') || '1';
  const sectionType = getMetadata('type') || 'general';

  return (
    <section 
      className={`${styles.section} ${styles[`level${sectionLevel}`]} ${styles[sectionType]}`}
      id={section.id}
    >
      {section.title && (
        <header className={styles.sectionHeader}>
          <h2 className={styles.sectionTitle}>
            <ParagraphSegmentRenderer segments={section.title.segments} />
          </h2>
          {section.metadata.length > 0 && (
            <div className={styles.sectionMeta}>
              <span className={styles.level}>Level {sectionLevel}</span>
              {sectionType !== 'general' && (
                <span className={styles.type}>{sectionType}</span>
              )}
            </div>
          )}
        </header>
      )}
      
      <div className={styles.sectionContent}>
        {section.content.map((contentNode, index) => (
          <ContentNodeRenderer key={index} node={contentNode} />
        ))}
      </div>
    </section>
  );
};

const ContentNodeRenderer: React.FC<{ node: SectionContentNode }> = ({ node }) => {
  if (node.Paragraph) {
    return (
      <div className={`${styles.paragraph} ${node.Paragraph.alignment ? styles[node.Paragraph.alignment] : ''}`}>
        <ParagraphSegmentRenderer segments={node.Paragraph.segments} />
      </div>
    );
  }

  if (node.SubSection) {
    return (
      <div className={styles.subsection}>
        {node.SubSection.title && (
          <h3 className={styles.subsectionTitle}>
            <ParagraphSegmentRenderer segments={node.SubSection.title.segments} />
          </h3>
        )}
        <div className={styles.subsectionContent}>
          {node.SubSection.content.map((subNode, index) => (
            <ContentNodeRenderer key={index} node={subNode} />
          ))}
        </div>
      </div>
    );
  }

  if (node.MathBlock) {
    return (
      <div className={styles.mathBlock}>
        <div className={styles.mathContent}>
          {renderMathNode(node.MathBlock.math)}
        </div>
        {node.MathBlock.label && (
          <div className={styles.mathLabel}>{node.MathBlock.label}</div>
        )}
        {node.MathBlock.caption && (
          <div className={styles.mathCaption}>{node.MathBlock.caption}</div>
        )}
      </div>
    );
  }

  if (node.List) {
    return (
      <div className={`${styles.list} ${node.List.style ? styles[node.List.style] : styles.unordered}`}>
        <ul className={styles.listItems}>
          {node.List.items.map((item, index) => (
            <li key={index} className={styles.listItem}>
              {item.content.map((itemNode, itemIndex) => (
                <ContentNodeRenderer key={itemIndex} node={itemNode} />
              ))}
            </li>
          ))}
        </ul>
      </div>
    );
  }

  if (node.Table) {
    return (
      <div className={styles.table}>
        <table className={styles.tableElement}>
          {node.Table.headers && (
            <thead>
              <tr>
                {node.Table.headers.map((header, index) => (
                  <th key={index} className={styles.tableHeader}>
                    {header.content.map((headerNode, headerIndex) => (
                      <ContentNodeRenderer key={headerIndex} node={headerNode} />
                    ))}
                  </th>
                ))}
              </tr>
            </thead>
          )}
          <tbody>
            {node.Table.rows.map((row, rowIndex) => (
              <tr key={rowIndex}>
                {row.cells.map((cell, cellIndex) => (
                  <td 
                    key={cellIndex} 
                    className={styles.tableCell}
                    colSpan={cell.span || 1}
                  >
                    {cell.content.map((cellNode, cellNodeIndex) => (
                      <ContentNodeRenderer key={cellNodeIndex} node={cellNode} />
                    ))}
                  </td>
                ))}
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    );
  }

  if (node.StructuredMath) {
    if (node.StructuredMath.Definition) {
      return (
        <div className={styles.structuredMathDefinition}>
          <h3 className={styles.definitionTitle}>
            {node.StructuredMath.Definition.term_display.map((term, index) => (
              <span key={index}>{term.Text ? cleanSegmentText(term.Text) : ''}</span>
            ))}
          </h3>
          <div className={styles.definitionContent}>
            {node.StructuredMath.Definition.body.map((contentNode, index) => (
              <ContentNodeRenderer key={index} node={contentNode} />
            ))}
          </div>
        </div>
      );
    }

    if (node.StructuredMath.Theorem) {
      return (
        <div className={styles.structuredMathTheorem}>
          <h3 className={styles.theoremTitle}>
            {node.StructuredMath.Theorem.label}
          </h3>
          <div className={styles.theoremContent}>
            {node.StructuredMath.Theorem.statement.map((contentNode, index) => (
              <ContentNodeRenderer key={index} node={contentNode} />
            ))}
          </div>
        </div>
      );
    }

    if (node.StructuredMath.Example) {
      return (
        <div className={styles.structuredMathExample}>
          <h3 className={styles.exampleTitle}>
            {node.StructuredMath.Example.title}
          </h3>
          <div className={styles.exampleContent}>
            {node.StructuredMath.Example.content.map((contentNode, index) => (
              <ContentNodeRenderer key={index} node={contentNode} />
            ))}
          </div>
        </div>
      );
    }
  }

  // Fallback for unknown content types
  return (
    <div className={styles.unknownContent}>
      <span className={styles.unknownType}>
        [Unknown content type: {Object.keys(node)[0]}]
      </span>
    </div>
  );
};

const ParagraphSegmentRenderer: React.FC<{ segments: ParagraphSegment[] }> = ({ segments }) => {
  return (
    <>
      {segments.map((segment, index) => {
        if (segment.Text) {
          return <span key={index}>{cleanSegmentText(segment.Text)}</span>;
        }
        
        if (segment.MathInline) {
          return (
            <span key={index} className={styles.inlineMath}>
              {renderMathNode(segment.MathInline)}
            </span>
          );
        }
        
        if (segment.MathBlock) {
          return (
            <div key={index} className={styles.blockMath}>
              {renderMathNode(segment.MathBlock)}
            </div>
          );
        }
        
        return (
          <span key={index} className={styles.unknownSegment}>
            [Unknown segment]
          </span>
        );
      })}
    </>
  );
};

export default SectionContentRenderer; 