import React, { useState, useEffect, useCallback } from "react";
import ReactFlow, {
  Node,
  Edge,
  ConnectionLineType,
  MarkerType,
  useNodesState,
  useEdgesState,
  Background,
  Controls,
  MiniMap,
  Position,
  NodeTypes,
} from "reactflow";
import "reactflow/dist/style.css";
import styles from "./dependency_graph.module.scss";
import { Definition } from "../../models/math";

// Custom node renderer for different definition types
const CustomNode = ({ data }: { data: { label: string; type: string } }) => {
  // Determine color based on type
  let backgroundColor = "#4b8bf4"; // Default blue for Struct
  if (data.type === "Enum") {
    backgroundColor = "#f48b4b"; // Orange for Enum
  } else if (data.type === "Trait") {
    backgroundColor = "#8b4bf4"; // Purple for Trait
  }

  return (
    <div className={styles.customNode} style={{ backgroundColor }}>
      <div className={styles.nodeLabel} title={data.label}>
        {data.label.length > 15
          ? `${data.label.substring(0, 12)}...`
          : data.label}
      </div>
      <div className={styles.nodeType}>{data.type}</div>
    </div>
  );
};

// Define node types for ReactFlow
const nodeTypes: NodeTypes = {
  custom: CustomNode,
};

/**
 * Component for rendering a dependency graph of definitions using ReactFlow
 */
const DependencyGraph: React.FC<{ definitions: Definition[] }> = ({
  definitions,
}) => {
  const [initialized, setInitialized] = useState(false);

  // Create nodes and edges from definitions
  const createNodesAndEdges = useCallback(() => {
    if (!definitions.length) return { nodes: [], edges: [] };

    const nodes: Node[] = [];
    const edges: Edge[] = [];
    const nodeMap = new Map<string, Definition>();

    // Create nodes first
    definitions.forEach((def, index) => {
      const node: Node = {
        id: def.name,
        data: {
          label: def.name,
          type: def.kind,
        },
        type: "custom", // Use our custom node renderer
        position: { x: 0, y: index * 100 }, // Initial position, will be arranged by layout
        sourcePosition: Position.Right,
        targetPosition: Position.Left,
      };

      nodes.push(node);
      nodeMap.set(def.name, def);
    });

    // Helper function to get dependencies from member types
    const getTypeDependencies = (typeName: string): string[] => {
      // Remove generic parameters for dependency tracking
      const baseType = typeName.split("<")[0].trim();
      // Check if this type is one of our definitions
      return nodeMap.has(baseType) ? [baseType] : [];
    };

    // Create edges based on dependencies
    definitions.forEach((def) => {
      if (def.members) {
        def.members.forEach((member) => {
          if (member.type) {
            const dependencies = getTypeDependencies(member.type);
            dependencies.forEach((depId) => {
              // Create an edge
              edges.push({
                id: `e-${def.name}-${depId}`,
                source: def.name,
                target: depId,
                type: "smoothstep",
                markerEnd: {
                  type: MarkerType.ArrowClosed,
                },
                style: { stroke: "#999" },
              });
            });
          }
        });
      }
    });

    return { nodes, edges };
  }, [definitions]);

  const { nodes: initialNodes, edges: initialEdges } = createNodesAndEdges();
  const [nodes, setNodes, onNodesChange] = useNodesState(initialNodes);
  const [edges, setEdges, onEdgesChange] = useEdgesState(initialEdges);

  // Initialize or update the graph when definitions change
  useEffect(() => {
    const { nodes: newNodes, edges: newEdges } = createNodesAndEdges();
    setNodes(newNodes);
    setEdges(newEdges);
    setInitialized(true);
  }, [definitions, createNodesAndEdges, setNodes, setEdges]);

  // Handle node click (scroll to definition)
  const onNodeClick = useCallback((_: React.MouseEvent, node: Node) => {
    const element = document.getElementById(`definition-${node.id}`);
    if (element) {
      element.scrollIntoView({ behavior: "smooth", block: "center" });
      element.classList.add(styles.highlight);
      setTimeout(() => {
        element.classList.remove(styles.highlight);
      }, 1500);
    }
  }, []);

  // Apply automatic layout
  useEffect(() => {
    if (initialized && nodes.length > 0) {
      // Simple hierarchical layout
      // For a real app, you might want to use a proper layout algorithm
      // like dagre or elk, but for basic purposes we'll do a simple arrangement
      const gap = 150;
      const newNodes = nodes.map((node, index) => {
        return {
          ...node,
          position: {
            x: 50,
            y: index * gap + 50,
          },
        };
      });
      setNodes(newNodes);
    }
  }, [initialized, nodes.length, setNodes]);

  if (!definitions.length) {
    return (
      <div className={styles.graphContainer}>
        <div className={styles.emptyState}>No definitions to display</div>
      </div>
    );
  }

  return (
    <div className={styles.graphContainer}>
      <ReactFlow
        nodes={nodes}
        edges={edges}
        onNodesChange={onNodesChange}
        onEdgesChange={onEdgesChange}
        onNodeClick={onNodeClick}
        nodeTypes={nodeTypes}
        fitView
        className={styles.dependencyGraph}
        minZoom={0.2}
        maxZoom={1.5}
        defaultViewport={{ x: 0, y: 0, zoom: 0.8 }}
        connectionLineType={ConnectionLineType.SmoothStep}
        defaultEdgeOptions={{
          type: "smoothstep",
          markerEnd: { type: MarkerType.ArrowClosed },
        }}
      >
        <Background color="#aaa" gap={16} />
        <Controls />
        <MiniMap />
      </ReactFlow>
    </div>
  );
};

export default DependencyGraph;
