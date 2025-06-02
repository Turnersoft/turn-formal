import React, { useState, useEffect } from 'react';
import { RichTextRenderer } from '../core/RichTextRenderer';
import type { RichTextSegment } from '../../turn-render/bindings/RichTextSegment';

interface GroupTheoryDocument {
  id: string;
  content_type: {
    ScientificPaper: {
      title: string;
      structure: {
        body: Array<{
          id: string;
          title?: {
            segments: RichTextSegment[];
          };
          content: Array<{
            Paragraph?: {
              segments: RichTextSegment[];
            };
            StructuredMath?: {
              Definition?: {
                term_display: Array<{ Text?: string }>;
                body: Array<{
                  Paragraph?: {
                    segments: RichTextSegment[];
                  };
                }>;
              };
            };
          }>;
        }>;
      };
    };
  };
}

interface GroupTheoryData {
  content: GroupTheoryDocument[];
}

/**
 * Test component that loads real group theory definitions 
 * and tests link functionality with the actual data
 */
export const GroupTheoryLinkTest: React.FC = () => {
  const [groupTheoryData, setGroupTheoryData] = useState<GroupTheoryData | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const loadGroupTheoryData = async () => {
      try {
        const response = await fetch('/group_theory.definitions.json');
        if (!response.ok) {
          throw new Error(`Failed to load group theory data: ${response.statusText}`);
        }
        const data = await response.json();
        setGroupTheoryData(data);
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Unknown error');
      } finally {
        setLoading(false);
      }
    };

    loadGroupTheoryData();
  }, []);

  const renderSectionContent = (content: any[], maxItems: number = 3) => {
    return content.slice(0, maxItems).map((item, index) => {
      if (item.Paragraph?.segments) {
        return (
          <p key={index} style={{ marginBottom: '10px' }}>
            <RichTextRenderer segments={item.Paragraph.segments} />
          </p>
        );
      }
      
      if (item.StructuredMath?.Definition) {
        const def = item.StructuredMath.Definition;
        return (
          <div key={index} style={{ 
            marginBottom: '15px', 
            padding: '10px', 
            backgroundColor: '#f9f9f9', 
            borderRadius: '5px' 
          }}>
            <h4 style={{ margin: '0 0 10px 0', color: '#0066cc' }}>
              {def.term_display.map((term: any, i: number) => (
                <span key={i}>{term.Text || ''}</span>
              ))}
            </h4>
            {def.body.slice(0, 2).map((bodyItem: any, bodyIndex: number) => {
              if (bodyItem.Paragraph?.segments) {
                return (
                  <p key={bodyIndex} style={{ margin: '5px 0', fontSize: '0.9em' }}>
                    <RichTextRenderer segments={bodyItem.Paragraph.segments} />
                  </p>
                );
              }
              return null;
            })}
          </div>
        );
      }
      
      return null;
    }).filter(Boolean);
  };

  if (loading) {
    return <div style={{ padding: '20px' }}>Loading group theory data...</div>;
  }

  if (error) {
    return (
      <div style={{ padding: '20px', color: 'red' }}>
        <h2>Error loading group theory data</h2>
        <p>{error}</p>
        <p>Make sure the group_theory.definitions.json file is in the public folder.</p>
      </div>
    );
  }

  if (!groupTheoryData?.content) {
    return <div style={{ padding: '20px' }}>No group theory data found.</div>;
  }

  return (
    <div style={{ padding: '20px', maxWidth: '1200px' }}>
      <h1>Group Theory Link Test - Real Data</h1>
      
      <div style={{ 
        marginBottom: '20px', 
        padding: '15px', 
        backgroundColor: '#e6f3ff', 
        borderRadius: '5px' 
      }}>
        <h3>Instructions</h3>
        <ul>
          <li>This component loads real group theory definitions from <code>group_theory.definitions.json</code></li>
          <li>Click on any blue links to test the definition navigation</li>
          <li>Links should try to scroll to the corresponding definition on this page</li>
          <li>Check the browser console for navigation logs</li>
          <li>Found {groupTheoryData.content.length} group theory definitions</li>
        </ul>
      </div>

      <div style={{ display: 'grid', gap: '20px' }}>
        {groupTheoryData.content.slice(0, 6).map((doc, index) => {
          const paper = doc.content_type.ScientificPaper;
          const mainSection = paper.structure.body[0];
          
          return (
            <div 
              key={doc.id}
              id={doc.id}
              style={{ 
                border: '1px solid #ddd', 
                borderRadius: '8px', 
                padding: '20px',
                backgroundColor: '#fff'
              }}
            >
              <h2 style={{ 
                margin: '0 0 15px 0', 
                color: '#333',
                borderBottom: '2px solid #007acc',
                paddingBottom: '10px'
              }}>
                {paper.title}
              </h2>
              
              {mainSection?.title?.segments && (
                <h3 style={{ color: '#666', margin: '0 0 15px 0' }}>
                  <RichTextRenderer segments={mainSection.title.segments} />
                </h3>
              )}
              
              <div style={{ lineHeight: '1.6' }}>
                {mainSection?.content && renderSectionContent(mainSection.content)}
              </div>
              
              <div style={{ 
                marginTop: '15px', 
                padding: '10px', 
                backgroundColor: '#f5f5f5', 
                borderRadius: '3px',
                fontSize: '0.8em',
                color: '#666'
              }}>
                ID: {doc.id}
              </div>
            </div>
          );
        })}
      </div>
      
      {groupTheoryData.content.length > 6 && (
        <div style={{ 
          marginTop: '20px', 
          padding: '15px', 
          textAlign: 'center',
          backgroundColor: '#f9f9f9',
          borderRadius: '5px'
        }}>
          <p>Showing first 6 of {groupTheoryData.content.length} definitions.</p>
          <p>Scroll up to test links - they should navigate to definitions on this page.</p>
        </div>
      )}
    </div>
  );
};

export default GroupTheoryLinkTest; 