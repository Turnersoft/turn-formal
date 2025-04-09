import React, { useState, useEffect } from "react";
import styles from "./TreeView.module.scss";

// Interface for folder node data
interface FolderNode {
  name: string;
  path: string;
  isDirectory: boolean;
  children?: FolderNode[];
}

// Props for the TreeView component
interface TreeViewProps {
  nodes: FolderNode[];
  selectedPath: string;
  onSelect: (path: string) => void;
}

// Main TreeView component that displays a list of nodes
const TreeView: React.FC<TreeViewProps> = ({
  nodes,
  selectedPath,
  onSelect,
}) => {
  return (
    <div className={styles.treeView}>
      {nodes.map((node) => (
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
  onSelect: (path: string) => void;
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
    if (
      selectedPath === node.path ||
      (node.children && selectedPath.startsWith(node.path))
    ) {
      setExpanded(true);
    }
  }, [selectedPath, node.path, node.children]);

  const isSelected = selectedPath === node.path;

  const handleToggle = (e: React.MouseEvent) => {
    if (node.isDirectory) {
      e.stopPropagation();
      setExpanded(!expanded);
    }
  };

  const handleSelect = () => {
    if (node.isDirectory) {
      onSelect(node.path);
    }
  };

  return (
    <div className={styles.treeNode}>
      <div
        className={`${styles.treeNodeItem} ${
          isSelected ? styles.selected : ""
        }`}
        onClick={handleSelect}
      >
        {node.isDirectory && (
          <span className={styles.expandIcon} onClick={handleToggle}>
            {expanded ? "▾" : "▸"}
          </span>
        )}
        <span className={styles.nodeIcon}>
          {node.isDirectory ? "📁" : "📄"}
        </span>
        <span className={styles.nodeName}>{node.name}</span>
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
