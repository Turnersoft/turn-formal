import React, { useEffect, useState } from 'react';
import { SectionContentRenderer } from './pages/MathPage/components/turn-render/components/section_node/section_node.tsx';
import type { Section } from './pages/MathPage/components/turn-render/bindings/Section.ts';

const TestMathRendering: React.FC = () => {
  const [groupTheoryData, setGroupTheoryData] = useState<any>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    fetch('/group_theory.theorems.json')
      .then(response => response.json())
      .then(data => {
        setGroupTheoryData(data);
        setLoading(false);
      })
      .catch(err => {
        setError(err.message);
        setLoading(false);
      });
  }, []);

  if (loading) return <div>Loading...</div>;
  if (error) return <div>Error: {error}</div>;
  if (!groupTheoryData) return <div>No data</div>;

  // Extract sections from the data
  const sections: Section[] = groupTheoryData.content_type?.ScientificPaper?.structure?.body || [];

  return (
    <div style={{ padding: '20px', maxWidth: '800px', margin: '0 auto' }}>
      <h1>Test: Math Rendering in Proof Steps</h1>
      <p>Testing if Goal steps with Math nodes render properly (instead of showing "[Math]")</p>
      <div style={{ border: '1px solid #ccc', padding: '20px', borderRadius: '8px' }}>
        <SectionContentRenderer sections={sections} />
      </div>
    </div>
  );
};

export default TestMathRendering; 