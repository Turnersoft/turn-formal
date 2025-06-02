import React, { useEffect, useState, useRef, useMemo } from "react";
import { useLocation } from "react-router-dom";
import '@shoelace-style/shoelace/dist/components/icon/icon.js';
import styles from "./math_content.module.scss";
import { 
  MathContent, 
  Definition, 
  Theorem
} from "../../models/math";
// Import the binding type directly
import type { MathematicalContent } from '../turn-render/bindings/MathematicalContent';
import DependencyGraph from "../dependency_graph/dependency_graph";
import DebugDisplay from "../debug_display/debug_display";
import DefinitionDetail from "../definition_detail/definition_detail";
import TheoremDetail from "../theorem_detail/theorem_detail";
import AbstractionViewer from './abstraction_viewer';
import { DocumentRenderer } from '../binding-renderers';
import { fetchGroupTheoryAbstractionData } from '../../../../services/mathAbstractionService';

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
 * Component that displays mathematical content from ContentBundle format
 */
const MathematicalContentSection = ({ 
  mathematicalContent, 
  title 
}: { 
  mathematicalContent: MathematicalContent[], 
  title: string 
}) => {
  if (!mathematicalContent || mathematicalContent.length === 0) {
    return null;
  }

  return (
    <div className={styles.mathematicalContentSection}>
      <h2 className={styles.sectionHeader}>
        {title}
        <span className={styles.count}>({mathematicalContent.length})</span>
      </h2>
      
      <div className={styles.contentGrid}>
        {mathematicalContent.map((content, index) => (
          <div key={content.id || index} className={styles.contentItem}>
            <DocumentRenderer content={content} className={styles.documentRenderer} />
          </div>
        ))}
      </div>
    </div>
  );
};

/**
 * Component that displays the definitions section (legacy format for backward compatibility)
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
 * Component that displays the theorems section (legacy format for backward compatibility)
 */
const TheoremsSection = ({ 
  theorems,
  theoremsRef
}: { 
  theorems: Theorem[],
  theoremsRef?: React.RefObject<HTMLDivElement>
}) => {
  // Debug log the theorems
  console.log("TheoremsSection: All theorems:", theorems);
  
  // Count theorems with proof steps (either direct or in the complex structure)
  const theoremsWithProofSteps = theorems.filter(theorem => {
    console.log(`Checking theorem ${theorem.id || 'unknown'} for proof steps:`, theorem);
    const hasDirectProofSteps = theorem.proof_steps && theorem.proof_steps.length > 0;
    const hasNestedProofSteps = (theorem.content?.Theorem?.initial_proof_state?.content?.Theorem?.proof_steps?.length ?? 0) > 0;
    
    console.log(`Theorem ${theorem.id || 'unknown'}:`, {
      hasDirectProofSteps,
      hasNestedProofSteps,
      directProofStepsCount: theorem.proof_steps?.length || 0,
      nestedProofStepsCount: theorem.content?.Theorem?.initial_proof_state?.content?.Theorem?.proof_steps?.length || 0
    });
    
    return hasDirectProofSteps || hasNestedProofSteps;
  }).length;

  console.log(`TheoremsSection: Found ${theoremsWithProofSteps} theorems with proof steps out of ${theorems.length} total`);

  return (
    <div className={styles.theoremsSection} ref={theoremsRef}>
      <h2 className={styles.sectionHeader}>
        Theorems
        <span className={styles.count}>({theorems.length})</span>
      </h2>
      
      <div className={styles.theoremsGrid}>
        {theorems.map((theorem, index) => (
          <TheoremDetail
            key={theorem.id || index}
            theorem={theorem}
            index={index}
          />
        ))}
      </div>
    </div>
  );
};

const MathContentComponent: React.FC<MathContentComponentProps> = ({
  content,
  loading,
  error,
}) => {
  const location = useLocation();
  const theoremsRef = useRef<HTMLDivElement>(null);
  const [showTheorems, setShowTheorems] = useState(false);
  const [contentReady, setContentReady] = useState(false);
  
  // Abstraction viewer state
  const [abstractionData, setAbstractionData] = useState<Record<string, any>>({});
  const [showAbstractionViewer, setShowAbstractionViewer] = useState(false);
  const [abstractionLoading, setAbstractionLoading] = useState(false);

  const scrollToDefinition = (name: string) => {
    const element = document.getElementById(`def-${name}`);
    if (element) {
      element.scrollIntoView({ behavior: "smooth" });
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

  // Load group theory definitions with abstraction levels if on the group theory page
  useEffect(() => {
    if (content && content.theory && content.theory.toLowerCase().includes('group')) {
      // Check if we're on the group theory page
      console.log("Detected group theory page, loading abstraction viewer data");
      
      const loadAbstractionData = async () => {
        setAbstractionLoading(true);
        try {
          const data = await fetchGroupTheoryAbstractionData();
          setAbstractionData(data);
          setShowAbstractionViewer(true);
        } catch (error) {
          console.error("Error loading abstraction data:", error);
        } finally {
          setAbstractionLoading(false);
        }
      };
      
      loadAbstractionData();
    } else {
      setShowAbstractionViewer(false);
    }
  }, [content]);

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

  // Get mathematical content from new format if available
  const definitionMathContent = content.mathematicalContent?.filter(mc => 
    content.contentBundles?.definitions?.content.some(c => c.id === mc.id)
  ) || [];
  
  const theoremMathContent = content.mathematicalContent?.filter(mc => 
    content.contentBundles?.theorems?.content.some(c => c.id === mc.id)
  ) || [];

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
        {/* Add abstraction viewer if we're on the group theory page */}
        {showAbstractionViewer && (
          <div className={styles.abstractionViewerContainer}>
            <h2>Group Theory Abstraction Levels</h2>
            <p>Explore group theory definitions at different abstraction levels with various display modes.</p>
            {abstractionLoading ? (
              <div className={styles.loadingIndicator}>Loading abstraction data...</div>
            ) : Object.keys(abstractionData).length > 0 ? (
              <AbstractionViewer sectionData={abstractionData} />
            ) : (
              <div className={styles.emptyState}>No abstraction data available</div>
            )}
          </div>
        )}
        
        <div className={styles.contentGrid}>
          {/* Only show structured content */}
          <>
            {/* NEW FORMAT: Mathematical Content from ContentBundle */}
            {definitionMathContent.length > 0 && !showTheorems && (
              <MathematicalContentSection 
                mathematicalContent={definitionMathContent} 
                title="Mathematical Definitions" 
              />
            )}
            
            {theoremMathContent.length > 0 && (
              <MathematicalContentSection 
                mathematicalContent={theoremMathContent} 
                title="Theorems" 
              />
            )}

            {/* LEGACY FORMAT: Fallback for backward compatibility */}
            {definitionMathContent.length === 0 && !showTheorems && content.definitions && content.definitions.length > 0 && (
              <DefinitionsSection 
                definitions={content.definitions} 
                onScroll={scrollToDefinition} 
              />
            )}

            {theoremMathContent.length === 0 && content.theorems && content.theorems.length > 0 && (
              <TheoremsSection 
                theorems={content.theorems} 
                theoremsRef={theoremsRef} 
              />
            )}

            {/* Empty State */}
            {definitionMathContent.length === 0 && theoremMathContent.length === 0 &&
              (!content.theorems || content.theorems.length === 0) &&
              (!content.definitions || content.definitions.length === 0) &&
              !showAbstractionViewer && (
                <div className={styles.emptyState}>
                  No content found in this theory. The JSON files may be
                  empty.
                </div>
              )}
          </>
        </div>
      </div>
    </div>
  );
};

export default MathContentComponent;
