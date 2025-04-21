import React from "react";
import styles from "../../components/mathcontent/mathcontent.module.scss";

interface SyntaxHighlightedJsonProps {
  jsonString: string;
}

/**
 * Component for rendering JSON with syntax highlighting
 */
const SyntaxHighlightedJson: React.FC<SyntaxHighlightedJsonProps> = ({
  jsonString,
}) => {
  // Process JSON string with safer line-by-line processing
  const processJsonForDisplay = (json: string): React.ReactNode[] => {
    try {
      const lines = json.split("\n");
      return lines.map((line, index) => {
        // Apply appropriate classes for different parts of JSON
        const processedLine = line
          // Handle JSON keys (always in double quotes followed by colon)
          .replace(/"([^"]+)":/g, '<span class="json-key">"$1":</span>')
          // Handle string values (in double quotes, not followed by colon)
          .replace(/"([^"]*)"(?!:)/g, '<span class="json-string">"$1"</span>')
          // Handle booleans
          .replace(/\b(true|false)\b/g, '<span class="json-boolean">$1</span>')
          // Handle numbers
          .replace(/\b(\d+(\.\d+)?)\b/g, '<span class="json-number">$1</span>')
          // Handle null
          .replace(/\bnull\b/g, '<span class="json-null">null</span>');

        return (
          <div
            key={index}
            dangerouslySetInnerHTML={{ __html: processedLine }}
          />
        );
      });
    } catch (error) {
      console.error("Error processing JSON for display:", error);
      return [<pre key="error">{jsonString}</pre>];
    }
  };

  return (
    <div className={styles.jsonContainer}>
      <pre className={styles.jsonPre}>
        <code className={styles.jsonCode}>
          {processJsonForDisplay(jsonString)}
        </code>
      </pre>
    </div>
  );
};

export default SyntaxHighlightedJson;
