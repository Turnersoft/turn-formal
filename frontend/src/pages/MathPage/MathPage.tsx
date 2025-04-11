import React, { useEffect, useState } from "react";
import { useNavigate, useLocation } from "react-router-dom";
import styles from "./MathPage.module.scss";
import Sidebar from "./components/sidebar/sidebar";
import MathContentComponent from "./components/math_content/math_content";
import { TheoryFolder, MathContent } from "./models/math";
import {
  fetchAvailableTheories,
  fetchTheoryContent,
  refreshTheoryCache,
  buildCompleteFileTree,
  FolderNode,
  debugFilePaths,
} from "../../services/mathService";

const MathPage: React.FC = () => {
  console.log("MathPage rendering");
  const navigate = useNavigate();
  const location = useLocation();

  // Extract theory path from URL
  // This will work for both /math/theoryId and deeper paths like /math/theories/groups
  const getTheoryPathFromUrl = (): string | undefined => {
    const path = location.pathname;
    if (path === "/math") return undefined;

    // Remove the leading '/math/' to get the theory path
    return path.replace(/^\/math\//, "");
  };

  const theoryPath = getTheoryPathFromUrl();
  console.log("Extracted theory path from URL:", theoryPath);

  const [theories, setTheories] = useState<TheoryFolder[]>([]);
  const [folderTree, setFolderTree] = useState<FolderNode[]>([]);
  const [mathContent, setMathContent] = useState<MathContent | null>(null);
  const [loading, setLoading] = useState<boolean>(true);
  const [contentLoading, setContentLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);

  // Load available theories on component mount and whenever the location changes (refresh)
  useEffect(() => {
    console.log("Loading theories effect triggered - location change or mount");

    const loadTheories = async () => {
      setLoading(true);
      try {
        // Debug file paths to help diagnose issues
        console.log("Running path diagnostics...");
        await debugFilePaths();

        // Refresh the theory cache to ensure we have the latest data
        await refreshTheoryCache();

        // Fetch available theories
        console.log("Fetching theories from filesystem");
        const theoryData = await fetchAvailableTheories();
        console.log("Theory data received:", theoryData);
        setTheories(theoryData);

        // Build complete folder tree for sidebar
        const treeData = buildCompleteFileTree();
        console.log("Complete folder tree:", treeData);
        setFolderTree(treeData);

        // If no theory is selected in the URL but theories are available, redirect to the first one
        if (!theoryPath && theoryData.length > 0) {
          console.log(
            "No theory selected, redirecting to first theory:",
            theoryData[0].path
          );
          navigate(`/math/${theoryData[0].path}`, { replace: true });
        }

        setError(null);
      } catch (err) {
        console.error("Error loading theories:", err);
        setError("Failed to load available theories");
      } finally {
        setLoading(false);
      }
    };

    loadTheories();
  }, [location.key]); // Run on mount and when the location key changes (page refresh)

  // Load content when the URL theory parameter changes
  useEffect(() => {
    console.log("Selected theory (from URL) changed:", theoryPath);
    if (!theoryPath) {
      setMathContent(null);
      return;
    }

    const loadTheoryContent = async () => {
      setContentLoading(true);
      try {
        // Fetch theory content from filesystem
        console.log("Fetching theory content for:", theoryPath);
        const content = await fetchTheoryContent(theoryPath);

        console.log("Theory content received:", content);
        setMathContent(content);
        setError(null);

        // Update the document title with the theory name
        document.title = content?.theory
          ? `${content.theory} - Turn-Formal`
          : "Mathematics - Turn-Formal";
      } catch (err) {
        console.error(`Error loading content for ${theoryPath}:`, err);
        setError(`Failed to load content for ${theoryPath}`);
      } finally {
        setContentLoading(false);
      }
    };

    loadTheoryContent();
  }, [theoryPath]);

  // Handle theory selection by updating the URL
  const handleTheorySelect = (theoryPath: string) => {
    console.log("Theory selected:", theoryPath);

    // No need for special handling of the same theory selection
    // as the router will handle it - only navigate if it's different
    if (getTheoryPathFromUrl() !== theoryPath) {
      // Update URL with theory ID, but don't replace the history entry to allow back/forward navigation
      navigate(`/math/${theoryPath}`);

      // Update document title immediately for better UX
      const theoryName =
        theories.find((t) => t.path === theoryPath)?.name || theoryPath;
      document.title = `${formatTheoryName(theoryName)} - Turn-Formal`;
    } else {
      // Force a refresh of the content if clicking the same theory
      console.log("Re-selecting same theory, forcing content refresh");

      // Refresh theory cache first
      refreshTheoryCache().then(() => {
        fetchTheoryContent(theoryPath).then((content) => {
          console.log("Refreshed theory content received:", content);
          setMathContent(content);
        });
      });
    }
  };

  // Helper function to format theory names consistently
  const formatTheoryName = (name: string): string => {
    return name
      .split("_")
      .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
      .join(" ");
  };

  console.log("Current state:", {
    theories: theories.length,
    selectedTheory: theoryPath,
    contentLoaded: !!mathContent,
    loading,
    contentLoading,
    error,
  });

  return (
    <div className={styles.mathPage}>
      <Sidebar
        theories={theories}
        folderTree={folderTree}
        selectedTheory={theoryPath || null}
        onTheorySelect={handleTheorySelect}
        loading={loading}
      />
      <div className={styles.mainContent}>
        <MathContentComponent
          content={mathContent}
          loading={contentLoading}
          error={error}
        />

        {!loading && !error && mathContent === null && (
          <div className={styles.emptyState}>
            <h3>No Mathematical Content Found</h3>
            <p>
              No mathematical content could be found in this folder. Please
              select a different folder from the sidebar or make sure your
              content is properly formatted as JSON files.
            </p>
          </div>
        )}
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
