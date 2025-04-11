import React from "react";
import styles from "./type_link.module.scss";
import { Definition } from "../../models/math";

interface TypeLinkProps {
  typeStr: string | null | undefined;
  definitionMap: Map<string, Definition>;
  onScrollToDefinition: (name: string) => void;
}

/**
 * Component for rendering type names with links to their definitions
 */
const TypeLink: React.FC<TypeLinkProps> = ({
  typeStr,
  definitionMap,
  onScrollToDefinition,
}) => {
  // Safety check - if typeStr is not a string or is empty, render an empty fragment
  if (typeof typeStr !== "string" || !typeStr) {
    return <span className={styles.invalidType}>unknown</span>;
  }

  const typeRegex = /\b([A-Z][a-zA-Z0-9_]*)\b/g;
  let match;
  let lastIndex = 0;
  const parts: JSX.Element[] = [];

  try {
    // Process the type string safely
    const typeText = typeStr.replace(/<([^>]*)>/g, "&lt;$1&gt;");

    // Clone regex to prevent state issues
    const regex = new RegExp(typeRegex.source, typeRegex.flags);

    while ((match = regex.exec(typeText)) !== null) {
      const [fullMatch, typeName] = match;

      // Add text before this match
      if (match.index > lastIndex) {
        parts.push(
          <span key={`text-${lastIndex}`}>
            {typeText.substring(lastIndex, match.index)}
          </span>
        );
      }

      // Check if this type exists in our definitions
      if (definitionMap.has(typeName)) {
        // It's a type we know about, so make it a link
        parts.push(
          <span
            key={`link-${match.index}`}
            className={styles.typeLink}
            onClick={() => onScrollToDefinition(typeName)}
            title={`View definition of ${typeName}`}
          >
            {fullMatch}
          </span>
        );
      } else {
        // Just a regular type name, no link
        parts.push(<span key={`text-${match.index}`}>{fullMatch}</span>);
      }

      lastIndex = match.index + fullMatch.length;
    }

    // Add any remaining text
    if (lastIndex < typeText.length) {
      parts.push(
        <span key={`text-${lastIndex}`}>{typeText.substring(lastIndex)}</span>
      );
    }

    return <>{parts}</>;
  } catch (error) {
    console.error(`Error processing type string: ${typeStr}`, error);
    return <span className={styles.invalidType}>{typeStr}</span>;
  }
};

export default TypeLink;
