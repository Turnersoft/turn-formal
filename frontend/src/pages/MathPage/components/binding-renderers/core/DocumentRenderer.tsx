import React from 'react';
import type { MathematicalContent } from '../../turn-render/bindings/MathematicalContent';
import type { MathematicalContentType } from '../../turn-render/bindings/MathematicalContentType';
import { SectionContentRenderer } from './SectionContentRenderer';
import { cleanGroupNotation } from '../../../../../utils/mathNotationCleaner';
import styles from './DocumentRenderer.module.css';

interface DocumentRendererProps {
  content: MathematicalContent;
  className?: string;
}

/**
 * Core document renderer that directly uses Rust binding types
 * Handles MathematicalContent and all its content type variants
 */
export const DocumentRenderer: React.FC<DocumentRendererProps> = ({ 
  content, 
  className = '' 
}) => {
  return (
    <div 
      className={`${styles.document} ${className}`}
      data-content-id={content.id}
    >
      <ContentTypeRenderer contentType={content.content_type} />
    </div>
  );
};

/**
 * Renders MathematicalContentType using exact binding type structure
 */
const ContentTypeRenderer: React.FC<{ contentType: MathematicalContentType }> = ({ 
  contentType 
}) => {
  // Handle each variant of MathematicalContentType exactly as defined in Rust
  if (typeof contentType === 'object' && contentType !== null && 'ScientificPaper' in contentType) {
    const paper = (contentType as any).ScientificPaper;
    return (
      <div className={styles.scientificPaper}>
        <header className={styles.paperHeader}>
          <h1 className={styles.paperTitle}>{cleanGroupNotation(paper.title)}</h1>
          <div className={styles.paperMeta}>
            <span className={styles.paperType}>{paper.paper_type}</span>
            {paper.venue && (
              <span className={styles.paperVenue}>{paper.venue}</span>
            )}
            {paper.peer_reviewed && (
              <span className={styles.peerReviewed}>Peer Reviewed</span>
            )}
          </div>
          {paper.academic_metadata?.authors && paper.academic_metadata.authors.length > 0 && (
            <div className={styles.paperAuthors}>
              By: {paper.academic_metadata.authors.join(', ')}
            </div>
          )}
          {paper.academic_metadata?.keywords && paper.academic_metadata.keywords.length > 0 && (
            <div className={styles.paperKeywords}>
              {paper.academic_metadata.keywords.map((keyword: string, i: number) => (
                <span key={i} className={styles.keyword}>{keyword}</span>
              ))}
            </div>
          )}
        </header>
        
        {paper.structure?.abstract_content && (
          <section className={styles.abstract}>
            <h2 className={styles.abstractTitle}>Abstract</h2>
            <div className={styles.abstractContent}>
              <SectionContentRenderer sections={[paper.structure.abstract_content]} />
            </div>
          </section>
        )}
        
        {paper.structure?.body && paper.structure.body.length > 0 && (
          <main className={styles.paperBody}>
            <SectionContentRenderer sections={paper.structure.body} />
          </main>
        )}

        {paper.structure?.footnotes && paper.structure.footnotes.length > 0 && (
          <footer className={styles.footnotes}>
            <h3>Footnotes</h3>
            {paper.structure.footnotes.map((footnote: any, index: number) => (
              <div key={index} className={styles.footnote}>
                {footnote}
              </div>
            ))}
          </footer>
        )}
      </div>
    );
  }

  if (typeof contentType === 'object' && contentType !== null && 'WikiPage' in contentType) {
    const wiki = (contentType as any).WikiPage;
    return (
      <div className={styles.wikiPage}>
        <header className={styles.wikiHeader}>
          <h1 className={styles.wikiTitle}>{cleanGroupNotation(wiki.title)}</h1>
          <div className={styles.wikiMeta}>
            <span className={styles.contentType}>Wiki Page</span>
          </div>
        </header>
        <div className={styles.wikiContent}>
          {wiki.structure?.body && (
            <SectionContentRenderer sections={wiki.structure.body} />
          )}
        </div>
      </div>
    );
  }

  if (typeof contentType === 'object' && contentType !== null && 'Textbook' in contentType) {
    const textbook = (contentType as any).Textbook;
    return (
      <div className={styles.textbook}>
        <header className={styles.textbookHeader}>
          <h1 className={styles.textbookTitle}>{cleanGroupNotation(textbook.title)}</h1>
          <div className={styles.textbookMeta}>
            <span className={styles.contentType}>Textbook</span>
          </div>
        </header>
        <div className={styles.textbookContent}>
          {textbook.structure?.body && (
            <SectionContentRenderer sections={textbook.structure.body} />
          )}
        </div>
      </div>
    );
  }

  // For any unhandled content type, show debug info
  const contentTypeKey = typeof contentType === 'object' && contentType !== null 
    ? Object.keys(contentType)[0] 
    : String(contentType);
    
  return (
    <div className={styles.unhandledContentType}>
      <h2>Unhandled Content Type</h2>
      <p>Content Type: <code>{contentTypeKey}</code></p>
      <details>
        <summary>Raw Data</summary>
        <pre>{JSON.stringify(contentType, null, 2)}</pre>
      </details>
    </div>
  );
};

export default DocumentRenderer; 