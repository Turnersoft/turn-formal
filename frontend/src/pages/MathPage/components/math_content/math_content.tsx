import React, { useEffect, useState, useMemo } from "react";
import styles from "./math_content.module.scss";
import { MathContent, Definition, Member, Theorem } from "../../models/math";
import { formatTheoryName } from "../../utils/formatters";

// Import separate components
import DebugDisplay from "../debug_display/debug_display";
import DependencyGraph from "../dependency_graph/dependency_graph";
import DefinitionDetail from "../definition_detail/definition_detail";
import TheoremDetail from "../theorem_detail/theorem_detail";

interface MathContentComponentProps {
  content: MathContent | null;
  loading: boolean;
  error: string | null;
}

/**
 * Main component to display mathematical content from JSON files
 */
const MathContentComponent: React.FC<MathContentComponentProps> = ({
  content,
  loading,
  error,
}) => {
  // State hooks - declare all hooks up front
  const [contentReady, setContentReady] = useState(false);
  const [showJsonView] = useState(false);

  // Function to scroll to a definition when clicked
  const scrollToDefinition = (name: string) => {
    const element = document.getElementById(`definition-${name}`);
    if (element) {
      element.scrollIntoView({ behavior: "smooth" });
      // Add a highlight effect
      element.classList.add(styles.highlightedDefinition);
      setTimeout(() => {
        element.classList.remove(styles.highlightedDefinition);
      }, 2000);
    }
  };

  // Build a map of all definitions for linking
  const definitionMap = useMemo(() => {
    const map = new Map<string, Definition>();
    if (content && content.definitions) {
      content.definitions.forEach((def: Definition) => {
        if (def && def.name) {
          // Ensure members array is always defined
          if (!def.members || !Array.isArray(def.members)) {
            console.warn(
              `Definition ${def.name} has invalid members. Adding placeholder.`
            );
            def.members = [
              {
                name: "placeholder",
                type: "String",
                docs: "Placeholder member",
              },
            ];
          }
          map.set(def.name, def);
        }
      });
    }
    return map;
  }, [content]);

  // Order definitions to show dependencies first
  const orderedDefinitions = useMemo(() => {
    if (!content || !content.definitions || content.definitions.length === 0) {
      return [];
    }

    // Create a dependency graph
    const dependencyGraph = new Map<string, Set<string>>();
    const defMap = new Map<string, Definition>();

    // Initialize graph
    content.definitions.forEach((def: Definition) => {
      dependencyGraph.set(def.name, new Set());
      defMap.set(def.name, def);
    });

    // Build dependencies
    content.definitions.forEach((def: Definition) => {
      // Check member types for dependencies
      if (def.members) {
        def.members.forEach((member: Member) => {
          // Look for types in the member.type field
          const typeRegex = /\b([A-Z][a-zA-Z0-9_]*)\b/g;
          let match;

          while ((match = typeRegex.exec(member.type)) !== null) {
            const [, typeName] = match;
            if (defMap.has(typeName) && typeName !== def.name) {
              // This definition depends on typeName
              const dependencies = dependencyGraph.get(def.name);
              if (dependencies) {
                dependencies.add(typeName);
              }
            }
          }
        });
      }
    });

    // Topological sort to order definitions
    const result: Definition[] = [];
    const visited = new Set<string>();
    const visiting = new Set<string>();

    function visit(name: string) {
      if (visited.has(name)) return;
      if (visiting.has(name)) return; // Handle cycles

      visiting.add(name);

      // Visit dependencies first
      const dependencies = dependencyGraph.get(name);
      if (dependencies) {
        dependencies.forEach((dep) => visit(dep));
      }

      visiting.delete(name);
      visited.add(name);

      const def = defMap.get(name);
      if (def) result.push(def);
    }

    // Visit all nodes
    defMap.forEach((_, name) => {
      if (!visited.has(name)) {
        visit(name);
      }
    });

    return result;
  }, [content]);

  // Calculate dependencies for graph visualization
  const dependencyStructure = useMemo(() => {
    if (!content || !content.definitions || content.definitions.length === 0) {
      return { nodes: [], links: [] };
    }

    // Extract nodes and links for the dependency graph
    const nodes: Array<{ id: string; kind: string }> = [];
    const links: Array<{ source: string; target: string }> = [];
    const processedLinks = new Set<string>();

    // Add all definitions as nodes
    content.definitions.forEach((def: Definition) => {
      nodes.push({
        id: def.name,
        kind: def.kind,
      });
    });

    // Find dependencies from member types
    content.definitions.forEach((def: Definition) => {
      if (def.members) {
        def.members.forEach((member: Member) => {
          // Look for types in the member.type field
          const typeRegex = /\b([A-Z][a-zA-Z0-9_]*)\b/g;
          let match;

          while ((match = typeRegex.exec(member.type)) !== null) {
            const [, typeName] = match;
            if (definitionMap.has(typeName) && typeName !== def.name) {
              // Create a link if we haven't already processed this pair
              const linkKey = `${def.name}-${typeName}`;
              if (!processedLinks.has(linkKey)) {
                links.push({
                  source: def.name,
                  target: typeName,
                });
                processedLinks.add(linkKey);
              }
            }
          }
        });
      }
    });

    return { nodes, links };
  }, [content, definitionMap]);

  // Render MathJax whenever the content changes
  useEffect(() => {
    if (!loading && content) {
      // Set a small delay to ensure DOM is updated before marking content as ready
      setTimeout(() => {
        setContentReady(true);
      }, 100);
    }
  }, [content, loading]);

  // Early rendering states
  if (loading) {
    return (
      <div className={styles.loadingIndicator}>
        Loading mathematics content...
      </div>
    );
  }

  if (error) {
    return <div className={styles.errorMessage}>{error}</div>;
  }

  if (!content) {
    return (
      <div className={styles.emptyState}>
        <h2>No theory selected</h2>
        <p>Please select a theory from the sidebar to view its content.</p>
      </div>
    );
  }

  // Main render output
  return (
    <div
      className={`${styles.contentArea} ${styles.fadeIn} ${
        contentReady ? styles.ready : ""
      }`}
    >
      <div className={styles.contentHeader}>
        <h1>{formatTheoryName(content.theory)} Theory</h1>

        <DebugDisplay data={content} title="Theory" />
      </div>

      <div className={styles.mainContent}>
        <div className={styles.contentGrid}>
          {/* Only show structured content if not showing JSON view */}
          {!showJsonView && (
            <>
              {/* Definitions section with dependency visualization */}
              {content.definitions && content.definitions.length > 0 && (
                <div className={styles.definitionsSection}>
                  <h2 className={styles.sectionHeader}>
                    Definitions
                    <span className={styles.count}>
                      ({content.definitions.length})
                    </span>
                  </h2>

                  {/* Definition Structure Overview */}
                  <div className={styles.structureOverview}>
                    <h3>Structure Overview</h3>
                    <div className={styles.definitionList}>
                      {orderedDefinitions.map((definition, index) => (
                        <div
                          key={`overview-${index}`}
                          className={styles.definitionItem}
                          onClick={() => scrollToDefinition(definition.name)}
                        >
                          <span className={styles.definitionKind}>
                            {definition.kind}
                          </span>
                          <span className={styles.definitionName}>
                            {definition.name}
                          </span>
                        </div>
                      ))}
                    </div>

                    {/* Render the dependency graph if we have definitions */}
                    {dependencyStructure.nodes.length > 0 &&
                      content.definitions && (
                        <DependencyGraph definitions={content.definitions} />
                      )}
                  </div>

                  {/* Detailed Definitions with Links */}
                  <div className={styles.detailedDefinitions}>
                    {orderedDefinitions.map((definition, index) => (
                      <DefinitionDetail
                        key={index}
                        definition={definition}
                        definitionMap={definitionMap}
                        onScrollToDefinition={scrollToDefinition}
                      />
                    ))}
                  </div>
                </div>
              )}

              {/* Theorems section */}
              {content.theorems && content.theorems.length > 0 && (
                <div className={styles.theoremsSection}>
                  <h2 className={styles.sectionHeader}>
                    Theorems
                    <span className={styles.count}>
                      ({content.theorems.length})
                    </span>
                  </h2>
                  {content.theorems.map((theorem: Theorem, index: number) => (
                    <TheoremDetail key={index} theorem={theorem} />
                  ))}
                </div>
              )}

              {(!content.theorems || content.theorems.length === 0) &&
                (!content.definitions || content.definitions.length === 0) && (
                  <div className={styles.emptyState}>
                    No content found in this theory. The JSON files may be
                    empty.
                  </div>
                )}
            </>
          )}
        </div>
      </div>
    </div>
  );
};

export default MathContentComponent;
