import React from 'react';
import { RichTextRenderer } from '../core/RichTextRenderer';
import type { RichTextSegment } from '../../turn-render/bindings/RichTextSegment';

/**
 * Test component to verify LinkRenderer functionality
 * with sample data from group_theory.definitions.json
 */
export const LinkTest: React.FC = () => {
  // Sample link data from the group_theory.definitions.json
  const sampleLinkSegments: RichTextSegment[] = [
    { "Text": "For the underlying group structure, see " },
    {
      "Link": {
        "content": [
          { "Text": "Group Theory" }
        ],
        "target": {
          "DefinitionId": {
            "term_id": "group_theory.symmetric-main-groupbasic-section",
            "theory_context": "GroupTheory"
          }
        },
        "tooltip": "View definition of group_theory.symmetric-main-groupbasic-section"
      }
    },
    { "Text": "." }
  ];

  const sampleMathLinkSegments: RichTextSegment[] = [
    { "Text": "Topological Group " },
    {
      "Math": {
        "id": "group_theory.topological-main-title-math",
        "content": {
          "Text": "(G, \\tau)"
        }
      }
    }
  ];

  return (
    <div style={{ padding: '20px', maxWidth: '800px' }}>
      <h2>Link Renderer Test</h2>
      
      <div style={{ marginBottom: '20px', padding: '15px', border: '1px solid #ddd', borderRadius: '5px' }}>
        <h3>Definition Link Test</h3>
        <p>
          <RichTextRenderer segments={sampleLinkSegments} />
        </p>
      </div>

      <div style={{ marginBottom: '20px', padding: '15px', border: '1px solid #ddd', borderRadius: '5px' }}>
        <h3>Math + Text Test</h3>
        <p>
          <RichTextRenderer segments={sampleMathLinkSegments} />
        </p>
      </div>

      <div style={{ marginBottom: '20px', padding: '15px', border: '1px solid #ddd', borderRadius: '5px' }}>
        <h3>Link Types Test</h3>
        <p>
          <RichTextRenderer segments={[
            { "Text": "External link: " },
            {
              "Link": {
                "content": [{ "Text": "Wikipedia" }],
                "target": { "Url": "https://en.wikipedia.org/wiki/Group_theory" },
                "tooltip": "External Wikipedia link"
              }
            }
          ]} />
        </p>
        
        <p>
          <RichTextRenderer segments={[
            { "Text": "Theorem link: " },
            {
              "Link": {
                "content": [{ "Text": "Lagrange's Theorem" }],
                "target": { "TheoremId": "lagrange_theorem" },
                "tooltip": "View Lagrange's theorem"
              }
            }
          ]} />
        </p>
      </div>

      <div style={{ padding: '15px', backgroundColor: '#f9f9f9', borderRadius: '5px' }}>
        <h3>Instructions</h3>
        <p>Click the links above to test the functionality:</p>
        <ul>
          <li><strong>Definition links</strong> (blue) should try to scroll to the definition on the page</li>
          <li><strong>External links</strong> (blue with 🔗) should open in a new tab</li>
          <li><strong>Theorem links</strong> (orange) should try to scroll to the theorem</li>
          <li>Check the browser console for navigation logs</li>
        </ul>
      </div>
    </div>
  );
};

export default LinkTest; 