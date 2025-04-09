import React, { useEffect, useState } from "react";
import styles from "./MathPage.module.scss";
import Sidebar from "./components/sidebar/sidebar";
import { useNavigate } from "react-router-dom";
import { MathContent as MathContentType } from "../../types/mathContent";
import MathContentComponent from "./components/mathcontent/mathcontent";

// Interfaces for our data structures
interface FolderNode {
  name: string;
  path: string;
  isDirectory: boolean;
  children?: FolderNode[];
}

interface ContentItem {
  id: string;
  title: string;
  content: string;
  type: "theorem" | "theory";
  path: string;
  metadata?: {
    foundationalTheory?: string;
    tags?: string[];
    dateCreated?: string;
    dateModified?: string;
  };
}

// Interface for parsed Rust data
interface RustMathData {
  theorems: {
    id: string;
    name: string;
    description: string;
    proofState?: any;
  }[];
  definitions: {
    id: string;
    name: string;
    content: string;
  }[];
}

const MathPage: React.FC = () => {
  const [mathContent, setMathContent] = useState<ContentItem[]>([]);
  const [formattedContent, setFormattedContent] =
    useState<MathContentType | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [selectedFolder, setSelectedFolder] = useState<string>("");
  const [rootFolder, setRootFolder] = useState<string>("");
  const [folderTree, setFolderTree] = useState<FolderNode[]>([]);
  const [rustData, setRustData] = useState<RustMathData | null>(null);
  const navigate = useNavigate();

  // Initial setup to let user choose root folder
  useEffect(() => {
    const setupRootFolder = async () => {
      try {
        // Simulating user selecting a root folder
        const defaultRoot =
          "/Users/johndoe/Documents/company/codetree/turn/turn-engine/src/turn-formal";
        setRootFolder(defaultRoot);

        // Load folder structure once root is selected
        const structure = await scanDirectory(defaultRoot);
        setFolderTree(structure);

        // Default to the first folder
        if (structure.length > 0 && structure[0].path) {
          setSelectedFolder(structure[0].path);
        }
      } catch (err) {
        console.error("Failed to setup root folder", err);
        setError(
          "Failed to setup the root folder. Please check console for details."
        );
      }
    };

    setupRootFolder();
  }, []);

  // Load selected folder content
  useEffect(() => {
    const loadMathContent = async () => {
      if (!selectedFolder) return;

      setLoading(true);
      setError(null);

      try {
        // Get content of the selected folder
        const content = await getFolderContent(rootFolder, selectedFolder);
        setMathContent(content);

        // Try to load Rust data if available
        try {
          const rustDataResult = await executeRustCode(
            rootFolder,
            selectedFolder
          );
          setRustData(rustDataResult);

          // Convert Rust data to the format expected by MathContentComponent
          if (rustDataResult) {
            const formattedData = await formatRustData(
              rustDataResult,
              selectedFolder
            );
            setFormattedContent(formattedData);
          }
        } catch (rustErr) {
          console.warn("Could not execute Rust code for this folder", rustErr);
          setRustData(null);
          setFormattedContent(null);
        }

        setLoading(false);
      } catch (err) {
        console.error("Failed to load mathematics content", err);
        setError(
          "Failed to load mathematics content. Check console for details."
        );
        setLoading(false);
      }
    };

    loadMathContent();
  }, [selectedFolder, rootFolder]);

  // Function to format Rust data for the MathContent component
  const formatRustData = async (
    _rustData: RustMathData,
    folderPath: string
  ): Promise<MathContentType> => {
    try {
      // In a real implementation, we would fetch the JSON data from a server
      // that runs the Rust code from export.rs to generate the data

      // For simplicity in this demo, we'll use mock data based on the folder path
      if (folderPath.includes("/subjects/math/theorem")) {
        // Mock theorem data
        return {
          definitions: [],
          theorems: [
            {
              id: "inverse_uniqueness",
              name: "Group Inverse Uniqueness",
              statement:
                "For all elements g in a group G, if g*h₁ = e and g*h₂ = e, then h₁ = h₂",
              description:
                "This theorem proves that inverses in a group are unique",
              proofSteps: [
                {
                  id: "step_1",
                  description: "Assume g*h₁ = e and g*h₂ = e",
                  formula: "g*h₁ = e, g*h₂ = e",
                  tacticName: "Intro",
                  justification: "Given assumptions",
                },
                {
                  id: "step_2",
                  description: "Multiply the first equation by h₂ on the left",
                  formula: "h₂*(g*h₁) = h₂*e",
                  tacticName: "Multiply",
                  justification: "Multiplication is well-defined",
                },
                {
                  id: "step_3",
                  description: "Use associativity: (h₂*g)*h₁ = h₂*e",
                  formula: "(h₂*g)*h₁ = h₂",
                  tacticName: "Associativity",
                  justification: "Group axiom of associativity",
                },
                {
                  id: "step_4",
                  description: "Use second assumption: h₂*g = e",
                  formula: "e*h₁ = h₂",
                  tacticName: "Substitute",
                  justification: "Substitution from second assumption",
                },
                {
                  id: "step_5",
                  description: "Use identity property: e*h₁ = h₁",
                  formula: "h₁ = h₂",
                  tacticName: "Identity",
                  justification: "Group axiom of identity",
                },
              ],
              tags: ["group theory", "basic properties"],
            },
            {
              id: "lagrange",
              name: "Lagrange's Theorem",
              statement:
                "If H is a subgroup of a finite group G, then the order of H divides the order of G",
              description:
                "This foundational theorem relates the size of a subgroup to the size of the group",
              proofSteps: [
                {
                  id: "step_1",
                  description: "Define left cosets of H in G",
                  formula: "gH = \\{gh : h \\in H\\} \\text{ for } g \\in G",
                  tacticName: "Define",
                  justification: "Standard definition of left cosets",
                },
                {
                  id: "step_2",
                  description: "Show that cosets partition G",
                  formula:
                    "G = \\bigcup_{g \\in G} gH \\text{ and } gH \\cap g'H = \\emptyset \\text{ or } gH = g'H",
                  tacticName: "Prove",
                  justification: "Equivalence relation properties",
                },
                {
                  id: "step_3",
                  description: "Each coset has |H| elements",
                  formula: "|gH| = |H| \\text{ for all } g \\in G",
                  tacticName: "Prove",
                  justification: "Bijection between H and gH",
                },
                {
                  id: "step_4",
                  description: "Let [G:H] be the number of distinct cosets",
                  formula: "[G:H] = \\text{number of distinct cosets}",
                  tacticName: "Define",
                  justification: "Definition of index",
                },
                {
                  id: "step_5",
                  description: "Then |G| = [G:H] * |H|",
                  formula: "|G| = [G:H] \\cdot |H|",
                  tacticName: "Conclude",
                  justification: "Counting elements in the partition",
                },
              ],
              tags: ["group theory", "finite groups", "subgroups"],
            },
          ],
        };
      } else if (folderPath.includes("/subjects/math/theories/groups")) {
        // Mock group theory definitions
        return {
          definitions: [
            {
              id: "group",
              name: "Group",
              description:
                "A group is a set G equipped with a binary operation that combines any two elements to form a third element. The operation must satisfy four conditions: closure, associativity, identity, and invertibility.",
              type: "struct",
              fields: [
                {
                  name: "base_set",
                  type: "Set",
                  description: "The underlying set",
                },
                {
                  name: "operation",
                  type: "GroupOperation",
                  description: "The binary operation with its properties",
                },
                {
                  name: "properties",
                  type: "Vec<GroupProperty>",
                  description: "Properties specific to the group structure",
                },
              ],
            },
            {
              id: "group_operation_variant",
              name: "Group Operation Variant",
              description:
                "Types of operations that can be used in group structures",
              type: "enum",
              variants: [
                {
                  id: "multiplication",
                  name: "Multiplication",
                  description:
                    "Standard multiplication (used in most abstract groups)",
                },
                {
                  id: "addition",
                  name: "Addition",
                  description: "Addition (used in additive groups)",
                },
                {
                  id: "composition",
                  name: "Composition",
                  description: "Composition (used in transformation groups)",
                },
                {
                  id: "matrix_multiplication",
                  name: "Matrix Multiplication",
                  description: "Matrix multiplication (for matrix groups)",
                },
                {
                  id: "direct_product",
                  name: "Direct Product",
                  description: "Direct product of groups",
                },
                {
                  id: "semidirect_product",
                  name: "Semidirect Product",
                  description: "Semidirect product of groups",
                },
                {
                  id: "free_product",
                  name: "Free Product",
                  description:
                    "Free product (used in combinatorial group theory)",
                },
              ],
            },
            {
              id: "abelian_property_variant",
              name: "Abelian Property Variant",
              description: "Describes whether a group is commutative or not",
              type: "enum",
              variants: [
                {
                  id: "abelian",
                  name: "Abelian",
                  description: "Commutative",
                },
                {
                  id: "non_abelian",
                  name: "NonAbelian",
                  description: "Non-commutative",
                },
              ],
            },
          ],
          theorems: [],
        };
      }

      // Default empty data
      return { definitions: [], theorems: [] };
    } catch (error) {
      console.error("Error formatting Rust data:", error);
      return { definitions: [], theorems: [] };
    }
  };

  // Function to render LaTeX when content changes
  useEffect(() => {
    if (mathContent.length > 0 && window.MathJax) {
      window.MathJax.typeset();
    }
  }, [mathContent]);
  // Scan directory recursively
  const scanDirectory = async (_dirPath: string): Promise<FolderNode[]> => {
    try {
      // In a real Tauri app, we would use fs.readDir
      // For this demo, we're mocking the functionality
      const mockFolders = [
        {
          name: "subjects",
          path: "/subjects",
          isDirectory: true,
          children: [
            {
              name: "math",
              path: "/subjects/math",
              isDirectory: true,
              children: [
                {
                  name: "theorem",
                  path: "/subjects/math/theorem",
                  isDirectory: true,
                  children: [
                    {
                      name: "core.rs",
                      path: "/subjects/math/theorem/core.rs",
                      isDirectory: false,
                    },
                    {
                      name: "proof.rs",
                      path: "/subjects/math/theorem/proof.rs",
                      isDirectory: false,
                    },
                    {
                      name: "expressions.rs",
                      path: "/subjects/math/theorem/expressions.rs",
                      isDirectory: false,
                    },
                    {
                      name: "properties.rs",
                      path: "/subjects/math/theorem/properties.rs",
                      isDirectory: false,
                    },
                    {
                      name: "relations.rs",
                      path: "/subjects/math/theorem/relations.rs",
                      isDirectory: false,
                    },
                  ],
                },
                {
                  name: "theories",
                  path: "/subjects/math/theories",
                  isDirectory: true,
                  children: [
                    {
                      name: "groups",
                      path: "/subjects/math/theories/groups",
                      isDirectory: true,
                      children: [
                        {
                          name: "definitions.rs",
                          path: "/subjects/math/theories/groups/definitions.rs",
                          isDirectory: false,
                        },
                      ],
                    },
                    {
                      name: "rings",
                      path: "/subjects/math/theories/rings",
                      isDirectory: true,
                      children: [
                        {
                          name: "definitions.rs",
                          path: "/subjects/math/theories/rings/definitions.rs",
                          isDirectory: false,
                        },
                      ],
                    },
                  ],
                },
              ],
            },
          ],
        },
      ];

      return mockFolders;
    } catch (error) {
      console.error("Failed to scan directory:", error);
      throw new Error(
        "Failed to read directory structure. Please check permissions."
      );
    }
  };

  // Get content of a folder
  const getFolderContent = async (
    _rootPath: string,
    folderPath: string
  ): Promise<ContentItem[]> => {
    try {
      // In a real app, we would read actual files
      // For this demo, we're returning mock data based on the folder path

      // Mock data for different paths
      if (folderPath.includes("/subjects/math/theorem")) {
        return [
          {
            id: "theorem-core.rs",
            title: "Theorem Core",
            content: "Defines the core theorem structure and interfaces",
            type: "theory",
            path: "/subjects/math/theorem/core.rs",
            metadata: {
              foundationalTheory: "Mathematics",
              tags: ["theory", "theorems"],
              dateModified: "2023-08-15",
            },
          },
          {
            id: "theorem-proof.rs",
            title: "Proof Structure",
            content:
              "Implements a rich proof structure for mathematical theorems",
            type: "theory",
            path: "/subjects/math/theorem/proof.rs",
            metadata: {
              foundationalTheory: "Logic",
              tags: ["proof", "logic"],
              dateModified: "2023-08-20",
            },
          },
        ];
      } else if (folderPath.includes("/subjects/math/theories/groups")) {
        return [
          {
            id: "theory-definitions.rs",
            title: "Group Theory Definitions",
            content: "Defines group theory concepts and structures",
            type: "theory",
            path: "/subjects/math/theories/groups/definitions.rs",
            metadata: {
              foundationalTheory: "Abstract Algebra",
              tags: ["groups", "algebra"],
              dateModified: "2023-07-10",
            },
          },
        ];
      } else if (folderPath.includes("/subjects/math/theories/rings")) {
        return [
          {
            id: "theory-definitions.rs",
            title: "Ring Theory Definitions",
            content: "Defines ring theory concepts and structures",
            type: "theory",
            path: "/subjects/math/theories/rings/definitions.rs",
            metadata: {
              foundationalTheory: "Abstract Algebra",
              tags: ["rings", "algebra"],
              dateModified: "2023-07-15",
            },
          },
        ];
      }

      return [];
    } catch (error) {
      console.error("Failed to get folder content:", error);
      throw new Error(
        "Failed to read folder content. Please check permissions."
      );
    }
  };

  // Execute Rust code to get data
  const executeRustCode = async (
    _rootPath: string,
    folderPath: string
  ): Promise<RustMathData> => {
    try {
      // In a real app, this would invoke the Rust code
      // For this demo, we're returning mock data

      // Mock data for different paths
      if (folderPath.includes("/subjects/math/theorem")) {
        return {
          theorems: [
            {
              id: "theorem1",
              name: "Pythagorean Theorem",
              description:
                "In a right triangle, the square of the length of the hypotenuse equals the sum of the squares of the lengths of the other two sides.",
              proofState: { status: "Complete" },
            },
            {
              id: "theorem2",
              name: "Law of Cosines",
              description:
                "Relates the lengths of the sides of a triangle to the cosine of one of its angles.",
              proofState: { status: "InProgress" },
            },
          ],
          definitions: [
            {
              id: "def1",
              name: "Group",
              content:
                "A group is a set equipped with an operation that combines any two elements to form a third element.",
            },
            {
              id: "def2",
              name: "Ring",
              content:
                "A ring is a set equipped with two binary operations satisfying properties generalizing those of addition and multiplication.",
            },
          ],
        };
      } else if (folderPath.includes("/subjects/math/theories/groups")) {
        return {
          theorems: [
            {
              id: "group-theorem1",
              name: "Lagrange's Theorem",
              description:
                "The order of any subgroup H of a finite group G divides the order of G.",
              proofState: { status: "Complete" },
            },
          ],
          definitions: [
            {
              id: "group-def1",
              name: "Normal Subgroup",
              content:
                "A subgroup N of a group G is called normal if it is invariant under conjugation by members of the group G.",
            },
            {
              id: "group-def2",
              name: "Quotient Group",
              content:
                "For a normal subgroup N of a group G, the quotient group G/N is the set of cosets of N in G with the operation induced by the operation in G.",
            },
          ],
        };
      }

      return { theorems: [], definitions: [] };
    } catch (error) {
      console.error("Failed to execute Rust code:", error);
      throw new Error(
        "Failed to execute Rust code. Please check console for details."
      );
    }
  };

  if (loading && !selectedFolder)
    return (
      <div className={styles.loadingIndicator}>
        Loading mathematics content...
      </div>
    );
  if (error) return <div className={styles.errorMessage}>{error}</div>;

  return (
    <div className={styles.mathPageContainer}>
      <div className={styles.contentWrapper}>
        {/* Sidebar Component */}
        <Sidebar
          folderTree={folderTree}
          selectedFolder={selectedFolder}
          onFolderSelect={setSelectedFolder}
          rustData={rustData}
        />

        {/* Use the new MathContent component */}
        <MathContentComponent
          content={formattedContent}
          loading={loading}
          error={error}
          onNavigate={navigate}
        />
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
