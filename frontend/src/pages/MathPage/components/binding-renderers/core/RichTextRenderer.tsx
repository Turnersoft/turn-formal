import React from 'react';

interface RichTextRendererProps {
  segments: any[];
}

/**
 * Stub RichTextRenderer - will be implemented later
 */
export const RichTextRenderer: React.FC<RichTextRendererProps> = ({ segments }) => {
  return (
    <span>
      {segments.map((segment, i) => (
        <span key={i}>{typeof segment === 'string' ? segment : '[Rich Text]'}</span>
      ))}
    </span>
  );
};

export default RichTextRenderer; 