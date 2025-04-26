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
      // Add the new turn_render patterns
      "/subjects/math/theories_turn_render/**/*.json",
      "../../../subjects/math/theories_turn_render/**/*.json",
      "./subjects/math/theories_turn_render/**/*.json",
    ];

    let foundFiles = false;

    // Try each pattern in succession until one works
    for (const pattern of pathPatterns) {
      if (foundFiles) continue;

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
        // Add the new patterns for turn_render
        else if (pattern === "/subjects/math/theories_turn_render/**/*.json") {
          result = await import.meta.glob(
            "/subjects/math/theories_turn_render/**/*.json",
            {
              eager: true,
            }
          );
        } else if (
          pattern === "../../../subjects/math/theories_turn_render/**/*.json"
        ) {
          result = await import.meta.glob(
            "../../../subjects/math/theories_turn_render/**/*.json",
            { eager: true }
          );
        } else if (
          pattern === "./subjects/math/theories_turn_render/**/*.json"
        ) {
          result = await import.meta.glob(
            "./subjects/math/theories_turn_render/**/*.json",
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

  // Define multiple paths to try, with more variations
  const pathsToTry = [
    // Standard paths
    `../subjects/math/theories/${theoryName}/definitions.json`,
    `/subjects/math/theories/${theoryName}/definitions.json`,
    `./subjects/math/theories/${theoryName}/definitions.json`,
    `/math/theories/${theoryName}/definitions.json`,

    // Path with full 'theories/' structure preserved
    ...(theoryName.includes("/")
      ? [
          `../subjects/math/${theoryName}/definitions.json`,
          `/subjects/math/${theoryName}/definitions.json`,
          `/math/${theoryName}/definitions.json`,
        ]
      : []),

    // Try definition.json singular form
    `../subjects/math/theories/${theoryName}/definition.json`,
    `/subjects/math/theories/${theoryName}/definition.json`,

    // Additional variations with different base paths
    `../../../subjects/math/theories/${theoryName}/definitions.json`,
    `../../subjects/math/theories/${theoryName}/definitions.json`,
    `/public/subjects/math/theories/${theoryName}/definitions.json`,

    // Try with 'content' subdirectory
    `/subjects/math/theories/${theoryName}/content/definitions.json`,

    // Try public static files
    `/public/theories/${theoryName}/definitions.json`,
    `/static/theories/${theoryName}/definitions.json`,
    `/assets/theories/${theoryName}/definitions.json`,
  ];

  for (const path of pathsToTry) {
    try {
      console.log(`Trying to fetch from: ${path}`);
      const response = await fetch(path);

      if (!response.ok) {
        console.log(`Failed to load from ${path}: ${response.status}`);
        continue;
      }

      // Check if the response is valid JSON
      const contentType = response.headers.get("content-type");
      if (!contentType || !contentType.includes("application/json")) {
        console.log(`Skipping non-JSON response from: ${path}`);
        continue;
      }

      const data = await response.json();
      console.log(`Successfully loaded definitions from ${path}:`, data);

      // Process the data based on its structure
      if (Array.isArray(data)) {
        // If it's an array, process each definition
        return data.map((def) => ({
          name: def.name || "Unknown",
          docs:
            def.docs || def.documentation || `Definition from ${theoryName}`,
          kind: def.kind || "struct",
          members: def.members || [],
        }));
      } else {
        // If it's a single object, wrap it in an array
        return [
          {
            name: data.name || "Unknown",
            docs:
              data.docs ||
              data.documentation ||
              `Definition from ${theoryName}`,
            kind: data.kind || "struct",
            members: data.members || [],
          },
        ];
      }
    } catch (error) {
      console.log(`Error loading from ${path}:`, error);
      // Continue to the next path if there's an error
      continue;
    }
  }

  // If we get here, we couldn't load from any path
  console.log(`Unable to load definitions for ${theoryName} from any path`);

  // Create placeholder content to prevent "No theory selected" message
  if (theoryName) {
    console.log(`Creating placeholder content for ${theoryName}`);
    return [
      {
        name: `${theoryName} (Placeholder)`,
        docs: `This is a placeholder for the ${theoryName} theory. The actual content might not be available yet.`,
        kind: "struct",
        members: [
          {
            name: "placeholder",
            type: "String",
            docs: "This is a placeholder member to ensure the definition can be rendered correctly.",
          },
        ],
      },
    ];
  }

  return [];
}

// Add a new function to load theorems directly
async function loadTheoremsDirectly(theoryName: string): Promise<Theorem[]> {
  console.log(`Attempting to load theorems directly for ${theoryName}`);

  // Define multiple paths to try, with more variations
  const pathsToTry = [
    // Standard paths
    `../subjects/math/theories/${theoryName}/theorems.json`,
    `/subjects/math/theories/${theoryName}/theorems.json`,
    `./subjects/math/theories/${theoryName}/theorems.json`,
    `/math/theories/${theoryName}/theorems.json`,

    // Path with full 'theories/' structure preserved
    ...(theoryName.includes("/")
      ? [
          `../subjects/math/${theoryName}/theorems.json`,
          `/subjects/math/${theoryName}/theorems.json`,
          `/math/${theoryName}/theorems.json`,
        ]
      : []),

    // Try theorem.json singular form
    `../subjects/math/theories/${theoryName}/theorem.json`,
    `/subjects/math/theories/${theoryName}/theorem.json`,

    // Additional variations with different base paths
    `../../../subjects/math/theories/${theoryName}/theorems.json`,
    `../../subjects/math/theories/${theoryName}/theorems.json`,
    `/public/subjects/math/theories/${theoryName}/theorems.json`,

    // Try with 'content' subdirectory
    `/subjects/math/theories/${theoryName}/content/theorems.json`,

    // Try public static files
    `/public/theories/${theoryName}/theorems.json`,
    `/static/theories/${theoryName}/theorems.json`,
    `/assets/theories/${theoryName}/theorems.json`,
  ];

  for (const path of pathsToTry) {
    try {
      console.log(`Trying to fetch from: ${path}`);
      const response = await fetch(path);

      if (!response.ok) {
        console.log(`Failed to load from ${path}: ${response.status}`);
        continue;
      }

      // Check if the response is valid JSON
      const contentType = response.headers.get("content-type");
      if (!contentType || !contentType.includes("application/json")) {
        console.log(`Skipping non-JSON response from: ${path}`);
        continue;
      }

      const data = await response.json();
      console.log(`Successfully loaded theorems from ${path}:`, data);

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
      console.log(`Error loading from ${path}:`, error);
      // Continue to the next path if there's an error
      continue;
    }
  }

  // If we get here, we couldn't load from any path
  console.log(`Unable to load theorems for ${theoryName} from any path`);
  return [];
}

/**
 * Selects a single JSON file from theoryCache by path
 * @param jsonPath Path to the JSON file
 * @returns Theory content (definitions and theorems)
 */
export function selectJsonFromAll(jsonPath: string): MathContent | null {
  // Extract the theory name for display
  const theoryName = jsonPath.includes("/") 
    ? jsonPath.split("/").pop() || jsonPath 
    : jsonPath;
  
  // Find exact match first, then fallback to partial match
  let matchingPath = Object.keys(theoryCache).find(path => 
    path.endsWith(jsonPath)
  );
  
  // If no exact match, try partial match
  if (!matchingPath) {
    matchingPath = Object.keys(theoryCache).find(path => 
      path.includes(jsonPath)
    );
  }
  
  // If no match found, return empty content
  if (!matchingPath) {
    return {
      definitions: [],
      theorems: [],
      folder: jsonPath,
      theory: theoryName
    };
  }
  
  // Get content from the matching file
  const content = theoryCache[matchingPath];
  
  // Process based on file type
  const definitions: Definition[] = [];
  const theorems: Theorem[] = [];
  
  if (matchingPath.includes("definition")) {
    if (Array.isArray(content)) {
      content.forEach((def: any) => {
        const parsed = parseDefinition(def, matchingPath as string);
        if (parsed) definitions.push(parsed);
      });
    } else {
      const parsed = parseDefinition(content, matchingPath);
      if (parsed) definitions.push(parsed);
    }
  } else if (matchingPath.includes("theorem")) {
    // Get a proper theory name from the path
    const pathParts = matchingPath.split('/');
    const theoryFolder = pathParts[pathParts.length - 2] || ''; // Get the parent folder name
    
    if (Array.isArray(content)) {
      content.forEach((thm: any) => {
        const parsed = parseTheorem(thm, matchingPath as string);
        if (parsed) {
          // Set proper theory name if the theorem is from a theory folder
          if (theoryFolder && parsed.name === "theorems") {
            parsed.name = `${theoryFolder} Theorem`;
          }
          theorems.push(parsed);
        }
      });
    } else {
      const parsed = parseTheorem(content, matchingPath);
      if (parsed) theorems.push(parsed);
    }
  }
  
  // Extract the theory folder name from the path for better display
  const folderPath = jsonPath.includes("/") ? jsonPath.substring(0, jsonPath.lastIndexOf("/")) : "";
  const theoryDisplayName = folderPath ? folderPath.split("/").pop() || theoryName : theoryName;
  
  return {
    definitions,
    theorems,
    folder: jsonPath,
    theory: theoryDisplayName
  };
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

    // Handle the nested structure in theorems.json
    if (fileContent.id && fileContent.content && fileContent.content.Theorem) {
      const theoremData = fileContent.content.Theorem;
      
      // Extract statement from the nested structure
      let statement = "";
      if (theoremData.initial_proof_state?.content?.Theorem?.initial_proof_state?.content?.ProofState?.statement?.content?.Text) {
        statement = theoremData.initial_proof_state.content.Theorem.initial_proof_state.content.ProofState.statement.content.Text;
      }
      
      // Create a theorem object from the content, preserving the original content property
      return {
        id: fileContent.id,
        name: theoremData.name || fallbackName,
        statement: statement,
        description: theoremData.description || `Theorem from ${fileName}`,
        proof_steps: theoremData.proof_steps || [],
        tags: theoremData.tags || [],
        // IMPORTANT: Preserve the original content property to keep the proof steps
        content: fileContent.content
      };
    }

    // Create a theorem object from the content (original format)
    return {
      name: fileContent.name || fallbackName,
      statement: fileContent.statement || "",
      description: fileContent.description || `Theorem from ${fileName}`,
      proof_steps: fileContent.proof_steps || [],
      tags: fileContent.tags || [],
      // If the fileContent itself is the content, preserve it
      content: fileContent.content || undefined
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

/**
 * Utility function to try loading theory content with alternative paths
 * This helps when the normal fetching fails or returns placeholder content
 * @param originalPath The original path that failed to load
 * @param depth Used internally to prevent recursion
 */
export async function tryLoadWithAlternativePaths(
  originalPath: string,
  depth: number = 0
): Promise<MathContent | null> {
  // Prevent infinite recursion
  if (depth > 1) {
    console.log(`Maximum recursion depth reached for ${originalPath}`);
    return null;
  }

  console.log(`Trying alternative paths for ${originalPath}`);

  // Collection of alternative paths to try
  const alternativePaths = [];

  // If the path doesn't have a theories/ prefix, add it
  if (!originalPath.startsWith("theories/")) {
    alternativePaths.push(`theories/${originalPath}`);
  }

  // If the path has a theories/ prefix, try without it
  if (originalPath.startsWith("theories/")) {
    alternativePaths.push(originalPath.slice("theories/".length));
  }

  // Also try adding or removing various other path structures
  if (!originalPath.includes("/")) {
    alternativePaths.push(`mathematics/${originalPath}`);
    alternativePaths.push(`math_theories/${originalPath}`);
  }

  // For each alternative path, try fetching content
  for (const path of alternativePaths) {
    // Skip if we're trying the original path again
    if (path === originalPath) continue;

    console.log(`Trying alternative path: ${path}`);
    try {
      const content = await selectJsonFromAll(path);

      // Check if we got useful content (not just placeholders)
      if (content && content.definitions && content.definitions.length > 0) {
        // Check if first definition is not just a placeholder
        const firstDef = content.definitions[0];
        if (
          !firstDef.name.includes("(Not Found)") &&
          !firstDef.name.includes("Placeholder")
        ) {
          console.log(`Found valid content with alternative path: ${path}`);
          return content;
        }
      }
    } catch (err) {
      console.log(`Failed to load with alternative path ${path}:`, err);
    }
  }

  console.log("No valid content found with alternative paths");
  return null;
}
