import React, { useRef, useEffect } from "react";
import styles from "./Sidebar.module.scss";
import TreeView from "../treeview/treeview";
import { TheoryFolder } from "../../models/math";
import { FolderNode } from "../../../../services/mathService";

interface SidebarProps {
  theories: TheoryFolder[];
  folderTree: FolderNode[];
  selectedTheory: string | null;
  onTheorySelect: (theoryPath: string) => void;
  loading: boolean;
}

const Sidebar: React.FC<SidebarProps> = ({
  folderTree,
  selectedTheory,
  onTheorySelect,
  loading,
}) => {
  const sidebarContentRef = useRef<HTMLDivElement>(null);

  // Initialize scrolling
  useEffect(() => {
    if (sidebarContentRef.current) {
      // Force scrolling to work on mobile devices
      const touchStartHandler = () => {};
      sidebarContentRef.current.addEventListener(
        "touchstart",
        touchStartHandler,
        { passive: true }
      );

      return () => {
        if (sidebarContentRef.current) {
          sidebarContentRef.current.removeEventListener(
            "touchstart",
            touchStartHandler
          );
        }
      };
    }
  }, []);

  // For debugging
  useEffect(() => {
    if (selectedTheory) {
      console.log("Sidebar received selectedTheory update:", selectedTheory);
      console.log("Sidebar has folderTree with nodes:", folderTree.length);
    }
  }, [selectedTheory, folderTree]);

  // Format theory name for display
  const formatTheoryName = (name: string): string => {
    return name
      .split("_")
      .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
      .join(" ");
  };

  // Handle selection from the tree view
  const handleTreeSelection = (path: string, isFile: boolean) => {
    console.log("Tree selection made:", path, "isFile:", isFile);
    // Update the URL by notifying parent component
    onTheorySelect(path);
  };

  // Display the current folder name
  const displayFolderName = () => {
    return selectedTheory ? formatTheoryName(selectedTheory) : "None selected";
  };

  return (
    <div className={styles.sidebar}>
      {/* Fixed header */}
      <div className={styles.sidebarHeader}>
        <h2>Mathematics Explorer</h2>
        <div className={styles.sidebarPath}>
          <span>Current folder:</span>
          <code>{displayFolderName()}</code>
        </div>
      </div>

      {/* Scrollable content */}
      <div
        className={styles.sidebarContent}
        ref={sidebarContentRef}
        onTouchStart={() => {}} // Empty handler to ensure iOS registers touch scrolling
      >
        {loading ? (
          <div className={styles.loading}>Loading theories...</div>
        ) : (
          <div className={styles.folderTree}>
            {folderTree.length > 0 && (
              <TreeView
                nodes={folderTree}
                selectedPath={selectedTheory || ""}
                onSelect={handleTreeSelection}
              />
            )}
          </div>
        )}

        {/* Mathematical content will be shown here in the future when API is ready */}
        <div className={styles.rustDataSection}>
          <h3>Mathematical Content</h3>
          <p className={styles.emptyState}>
            Select a theory file to view its content.
          </p>
        </div>
      </div>
    </div>
  );
};

export default Sidebar;
