import React, { useEffect, useState, useMemo, useRef, useCallback } from "react";
import { useLocation } from "react-router-dom";
import styles from "./math_content.module.scss";
import { MathContent, Definition, Member, Theorem } from "../../models/math";
import DependencyGraph from "../dependency_graph/dependency_graph";
import DebugDisplay from "../debug_display/debug_display";
import DefinitionDetail from "../definition_detail/definition_detail";
import TheoremDetail from "../theorem_detail/theorem_detail";

// Helper function to extract all type names from a type string
function extractTypeNames(typeStr: string): string[] {
  if (!typeStr || typeof typeStr !== "string") return [];

  // Array to store all found type names
  const typeNames: string[] = [];

  // Find all capitalized type names (standard naming convention)
  const typeRegex = /\b([A-Z][a-zA-Z0-9_]*)\b/g;
  let match;

  while ((match = typeRegex.exec(typeStr)) !== null) {
    const [, typeName] = match;
    // Add the type name if it's not already in the array
    if (!typeNames.includes(typeName)) {
      typeNames.push(typeName);
    }
  }

  // Handle nested generic types like Vec<TypeName>
  const genericTypeRegex = /<([^<>]+)>/g;
  let genericMatch;

  while ((genericMatch = genericTypeRegex.exec(typeStr)) !== null) {
    const innerContent = genericMatch[1];
    // For each inner content, recursively find type names
    const innerTypes = extractTypeNames(innerContent);

    // Add all inner types
    innerTypes.forEach((innerType) => {
      if (!typeNames.includes(innerType)) {
        typeNames.push(innerType);
      }
    });
  }

  return typeNames;
}

interface MathContentComponentProps {
  content: MathContent | null;
  loading: boolean;
  error: string | null;
}

/**
 * Formats a theory name for display
 */
const formatTheoryName = (name: string): string => {
  return name
    .split("_")
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join(" ");
};

/**
 * Component that displays the definitions section
 */
const DefinitionsSection = ({ 
  definitions, 
  onScroll 
}: { 
  definitions: Definition[], 
  onScroll: (name: string) => void 
}) => {
  // Create a map of definitions for quick lookup
  const definitionMap = new Map<string, Definition>();
  definitions.forEach((def) => {
    definitionMap.set(def.name, def);
  });

  // Order definitions based on dependencies
  const orderedDefinitions = [...definitions];

  // Build dependency structure for visualization
  const dependencyStructure = {
    nodes: definitions.map((def) => ({ id: def.name, label: def.name })),
    edges: [] as Array<{from: string, to: string}>
  };

  // Add edges based on member type references
  definitions.forEach((def) => {
    def.members?.forEach((member) => {
      definitions.forEach((targetDef) => {
        if (member.type?.includes(targetDef.name)) {
          dependencyStructure.edges.push({
            from: def.name,
            to: targetDef.name,
          });
        }
      });
    });
  });

  return (
    <div className={styles.definitionsSection}>
      <h2 className={styles.sectionHeader}>
        Definitions
        <span className={styles.count}>({definitions.length})</span>
      </h2>

      {/* Definition Structure Overview */}
      <div className={styles.structureOverview}>
        <h3>Structure Overview</h3>
        <div className={styles.definitionList}>
          {orderedDefinitions.map((definition, index) => (
            <div
              key={`overview-${index}`}
              className={styles.definitionItem}
              onClick={() => onScroll(definition.name)}
              onMouseEnter={() => {
                // When hovering an item in the structure overview,
                // dispatch event to highlight it in the graph
                const event = new CustomEvent("overviewHover", {
                  detail: { nodeId: definition.name },
                });
                document.dispatchEvent(event);
              }}
              onMouseLeave={() => {
                // Clear highlight when mouse leaves
                const event = new CustomEvent("overviewHover", {
                  detail: { nodeId: null },
                });
                document.dispatchEvent(event);
              }}
              data-node-id={definition.name}
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
        {dependencyStructure.nodes.length > 0 && (
          <DependencyGraph definitions={definitions} />
        )}
      </div>

      {/* Detailed Definitions with Links */}
      <div className={styles.detailedDefinitions}>
        {orderedDefinitions.map((definition, index) => (
          <DefinitionDetail
            key={index}
            definition={definition}
            definitionMap={definitionMap}
            onScrollToDefinition={onScroll}
          />
        ))}
      </div>
    </div>
  );
};

/**
 * Component that displays the theorems section
 */
const TheoremsSection = ({ 
  theorems,
  theoremsRef
}: { 
  theorems: Theorem[],
  theoremsRef?: React.RefObject<HTMLDivElement>
}) => {
  return (
    <div className={styles.theoremsSection} ref={theoremsRef}>
      <h2 className={styles.sectionHeader}>
        Theorems
        <span className={styles.count}>({theorems.length})</span>
        {theorems.length > 0 && 
          <small className={styles.debugInfo}>
            <span title="Theorems loaded successfully">✓</span>
          </small>
        }
      </h2>
      {theorems.map((theorem: Theorem, index: number) => (
        <TheoremDetail key={index} theorem={theorem} />
      ))}
    </div>
  );
};

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
  const [showTheorems, setShowTheorems] = useState(false);
  const theoremsRef = useRef<HTMLDivElement>(null);
  const location = useLocation();

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
    if (content && Array.isArray(content.definitions)) {
      content.definitions.forEach((def: Definition) => {
        try {
          if (def && def.name) {
            // Make a safe deep copy to prevent mutation issues
            const safeDef = { ...def };

            // Ensure members array is always defined and valid
            if (!safeDef.members || !Array.isArray(safeDef.members)) {
              console.warn(
                `Definition ${safeDef.name} has invalid members. Adding empty array.`
              );
              safeDef.members = [];
            }

            // Ensure each member has the required fields
            safeDef.members = safeDef.members.map((member) => {
              if (!member)
                return { name: "unknown", type: "unknown", docs: "" };

              // Handle both type and type_info fields
              const hasType = !!member.type;
              const hasTypeInfo = !!member.type_info;

              // Create a normalized member with all fields
              return {
                ...member,
                name: member.name || "unnamed",
                type: hasType ? member.type : null,
                type_info: hasTypeInfo ? member.type_info : null,
                docs: member.docs || "",
              };
            });

            map.set(safeDef.name, safeDef);
          }
        } catch (error) {
          console.error(
            `Error processing definition: ${def?.name || "unknown"}`,
            error
          );
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
      if (def.members) {
        def.members.forEach((member: Member) => {
          // Process both type and type_info fields
          const typeFields = [];
          if (typeof member.type === "string") typeFields.push(member.type);
          if (typeof member.type_info === "string")
            typeFields.push(member.type_info);

          // Process each type field
          typeFields.forEach((typeField) => {
            // Extract all type names including from complex nested types
            const typeNames = extractTypeNames(typeField);

            // Add dependencies for each found type
            typeNames.forEach((typeName) => {
              if (defMap.has(typeName) && typeName !== def.name) {
                // This definition depends on typeName
                const dependencies = dependencyGraph.get(def.name);
                if (dependencies) {
                  dependencies.add(typeName);
                }
              }
            });
          });
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
          // Process both type and type_info fields
          const typeFields = [];
          if (typeof member.type === "string") typeFields.push(member.type);
          if (typeof member.type_info === "string")
            typeFields.push(member.type_info);

          // Process each type field
          typeFields.forEach((typeField) => {
            // Extract all type names including from complex nested types
            const typeNames = extractTypeNames(typeField);

            // Add links for each found type
            typeNames.forEach((typeName) => {
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
            });
          });
        });
      }
    });

    return { nodes, links };
  }, [content, definitionMap]);

  // Check if the current URL contains "theorem" to auto-focus on theorems section
  useEffect(() => {
    const path = location.pathname;
    const isTheoremFile = path.toLowerCase().includes('theorem');
    
    console.log("Path check for theorem focus:", path, isTheoremFile);
    setShowTheorems(isTheoremFile);
    
    // If it's a theorem file and we have theorems content, scroll to the theorem section
    if (isTheoremFile && content?.theorems && content.theorems.length > 0 && theoremsRef.current) {
      setTimeout(() => {
        theoremsRef.current?.scrollIntoView({ behavior: 'smooth' });
      }, 300); // Small delay to ensure the component is rendered
    }
  }, [location.pathname, content]);

  // Render MathJax whenever the content changes
  useEffect(() => {
    if (!loading && content) {
      // Log theorem data for debugging
      if (content.theorems && content.theorems.length > 0) {
        console.log(`Loaded ${content.theorems.length} theorems for ${content.theory}:`, content.theorems);
      }
      
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
      className={`${styles.mathContentArea} ${styles.fadeIn} ${
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
              {/* Definitions Section */}
              {!showTheorems && content.definitions && content.definitions.length > 0 && (
                <DefinitionsSection 
                  definitions={content.definitions} 
                  onScroll={scrollToDefinition} 
                />
              )}

              {/* Theorems Section */}
              {content.theorems && content.theorems.length > 0 && (
                <TheoremsSection 
                  theorems={content.theorems} 
                  theoremsRef={theoremsRef} 
                />
              )}

              {/* Empty State */}
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
