import React from 'react';
import { FlexibleDocument, TocNode } from './types';
import { SectionRenderer } from './SectionRenderer.tsx';
import { DocumentTypeRenderer } from './DocumentTypeRenderers';
import styles from './styles.module.css';

interface FlexibleDocumentRendererProps {
  document: FlexibleDocument;
  className?: string;
}

export const FlexibleDocumentRenderer: React.FC<FlexibleDocumentRendererProps> = ({
  document,
  className
}) => {
  const documentClass = `${styles.flexibleDocument} ${styles[`document-${document.document_type.toLowerCase()}`]} ${className || ''}`;

  return (
    <div className={documentClass} data-document-type={document.document_type}>
      <DocumentTypeRenderer document={document}>
        {/* Document header */}
        <header className={styles.documentHeader}>
          <h1 className={styles.documentTitle}>{document.title}</h1>
          
          {document.authors && document.authors.length > 0 && (
            <div className={styles.documentAuthors}>
              {document.authors.map((author, index) => (
                <span key={index} className={styles.author}>
                  {author}
                </span>
              ))}
            </div>
          )}
          
          {(document.date_published || document.date_modified) && (
            <div className={styles.documentDates}>
              {document.date_published && (
                <span className={styles.datePublished}>
                  Published: {document.date_published}
                </span>
              )}
              {document.date_modified && (
                <span className={styles.dateModified}>
                  Modified: {document.date_modified}
                </span>
              )}
            </div>
          )}
        </header>

        {/* Abstract */}
        {document.abstract_content && (
          <section className={styles.documentAbstract}>
            <h2 className={styles.abstractTitle}>Abstract</h2>
            <SectionRenderer section={document.abstract_content} />
          </section>
        )}

        {/* Table of Contents */}
        {document.table_of_contents && (
          <nav className={styles.tableOfContents}>
            <h2>Table of Contents</h2>
            <TocRenderer toc={document.table_of_contents} />
          </nav>
        )}

        {/* Main content body */}
        <main className={styles.documentBody}>
          {document.body.map((section, index) => (
            <SectionRenderer 
              key={section.id || index} 
              section={section}
              sectionIndex={index}
            />
          ))}
        </main>

        {/* Footnotes */}
        {document.footnotes && document.footnotes.length > 0 && (
          <aside className={styles.documentFootnotes}>
            <h2>Footnotes</h2>
            {document.footnotes.map((footnote, index) => (
              <SectionRenderer 
                key={footnote.id || `footnote-${index}`} 
                section={footnote}
              />
            ))}
          </aside>
        )}

        {/* Glossary */}
        {document.glossary && document.glossary.length > 0 && (
          <aside className={styles.documentGlossary}>
            <h2>Glossary</h2>
            {document.glossary.map((entry, index) => (
              <SectionRenderer 
                key={entry.id || `glossary-${index}`} 
                section={entry}
              />
            ))}
          </aside>
        )}

        {/* Bibliography */}
        {document.bibliography.length > 0 && (
          <aside className={styles.documentBibliography}>
            <h2>References</h2>
            <div className={styles.bibliographyList}>
              {document.bibliography.map((entry, index) => (
                <div key={index} className={styles.bibliographyEntry}>
                  <span className={styles.bibNumber}>[{index + 1}]</span>
                  <div className={styles.bibFields}>
                    {entry.fields.map(([key, value], fieldIndex) => (
                      <span key={fieldIndex} className={`${styles.bibField} ${styles[`bib-${key}`]}`}>
                        {key === 'author' || key === 'title' ? (
                          <strong>{value}</strong>
                        ) : (
                          value
                        )}
                        {fieldIndex < entry.fields.length - 1 && ', '}
                      </span>
                    ))}
                  </div>
                </div>
              ))}
            </div>
          </aside>
        )}
      </DocumentTypeRenderer>
    </div>
  );
};

// Table of Contents renderer with proper typing
interface TocRendererProps {
  toc: TocNode;
}

const TocRenderer: React.FC<TocRendererProps> = ({ toc }) => {
  return (
    <ul className={styles.tocList}>
      <li className={styles.tocItem}>
        <a href={`#${toc.target_id}`} className={styles.tocLink}>
          {toc.title}
        </a>
        {toc.children && toc.children.length > 0 && (
          <ul className={styles.tocSublist}>
            {toc.children.map((child, index) => (
              <TocRenderer key={index} toc={child} />
            ))}
          </ul>
        )}
      </li>
    </ul>
  );
};

export default FlexibleDocumentRenderer; 