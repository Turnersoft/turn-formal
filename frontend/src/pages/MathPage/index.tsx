import React, { useEffect, useState } from "react";
import styles from "./MathPage.module.scss";

interface ContentItem {
  id: string;
  title: string;
  content: string;
}

const MathPage: React.FC = () => {
  const [mathContent, setMathContent] = useState<ContentItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    // In a real app, this would fetch from an API endpoint
    // that provides the compiled JSON from your Rust code
    const fetchMathContent = async () => {
      try {
        // For demo purposes, we'll use a simple mock
        // In production, this would be: await fetch('/api/content/math')
        setTimeout(() => {
          setMathContent([
            {
              id: "math-1",
              title: "Introduction to Set Theory",
              content:
                "Set theory is the branch of mathematical logic that studies sets, which can be informally described as collections of objects. $$A \\cup B = \\{x : x \\in A \\text{ or } x \\in B\\}$$",
            },
            {
              id: "math-2",
              title: "Group Theory",
              content:
                "A group is a set equipped with an operation that combines any two elements to form a third element while satisfying four conditions: closure, associativity, identity, and invertibility. $$G = (\\mathbb{Z}, +)$$",
            },
          ]);
          setLoading(false);
        }, 500);
      } catch (err) {
        setError("Failed to load mathematics content");
        setLoading(false);
      }
    };

    fetchMathContent();
  }, []);

  // Function to render LaTeX when content changes
  useEffect(() => {
    if (mathContent.length > 0 && window.MathJax) {
      window.MathJax.typeset();
    }
  }, [mathContent]);

  if (loading)
    return <div className={styles.loading}>Loading mathematics content...</div>;
  if (error) return <div className={styles.error}>{error}</div>;

  return (
    <div className={styles.mathPage}>
      <h1>Mathematics</h1>
      <div className={styles.contentGrid}>
        {mathContent.map((item) => (
          <div key={item.id} className={styles.mathCard}>
            <h2>{item.title}</h2>
            <div dangerouslySetInnerHTML={{ __html: item.content }} />
          </div>
        ))}
      </div>
    </div>
  );
};

// Extend Window interface to include MathJax
declare global {
  interface Window {
    MathJax: any;
  }
}

export default MathPage;
