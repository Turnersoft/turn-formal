import React from 'react';
import { SectionContentNode } from './types';
import { RichTextRenderer } from './RichTextRenderer';
import { MathRenderer } from './MathRenderer';
import { SectionRenderer } from './SectionRenderer';
import { LayoutRenderers } from './LayoutRenderers';
import styles from './styles.module.css';

interface SectionContentRendererProps {
  content: SectionContentNode;
  contentIndex?: number;
}

export const SectionContentRenderer: React.FC<SectionContentRendererProps> = ({ 
  content, 
  contentIndex: _contentIndex
}) => {
  switch (content.type) {
    case 'Paragraph':
      return (
        <div className={`${styles.paragraph} ${getAlignmentClass(content.content.alignment)}`}>
          <RichTextRenderer segments={content.content.segments} />
        </div>
      );

    case 'MathBlock':
      return (
        <div className={styles.mathBlock}>
          {content.label && (
            <div className={styles.mathLabel}>({content.label})</div>
          )}
          <MathRenderer math={content.math} inline={false} />
          {content.caption && (
            <div className={styles.mathCaption}>
              <RichTextRenderer segments={content.caption.segments} />
            </div>
          )}
        </div>
      );

    case 'List':
      return <ListRenderer list={content.content} />;

    case 'Table':
      return <TableRenderer table={content.content} />;

    case 'CodeBlock':
      return <CodeBlockRenderer codeBlock={content.content} />;

    case 'Image':
      return <ImageRenderer image={content.content} />;

    case 'CollapsibleBlock':
      return <CollapsibleBlockRenderer block={content.content} />;

    case 'Grid':
      return <GridRenderer grid={content.content} />;

    case 'Columns':
      return <ColumnsRenderer columns={content.content} />;

    case 'ThematicBreak':
      return <hr className={styles.thematicBreak} />;

    case 'QuoteBlock':
      return (
        <blockquote className={styles.quoteBlock}>
          {content.content.content.map((paragraph, index) => (
            <div key={index} className={styles.quoteParagraph}>
              <RichTextRenderer segments={paragraph.segments} />
            </div>
          ))}
          {content.content.attribution && (
            <cite className={styles.quoteAttribution}>
              <RichTextRenderer segments={content.content.attribution.segments} />
            </cite>
          )}
        </blockquote>
      );

    case 'AlertBox':
      return (
        <div className={`${styles.alertBox} ${styles[`alert-${content.style.toLowerCase()}`]}`}>
          {content.content.map((alertContent, index) => (
            <SectionContentRenderer 
              key={index}
              content={alertContent}
              contentIndex={index}
            />
          ))}
        </div>
      );

    case 'SubSection':
      return <SectionRenderer section={content.content} level={2} />;

    case 'SideBySideLayout':
      return <LayoutRenderers.SideBySideRenderer layout={content.content} />;

    case 'PanelLayout':
      return <LayoutRenderers.PanelLayoutRenderer layout={content.content} />;

    case 'AnnotationOverlay':
      return <LayoutRenderers.AnnotationOverlayRenderer overlay={content.content} />;

    case 'InteractiveControls':
      return <LayoutRenderers.InteractiveControlsRenderer controls={content.content} />;

    case 'EmbeddedDocument':
      return (
        <div className={styles.embeddedDocument}>
          {/* Would import and use FlexibleDocumentRenderer here */}
          <div>Embedded Document: {content.content.title}</div>
        </div>
      );

    case 'CustomComponent':
      return (
        <div className={styles.customComponent} data-component={content.component_name}>
          <div className={styles.customComponentPlaceholder}>
            Custom Component: {content.component_name}
          </div>
          {content.fallback_content && (
            <div className={styles.fallbackContent}>
              {content.fallback_content.map((fallbackItem, index) => (
                <SectionContentRenderer 
                  key={index}
                  content={fallbackItem}
                  contentIndex={index}
                />
              ))}
            </div>
          )}
        </div>
      );

    default:
      return (
        <div className={styles.unknownContent}>
          Unknown content type: {(content as any).type}
        </div>
      );
  }
};

// Helper function to get alignment CSS class
function getAlignmentClass(alignment?: any): string {
  if (!alignment) return '';
  return styles[`align-${alignment.toLowerCase()}`] || '';
}

// List renderer
const ListRenderer: React.FC<{ list: any }> = ({ list }) => {
  const isOrdered = list.style.type === 'Ordered';
  const Tag = isOrdered ? 'ol' : 'ul';
  
  const getListStyleClass = () => {
    if (isOrdered) {
      return styles[`ordered-${list.style.content.toLowerCase()}`];
    } else {
      return styles[`unordered-${list.style.content.toLowerCase()}`];
    }
  };

  return (
    <Tag 
      className={`${styles.list} ${getListStyleClass()}`}
      start={list.start_index}
    >
      {list.items.map((item: any, index: number) => (
        <li key={index} className={styles.listItem}>
          {item.content.map((contentNode: any, contentIndex: number) => (
            <SectionContentRenderer 
              key={contentIndex}
              content={contentNode}
              contentIndex={contentIndex}
            />
          ))}
        </li>
      ))}
    </Tag>
  );
};

// Table renderer
const TableRenderer: React.FC<{ table: any }> = ({ table }) => {
  return (
    <div className={styles.tableWrapper}>
      {table.caption && (
        <div className={styles.tableCaption}>
          <RichTextRenderer segments={table.caption.segments} />
        </div>
      )}
      <table className={styles.table}>
        {table.header_rows && (
          <thead>
            {table.header_rows.map((row: any, index: number) => (
              <tr key={index}>
                {row.cells.map((cell: any, cellIndex: number) => (
                  <th 
                    key={cellIndex}
                    colSpan={cell.col_span}
                    rowSpan={cell.row_span}
                    className={getAlignmentClass(cell.alignment)}
                  >
                    {cell.content.map((contentNode: any, contentIndex: number) => (
                      <SectionContentRenderer 
                        key={contentIndex}
                        content={contentNode}
                        contentIndex={contentIndex}
                      />
                    ))}
                  </th>
                ))}
              </tr>
            ))}
          </thead>
        )}
        <tbody>
          {table.body_rows.map((row: any, index: number) => (
            <tr key={index}>
              {row.cells.map((cell: any, cellIndex: number) => (
                <td 
                  key={cellIndex}
                  colSpan={cell.col_span}
                  rowSpan={cell.row_span}
                  className={getAlignmentClass(cell.alignment)}
                >
                  {cell.content.map((contentNode: any, contentIndex: number) => (
                    <SectionContentRenderer 
                      key={contentIndex}
                      content={contentNode}
                      contentIndex={contentIndex}
                    />
                  ))}
                </td>
              ))}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
};

// Code block renderer
const CodeBlockRenderer: React.FC<{ codeBlock: any }> = ({ codeBlock }) => {
  return (
    <div className={styles.codeBlockWrapper}>
      {codeBlock.caption && (
        <div className={styles.codeCaption}>
          <RichTextRenderer segments={codeBlock.caption.segments} />
        </div>
      )}
      <pre className={styles.codeBlock}>
        <code 
          className={codeBlock.language ? `language-${codeBlock.language}` : ''}
          data-language={codeBlock.language}
        >
          {codeBlock.code}
        </code>
      </pre>
    </div>
  );
};

// Image renderer
const ImageRenderer: React.FC<{ image: any }> = ({ image }) => {
  return (
    <figure className={`${styles.figure} ${getAlignmentClass(image.alignment)}`}>
      <img 
        src={image.src}
        alt={image.alt_text || ''}
        className={styles.image}
        style={{
          width: image.width,
          height: image.height
        }}
      />
      {image.caption && (
        <figcaption className={styles.imageCaption}>
          <RichTextRenderer segments={image.caption.segments} />
        </figcaption>
      )}
    </figure>
  );
};

// Collapsible block renderer
const CollapsibleBlockRenderer: React.FC<{ block: any }> = ({ block }) => {
  const [isOpen, setIsOpen] = React.useState(!block.initially_collapsed);

  return (
    <details 
      className={styles.collapsibleBlock}
      open={isOpen}
      onToggle={(e) => setIsOpen((e.target as HTMLDetailsElement).open)}
    >
      <summary className={styles.collapsibleSummary}>
        <RichTextRenderer segments={block.summary} />
      </summary>
      <div className={styles.collapsibleDetails}>
        {block.details.map((contentNode: any, index: number) => (
          <SectionContentRenderer 
            key={index}
            content={contentNode}
            contentIndex={index}
          />
        ))}
      </div>
    </details>
  );
};

// Grid renderer
const GridRenderer: React.FC<{ grid: any }> = ({ grid }) => {
  const gridStyle: React.CSSProperties = {
    gridTemplateColumns: grid.column_template,
    gap: `${grid.row_gap || '1rem'} ${grid.column_gap || '1rem'}`
  };

  return (
    <div className={styles.grid} style={gridStyle}>
      {grid.items.map((item: any, index: number) => {
        const itemStyle: React.CSSProperties = {
          gridColumn: item.col_start ? `${item.col_start} / ${item.col_end || 'auto'}` : undefined,
          gridRow: item.row_start ? `${item.row_start} / ${item.row_end || 'auto'}` : undefined
        };

        return (
          <div key={index} className={styles.gridItem} style={itemStyle}>
            <SectionContentRenderer 
              content={item.content}
              contentIndex={index}
            />
          </div>
        );
      })}
    </div>
  );
};

// Columns renderer
const ColumnsRenderer: React.FC<{ columns: any }> = ({ columns }) => {
  const columnsStyle: React.CSSProperties = {
    gridTemplateColumns: columns.column_widths?.join(' ') || `repeat(${columns.columns_content.length}, 1fr)`,
    gap: columns.gap || '1rem'
  };

  return (
    <div className={styles.columns} style={columnsStyle}>
      {columns.columns_content.map((columnContent: any[], index: number) => (
        <div key={index} className={styles.column}>
          {columnContent.map((contentNode: any, contentIndex: number) => (
            <SectionContentRenderer 
              key={contentIndex}
              content={contentNode}
              contentIndex={contentIndex}
            />
          ))}
        </div>
      ))}
    </div>
  );
}; 