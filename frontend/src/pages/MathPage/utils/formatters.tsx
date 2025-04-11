/**
 * Format a type name for display, handling generic syntax
 */
export const formatTypeName = (type: string): string => {
  return type.replace(/<([^>]*)>/g, "&lt;$1&gt;");
};

/**
 * Format a theory name for display, converting snake_case to Title Case
 */
export const formatTheoryName = (name: string): string => {
  return name
    .split("_")
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join(" ");
};

/**
 * Render text that may contain mathematical notation
 */
export const renderMathText = (text: string): JSX.Element => {
  // Simple heuristic: if text contains potential LaTeX markers, use dangerouslySetInnerHTML
  const hasMathContent =
    text.includes("$") ||
    text.includes("\\") ||
    text.includes("{") ||
    text.includes("}");

  if (hasMathContent) {
    return <span dangerouslySetInnerHTML={{ __html: text }} />;
  }
  return <span>{text}</span>;
};
