import {
  Definition,
  MathContent,
  Theorem,
  TheoryFolder,
} from "../pages/MathPage/models/math";

// Define FolderNode interface locally to avoid import issues
export interface FolderNode {
  name: string;
  path: string;
  isDirectory: boolean;
  children?: FolderNode[];
}

// Map to store imported theories data
let theoryCache: Record<string, any> = {};

/**
 * Refreshes the theory cache by re-importing all JSON files
 * This is called on page refresh to ensure we have the latest data
 */
export async function refreshTheoryCache(): Promise<void> {
  console.log("Refreshing theory cache with latest data");
  // Clear existing cache
  theoryCache = {};

  // Re-build the cache with latest data
  try {
    console.log("Loading all theory files and folders...");

    // Try these patterns in order until one works
    const pathPatterns = [
      "/subjects/math/theories/**/*.json",
      "../../../subjects/math/theories/**/*.json",
      "./subjects/math/theories/**/*.json",
    ];

    let foundFiles = false;

    // Try each pattern in succession until one works
    for (const pattern of pathPatterns) {
      if (foundFiles) break;

      try {
        console.log(`Trying glob pattern: ${pattern}`);
        let result: Record<string, any> = {};

        // Use separate import.meta.glob calls for each pattern
        if (pattern === "/subjects/math/theories/**/*.json") {
          result = await import.meta.glob("/subjects/math/theories/**/*.json", {
            eager: true,
          });
        } else if (pattern === "../../../subjects/math/theories/**/*.json") {
          result = await import.meta.glob(
            "../../../subjects/math/theories/**/*.json",
            { eager: true }
          );
        } else if (pattern === "./subjects/math/theories/**/*.json") {
          result = await import.meta.glob(
            "./subjects/math/theories/**/*.json",
            { eager: true }
          );
        }

        const paths = Object.keys(result);

        if (paths.length > 0) {
          console.log(
            `✅ Found ${paths.length} files with pattern: ${pattern}`
          );
          console.log("Sample paths:", paths.slice(0, 3));

          // Process each file in the result to ensure it's loaded properly
          for (const path of paths) {
            try {
              const content = result[path];
              console.log(
                `Loaded file: ${path}, Content type: ${typeof content}`
              );

              // If this is a module with a default export, use that
              if (
                content &&
                typeof content === "object" &&
                "default" in content
              ) {
                theoryCache[path] = content.default;
                console.log(`Using default export for ${path}`);
              } else {
                theoryCache[path] = content;
              }

              // Log a sample of the content to verify it's loaded correctly
              if (Array.isArray(theoryCache[path])) {
                console.log(
                  `File ${path} contains an array of ${theoryCache[path].length} items`
                );
                if (theoryCache[path].length > 0) {
                  console.log(
                    `First item sample:`,
                    JSON.stringify(theoryCache[path][0]).substring(0, 200) +
                      "..."
                  );
                }
              } else if (
                theoryCache[path] &&
                typeof theoryCache[path] === "object"
              ) {
                console.log(`File ${path} contains an object`);
                console.log(
                  `Sample properties:`,
                  Object.keys(theoryCache[path]).slice(0, 3)
                );
              }
            } catch (fileErr) {
              console.error(`Error processing file ${path}:`, fileErr);
            }
          }

          foundFiles = true;
        } else {
          console.log(`⚠️ No files found with pattern: ${pattern}`);
        }
      } catch (err: any) {
        console.error(`❌ Error with pattern ${pattern}:`, err.message);
      }
    }

    console.log(
      `Refreshed theory cache. Found ${Object.keys(theoryCache).length} files`
    );

    // Log the full list of loaded files
    if (Object.keys(theoryCache).length > 0) {
      console.log("All loaded files:", Object.keys(theoryCache));
    }
  } catch (error) {
    console.error("Error refreshing theory cache:", error);
  }
}

/**
 * Parses the filesystem structure to create a flat theory folder list
 * for backward compatibility with the existing code
 */
function parseFileStructure(): TheoryFolder[] {
  console.log("Parsing file structure for theories list");
  const theories: TheoryFolder[] = [];
  const folderMap: Record<string, TheoryFolder> = {};

  // Get all unique top-level theory folders from JSON files only
  Object.keys(theoryCache).forEach((filePath) => {
    // Skip non-JSON files
    if (!filePath.endsWith(".json")) {
      return;
    }

    // Extract path parts, looking for the directory after "theories"
    const pathParts = filePath.split("/");
    const theoriesIndex = pathParts.findIndex((part) => part === "theories");

    if (theoriesIndex >= 0 && theoriesIndex + 1 < pathParts.length) {
      const theoryPath = pathParts[theoriesIndex + 1];

      // Skip if we've already processed this theory folder
      if (!folderMap[theoryPath]) {
        // Check if we have definition or theorem files anywhere in this folder's path
        // Only check JSON files
        const hasDefinitions = Object.keys(theoryCache).some(
          (p) =>
            p.includes(`/theories/${theoryPath}/`) &&
            p.includes("definition") &&
            p.endsWith(".json")
        );

        const hasTheorems = Object.keys(theoryCache).some(
          (p) =>
            p.includes(`/theories/${theoryPath}/`) &&
            p.includes("theorem") &&
            p.endsWith(".json")
        );

        // Create the theory folder entry
        const theoryFolder: TheoryFolder = {
          name: formatTheoryName(theoryPath),
          path: theoryPath,
          hasDefinitions,
          hasTheorems,
        };

        folderMap[theoryPath] = theoryFolder;
        theories.push(theoryFolder);
      }
    }
  });

  return theories;
}

/**
 * Formats a theory path into a display name
 */
function formatTheoryName(path: string): string {
  return path
    .split("_")
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join(" ");
}

/**
 * Fetches available theories from the filesystem
 * @returns Promise with list of available theory folders
 */
export async function fetchAvailableTheories(): Promise<TheoryFolder[]> {
  console.log("Fetching available theories from filesystem");

  // Refresh the cache first to ensure we have the latest data
  await refreshTheoryCache();

  try {
    // Parse the file structure to get the available theories
    const theories = parseFileStructure();
    console.log("Available theories:", theories);
    return theories;
  } catch (error) {
    console.error("Error fetching theories:", error);
    return [];
  }
}

// Add a new function to load definitions directly
async function loadDefinitionsDirectly(
  theoryName: string
): Promise<Definition[]> {
  console.log(`Attempting to load definitions directly for ${theoryName}`);

  try {
    const response = await fetch(
      `../subjects/math/theories/${theoryName}/definitions.json`
    );
    if (!response.ok) {
      throw new Error(`HTTP error ${response.status}`);
    }

    const data = await response.json();
    console.log(`Successfully loaded definitions for ${theoryName}:`, data);

    // Process the data based on its structure
    if (Array.isArray(data)) {
      // If it's an array, process each definition
      return data.map((def) => ({
        name: def.name || "Unknown",
        docs: def.docs || def.documentation || `Definition from ${theoryName}`,
        kind: def.kind || "struct",
        members: def.members || [],
      }));
    } else {
      // If it's a single object, wrap it in an array
      return [
        {
          name: data.name || "Unknown",
          docs:
            data.docs || data.documentation || `Definition from ${theoryName}`,
          kind: data.kind || "struct",
          members: data.members || [],
        },
      ];
    }
  } catch (error) {
    console.error(
      `Error loading definitions directly for ${theoryName}:`,
      error
    );
    return [];
  }
}

// Add a new function to load theorems directly
async function loadTheoremsDirectly(theoryName: string): Promise<Theorem[]> {
  console.log(`Attempting to load theorems directly for ${theoryName}`);

  try {
    const response = await fetch(
      `../subjects/math/theories/${theoryName}/theorems.json`
    );
    if (!response.ok) {
      throw new Error(`HTTP error ${response.status}`);
    }

    const data = await response.json();
    console.log(`Successfully loaded theorems for ${theoryName}:`, data);

    // Process the data based on its structure
    if (Array.isArray(data)) {
      // If it's an array, process each theorem
      return data.map((thm) => ({
        name: thm.name || "Unknown",
        statement: thm.statement || "",
        description: thm.description || `Theorem from ${theoryName}`,
        proof_steps: thm.proof_steps || [],
        tags: thm.tags || [],
      }));
    } else {
      // If it's a single object, wrap it in an array
      return [
        {
          name: data.name || "Unknown",
          statement: data.statement || "",
          description: data.description || `Theorem from ${theoryName}`,
          proof_steps: data.proof_steps || [],
          tags: data.tags || [],
        },
      ];
    }
  } catch (error) {
    console.error(`Error loading theorems directly for ${theoryName}:`, error);
    return [];
  }
}

/**
 * Fetches content for a specific theory
 * @param theoryPath Path to the theory folder
 * @returns Promise with theory content (definitions and theorems)
 */
export async function fetchTheoryContent(
  theoryPath: string
): Promise<MathContent | null> {
  console.log(`Fetching content for theory: ${theoryPath} from filesystem`);

  try {
    // Handle both old path format (just the theory name) and new format (theories/name/subpath)
    let searchPath = theoryPath;

    // If the path uses the new format with 'theories/' prefix, use it directly
    if (theoryPath.startsWith("theories/")) {
      searchPath = theoryPath.slice("theories/".length);
    }

    // Extract the theory name from the path for display purposes
    const theoryName = theoryPath.includes("/")
      ? theoryPath.split("/").pop() || theoryPath
      : theoryPath;

    // First, try the regular method using the theory cache
    // Get all JSON files for this theory - supporting both paths with and without subfolders
    const theoryFiles = Object.keys(theoryCache).filter(
      (path) =>
        (path.includes(`/theories/${searchPath}/`) ||
          // Handle subfolders if using the complete path
          (theoryPath.includes("/") && path.includes(theoryPath))) &&
        // Only include JSON files
        path.endsWith(".json")
    );

    if (theoryFiles.length > 0) {
      console.log(
        `Found ${theoryFiles.length} JSON files for theory ${theoryPath}:`,
        theoryFiles
      );

      // Collect all definitions and theorems from the files
      const definitions: Definition[] = [];
      const theorems: Theorem[] = [];

      theoryFiles.forEach((filePath) => {
        console.log(`Processing file: ${filePath}`);
        const fileContent = theoryCache[filePath];
        console.log(
          `File content type: ${typeof fileContent}`,
          `Is Array: ${Array.isArray(fileContent)}`,
          `Length: ${Array.isArray(fileContent) ? fileContent.length : "N/A"}`
        );

        if (fileContent === undefined || fileContent === null) {
          console.error(`Empty or undefined content for file: ${filePath}`);
          return;
        }

        // Handle definition files
        if (filePath.includes("definition")) {
          try {
            // Check if the file content is an array
            if (Array.isArray(fileContent)) {
              // Process each definition in the array
              console.log(
                `Processing array of ${fileContent.length} definitions`
              );
              fileContent.forEach((def, index) => {
                console.log(
                  `Processing definition ${index + 1}/${fileContent.length}: ${
                    def.name || "unnamed"
                  }`
                );
                const parsedDef = parseDefinition(def, filePath);
                if (parsedDef) definitions.push(parsedDef);
              });
            } else {
              // Process as a single definition
              console.log(`Processing single definition`);
              const def = parseDefinition(fileContent, filePath);
              if (def) definitions.push(def);
            }
          } catch (e) {
            console.error(`Error parsing definition in ${filePath}:`, e);
          }
        }

        // Handle theorem files
        if (filePath.includes("theorem")) {
          try {
            // Check if the file content is an array
            if (Array.isArray(fileContent)) {
              // Process each theorem in the array
              console.log(`Processing array of ${fileContent.length} theorems`);
              fileContent.forEach((thm, index) => {
                console.log(
                  `Processing theorem ${index + 1}/${fileContent.length}: ${
                    thm.name || "unnamed"
                  }`
                );
                const parsedThm = parseTheorem(thm, filePath);
                if (parsedThm) theorems.push(parsedThm);
              });
            } else {
              // Process as a single theorem
              console.log(`Processing single theorem`);
              const thm = parseTheorem(fileContent, filePath);
              if (thm) theorems.push(thm);
            }
          } catch (e) {
            console.error(`Error parsing theorem in ${filePath}:`, e);
          }
        }
      });

      // Create the content object
      const content: MathContent = {
        definitions,
        theorems,
        folder: theoryPath,
        theory: theoryName,
      };

      console.log(
        `Assembled content for ${theoryPath}:`,
        `${definitions.length} definitions, ${theorems.length} theorems`
      );

      return content;
    } else {
      console.warn(
        `No JSON files found in cache for theory: ${theoryPath}, trying direct loading...`
      );

      // Try to load the files directly using fetch as a fallback
      const definitions = await loadDefinitionsDirectly(searchPath);
      const theorems = await loadTheoremsDirectly(searchPath);

      console.log(
        `Direct loading results - Definitions: ${definitions.length}, Theorems: ${theorems.length}`
      );

      if (definitions.length > 0 || theorems.length > 0) {
        const content: MathContent = {
          definitions,
          theorems,
          folder: theoryPath,
          theory: theoryName,
        };

        console.log(
          `Assembled content for ${theoryPath} via direct loading:`,
          `${definitions.length} definitions, ${theorems.length} theorems`
        );

        return content;
      }

      return null;
    }
  } catch (error) {
    console.error(`Error fetching theory content for ${theoryPath}:`, error);
    return null;
  }
}

/**
 * Parse a definition from a JSON file
 */
function parseDefinition(
  fileContent: any,
  filePath: string
): Definition | null {
  try {
    // Extract a name fallback from the file path if needed
    const fileName = filePath.split("/").pop() || "";
    const fallbackName = fileName
      .replace(".json", "")
      .replace("definition_", "");

    // Create a definition object from the content
    return {
      name: fileContent.name || fallbackName,
      docs:
        fileContent.documentation ||
        fileContent.docs ||
        `Definition from ${fileName}`,
      kind: fileContent.kind || "struct",
      members: fileContent.members || [],
    };
  } catch (e) {
    console.error(`Error parsing definition:`, e);
    return null;
  }
}

/**
 * Parse a theorem from a JSON file
 */
function parseTheorem(fileContent: any, filePath: string): Theorem | null {
  try {
    // Extract a name fallback from the file path if needed
    const fileName = filePath.split("/").pop() || "";
    const fallbackName = fileName.replace(".json", "").replace("theorem_", "");

    // Create a theorem object from the content
    return {
      name: fileContent.name || fallbackName,
      statement: fileContent.statement || "",
      description: fileContent.description || `Theorem from ${fileName}`,
      proof_steps: fileContent.proof_steps || [],
      tags: fileContent.tags || [],
    };
  } catch (e) {
    console.error(`Error parsing theorem:`, e);
    return null;
  }
}

/**
 * Builds a complete recursive folder tree structure from the theory files
 * @returns Array of root FolderNode objects
 */
export function buildCompleteFileTree(): FolderNode[] {
  console.log("Building complete file tree");

  // If no files in cache, return empty root
  if (Object.keys(theoryCache).length === 0) {
    console.warn("No files in cache, returning empty tree");
    return [
      {
        name: "Theories",
        path: "theories",
        isDirectory: true,
        children: [],
      },
    ];
  }

  // Root tree structure
  const rootNodes: FolderNode[] = [];
  const nodeMap: Record<string, FolderNode> = {};

  // Create virtual root node
  const theoriesRoot: FolderNode = {
    name: "Theories",
    path: "theories",
    isDirectory: true,
    children: [],
  };

  rootNodes.push(theoriesRoot);
  nodeMap["theories"] = theoriesRoot;

  // Process all JSON files from the cache to build folder structure
  Object.keys(theoryCache).forEach((filePath) => {
    // Skip if the path doesn't include theories or isn't a JSON file
    if (!filePath.includes("/theories/") || !filePath.endsWith(".json")) {
      return;
    }

    // Extract the path after "theories/"
    const match = filePath.match(/.*\/theories\/(.+)/);
    if (!match || !match[1]) return;

    const relativePath = match[1];
    const pathParts = relativePath.split("/");

    // Process each path segment to build the tree
    let currentPath = "theories";

    // Create path for each segment
    for (let i = 0; i < pathParts.length; i++) {
      const segment = pathParts[i];
      const isLastSegment = i === pathParts.length - 1;
      const isDirectory = !isLastSegment || !segment.includes("."); // If it has no extension, it's a directory

      // Build the path string as we go
      const prevPath = currentPath;
      currentPath =
        i === 0 ? `theories/${segment}` : `${currentPath}/${segment}`;

      // Skip if we already processed this node
      if (!nodeMap[currentPath]) {
        // Create a new node
        const newNode: FolderNode = {
          name: segment,
          path: currentPath,
          isDirectory: isDirectory,
          children: isDirectory ? [] : undefined,
        };

        // Add to parent's children
        const parent = nodeMap[prevPath];
        if (parent && parent.children) {
          parent.children.push(newNode);
        }

        // Add to node map
        nodeMap[currentPath] = newNode;
      }
    }
  });

  // Sort children alphabetically in each folder
  const sortNodeChildren = (node: FolderNode) => {
    if (node.children && node.children.length > 0) {
      // Sort directories first, then alphabetically
      node.children.sort((a, b) => {
        // Directories come before files
        if (a.isDirectory && !b.isDirectory) return -1;
        if (!a.isDirectory && b.isDirectory) return 1;

        // Alphabetical sort
        return a.name.localeCompare(b.name);
      });

      // Recursively sort children
      node.children.forEach(sortNodeChildren);
    }
  };

  // Sort the tree
  sortNodeChildren(theoriesRoot);

  console.log("Built tree with nodes:", Object.keys(nodeMap).length);
  return rootNodes;
}

// Debug utility function to check if files exist at a specific path
export async function debugFilePaths(): Promise<void> {
  console.log("===== DEBUG: Checking file paths =====");

  try {
    // Check various paths to see which ones work - all must start with / or ./
    const paths = [
      "/",
      "/subjects",
      "/subjects/math",
      "../subjects",
      "../../subjects",
      "../../../subjects",
      "./subjects",
      "./",
    ];

    for (const basePath of paths) {
      try {
        console.log(`Trying path: ${basePath}`);
        let files = {};

        // Only search for JSON files to avoid issues with Rust files
        if (basePath === "/") {
          files = await import.meta.glob("/**/*.json", {
            eager: false,
            as: "url",
          });
        } else if (basePath === "/subjects") {
          files = await import.meta.glob("/subjects/**/*.json", {
            eager: false,
            as: "url",
          });
        } else if (basePath === "/subjects/math") {
          files = await import.meta.glob("/subjects/math/**/*.json", {
            eager: false,
            as: "url",
          });
        } else if (basePath === "../subjects") {
          files = await import.meta.glob("../subjects/**/*.json", {
            eager: false,
            as: "url",
          });
        } else if (basePath === "../../subjects") {
          files = await import.meta.glob("../../subjects/**/*.json", {
            eager: false,
            as: "url",
          });
        } else if (basePath === "../../../subjects") {
          files = await import.meta.glob("../../../subjects/**/*.json", {
            eager: false,
            as: "url",
          });
        } else if (basePath === "./subjects") {
          files = await import.meta.glob("./subjects/**/*.json", {
            eager: false,
            as: "url",
          });
        } else if (basePath === "./") {
          files = await import.meta.glob("./**/*.json", {
            eager: false,
            as: "url",
          });
        }

        const fileCount = Object.keys(files).length;
        console.log(
          `  - Found ${fileCount} JSON files with pattern: ${basePath}/**/*.json`
        );

        if (fileCount > 0) {
          console.log(`  - First few files:`, Object.keys(files).slice(0, 3));
        }

        // If found files at the root, try to find the theories folder
        if (fileCount > 0 && basePath) {
          // Try different theory paths with static patterns - only looking for JSON files
          let theories = {};
          let theoryCount = 0;

          if (basePath === "/") {
            theories = await import.meta.glob(
              "/subjects/math/theories/**/*.json",
              { eager: false, as: "url" }
            );
          } else if (basePath === "../subjects") {
            theories = await import.meta.glob(
              "../subjects/math/theories/**/*.json",
              { eager: false, as: "url" }
            );
          } else if (basePath === "../../subjects") {
            theories = await import.meta.glob(
              "../../subjects/math/theories/**/*.json",
              { eager: false, as: "url" }
            );
          } else if (basePath === "../../../subjects") {
            theories = await import.meta.glob(
              "../../../subjects/math/theories/**/*.json",
              { eager: false, as: "url" }
            );
          } else if (basePath === "./subjects") {
            theories = await import.meta.glob(
              "./subjects/math/theories/**/*.json",
              { eager: false, as: "url" }
            );
          } else if (basePath === "./") {
            theories = await import.meta.glob(
              "./subjects/math/theories/**/*.json",
              { eager: false, as: "url" }
            );
          } else if (basePath === "/subjects") {
            theories = await import.meta.glob(
              "/subjects/math/theories/**/*.json",
              { eager: false, as: "url" }
            );
          } else if (basePath === "/subjects/math") {
            theories = await import.meta.glob(
              "/subjects/math/theories/**/*.json",
              { eager: false, as: "url" }
            );
          }

          theoryCount = Object.keys(theories).length;
          console.log(`  - Found ${theoryCount} JSON files in theories folder`);

          if (theoryCount > 0) {
            console.log(
              `  - Sample theory files:`,
              Object.keys(theories).slice(0, 3)
            );
          }
        }
      } catch (err: any) {
        console.log(`  - Error with path ${basePath}:`, err.message);
      }
    }
  } catch (error) {
    console.error("Error in debug function:", error);
  }

  console.log("===== DEBUG: Path check complete =====");
}
