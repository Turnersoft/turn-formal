import React from 'react';
import { Section } from './types';
import { SectionContentRenderer } from './SectionContentRenderer.tsx';
import { RichTextRenderer } from './RichTextRenderer.tsx';
import styles from './styles.module.css';

interface SectionRendererProps {
  section: Section;
  sectionIndex?: number;
  level?: number; // For nested sections
}

export const SectionRenderer: React.FC<SectionRendererProps> = ({ 
  section, 
  sectionIndex = 0,
  level = 1 
}) => {
  const HeadingTag = `h${Math.min(level + 1, 6)}` as keyof JSX.IntrinsicElements;
  
  return (
    <section 
      id={section.id} 
      className={`${styles.section} ${styles[`section-level-${level}`]}`}
      data-section-id={section.id}
    >
      {section.title && (
        <HeadingTag className={styles.sectionTitle}>
          {section.display_options?.show_title_numbering && (
            <span className={styles.sectionNumber}>
              {generateSectionNumber(sectionIndex, level)}.
            </span>
          )}
          <RichTextRenderer segments={section.title.segments} />
        </HeadingTag>
      )}
      
      <div className={styles.sectionContent}>
        {section.content.map((contentNode, index) => (
          <SectionContentRenderer 
            key={index}
            content={contentNode}
            contentIndex={index}
          />
        ))}
      </div>
      
      {section.metadata && (
        <div className={styles.sectionMetadata}>
          {section.metadata.map(([key, value], index) => (
            <span key={index} className={`${styles.metadataItem} ${styles[`metadata-${key}`]}`}>
              <span className={styles.metadataKey}>{key}:</span>
              <span className={styles.metadataValue}>{value}</span>
            </span>
          ))}
        </div>
      )}
    </section>
  );
};

// Helper function to generate section numbers
function generateSectionNumber(index: number, _level: number): string {
  // Simple numbering scheme - could be made more sophisticated
  return `${index + 1}`;
}

export default SectionRenderer; 