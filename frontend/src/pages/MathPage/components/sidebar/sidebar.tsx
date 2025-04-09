import React, { useRef, useEffect } from "react";
import styles from "./Sidebar.module.scss";
import TreeView from "../Treeview/treeview";

// Interfaces for our data structures
interface FolderNode {
  name: string;
  path: string;
  isDirectory: boolean;
  children?: FolderNode[];
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

interface SidebarProps {
  folderTree: FolderNode[];
  selectedFolder: string;
  onFolderSelect: (folderPath: string) => void;
  rustData: RustMathData | null;
}

const Sidebar: React.FC<SidebarProps> = ({
  folderTree,
  selectedFolder,
  onFolderSelect,
  rustData,
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

  return (
    <div className={styles.sidebar}>
      {/* Fixed header */}
      <div className={styles.sidebarHeader}>
        <h2>Mathematics Explorer</h2>
        <div className={styles.sidebarPath}>
          <span>Current path:</span>
          <code>{selectedFolder}</code>
        </div>
      </div>

      {/* Scrollable content */}
      <div
        className={styles.sidebarContent}
        ref={sidebarContentRef}
        onTouchStart={() => {}} // Empty handler to ensure iOS registers touch scrolling
      >
        <div className={styles.folderTree}>
          <TreeView
            nodes={folderTree}
            selectedPath={selectedFolder}
            onSelect={onFolderSelect}
          />
        </div>

        {rustData && (
          <div className={styles.rustDataSection}>
            <h3>Generated Data</h3>

            {rustData.theorems.length > 0 && (
              <div className={styles.dataSection}>
                <h4>Theorems</h4>
                <ul className={styles.dataList}>
                  {rustData.theorems.map((theorem) => (
                    <li key={theorem.id} className={styles.dataItem}>
                      <div className={styles.dataTitle}>{theorem.name}</div>
                      <div className={styles.dataStatus}>
                        {theorem.proofState?.status && (
                          <span
                            className={`${styles.statusTag} ${
                              theorem.proofState.status === "Complete"
                                ? styles.complete
                                : styles.inProgress
                            }`}
                          >
                            {theorem.proofState.status}
                          </span>
                        )}
                      </div>
                    </li>
                  ))}
                </ul>
              </div>
            )}

            {rustData.definitions.length > 0 && (
              <div className={styles.dataSection}>
                <h4>Definitions</h4>
                <ul className={styles.dataList}>
                  {rustData.definitions.map((def) => (
                    <li key={def.id} className={styles.dataItem}>
                      <div className={styles.dataTitle}>{def.name}</div>
                    </li>
                  ))}
                </ul>
              </div>
            )}
          </div>
        )}
      </div>
    </div>
  );
};

export default Sidebar;
