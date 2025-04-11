import React, { useState, useEffect } from "react";
import styles from "./TreeView.module.scss";
import { FolderNode } from "../../../../services/mathService";

// Props for the TreeView component
interface TreeViewProps {
  nodes: FolderNode[];
  selectedPath: string;
  onSelect: (path: string, isFile: boolean) => void;
}

// Main TreeView component that displays a list of nodes
const TreeView: React.FC<TreeViewProps> = ({
  nodes,
  selectedPath,
  onSelect,
}) => {
  // Log selection info for debugging
  useEffect(() => {
    console.log("TreeView selectedPath:", selectedPath);
    console.log("TreeView nodes:", nodes);
  }, [selectedPath, nodes]);

  return (
    <div className={styles.treeView}>
      {nodes &&
        nodes.map((node) => (
          <TreeNode
            key={node.path}
            node={node}
            selectedPath={selectedPath}
            onSelect={onSelect}
          />
        ))}
    </div>
  );
};

// Props for the TreeNode component
interface TreeNodeProps {
  node: FolderNode;
  selectedPath: string;
  onSelect: (path: string, isFile: boolean) => void;
}

// Individual TreeNode component (recursive)
const TreeNode: React.FC<TreeNodeProps> = ({
  node,
  selectedPath,
  onSelect,
}) => {
  const [expanded, setExpanded] = useState(false);

  // Auto-expand if this node or any of its children is selected
  useEffect(() => {
    // Skip if no selectedPath
    if (!selectedPath) return;

    const isExactMatch = selectedPath === node.path;
    const isPathParent =
      selectedPath.startsWith(node.path + "/") ||
      selectedPath.startsWith(node.path + "\\");

    if (isExactMatch || isPathParent) {
      console.log(
        "Expanding node:",
        node.path,
        "for selected path:",
        selectedPath
      );
      setExpanded(true);
    }
  }, [selectedPath, node.path, node.children]);

  // More flexible path comparison for selection
  const isSelected =
    selectedPath &&
    (selectedPath === node.path ||
      // Also check for trailing slash variations
      selectedPath + "/" === node.path ||
      selectedPath === node.path + "/");

  // Check if this node has JSON files as children
  const hasJsonChildren =
    node.isDirectory &&
    node.children?.some(
      (child) =>
        !child.isDirectory && child.name.toLowerCase().endsWith(".json")
    );

  // Count JSON files
  const jsonFileCount =
    node.isDirectory && node.children
      ? node.children.filter(
          (child) =>
            !child.isDirectory && child.name.toLowerCase().endsWith(".json")
        ).length
      : 0;

  const handleToggle = (e: React.MouseEvent) => {
    if (node.isDirectory) {
      e.stopPropagation();
      setExpanded(!expanded);
    }
  };

  const handleSelect = () => {
    const isFile = !node.isDirectory;
    console.log(`Selected node: ${node.path}, isFile: ${isFile}`);

    // When selecting a node, always invoke onSelect to update the URL
    onSelect(node.path, isFile);
  };

  // Determine which icon to show
  const getNodeIcon = () => {
    if (!node.isDirectory) {
      if (node.name.toLowerCase().endsWith(".json")) {
        return (
          <span className={`${styles.nodeIcon} ${styles.jsonFileIcon}`}>
            📊
          </span>
        );
      }
      return (
        <span className={`${styles.nodeIcon} ${styles.fileIcon}`}>📄</span>
      );
    }
    if (hasJsonChildren) {
      return (
        <span className={`${styles.nodeIcon} ${styles.folderIcon}`}>📂</span>
      );
    }
    return (
      <span className={`${styles.nodeIcon} ${styles.folderIcon}`}>📁</span>
    );
  };

  return (
    <div className={styles.treeNode}>
      <div
        className={`${styles.treeNodeItem} ${
          isSelected ? styles.selected : ""
        } ${hasJsonChildren ? styles.hasJsonContent : ""}`}
        onClick={handleSelect}
      >
        {node.isDirectory && (
          <span className={styles.expandIcon} onClick={handleToggle}>
            {expanded ? "▾" : "▸"}
          </span>
        )}
        {getNodeIcon()}
        <span className={styles.nodeName}>{node.name}</span>
        {jsonFileCount > 0 && (
          <span className={styles.jsonBadge}>{jsonFileCount}</span>
        )}
      </div>

      {expanded && node.children && node.children.length > 0 && (
        <div className={styles.treeNodeChildren}>
          {node.children.map((child) => (
            <TreeNode
              key={child.path}
              node={child}
              selectedPath={selectedPath}
              onSelect={onSelect}
            />
          ))}
        </div>
      )}
    </div>
  );
};

export { TreeView, TreeNode };
export default TreeView;
