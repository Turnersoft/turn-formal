import React, {
  useState,
  useEffect,
  useCallback,
  useMemo,
  useRef,
} from "react";
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
  BackgroundVariant,
  ReactFlowProvider,
  EdgeTypes,
  Panel,
  Handle,
} from "reactflow";
import dagre from "dagre";
import "reactflow/dist/style.css";
import styles from "./dependency_graph.module.scss";
import { Definition } from "../../models/math";
import FloatingEdge from "./FloatingEdge";
import AnimatedEdge from "./AnimatedEdge";

// Layout direction: TB = top to bottom, LR = left to right
type LayoutDirection = "TB" | "LR";
const LAYOUT_DIRECTION = "TB" as LayoutDirection;
const NODE_WIDTH = 180;
const NODE_HEIGHT = 80;

// Define custom edge types
const edgeTypes: EdgeTypes = {
  floating: FloatingEdge,
  animated: AnimatedEdge,
};

// Dagre graph layout setup
const getLayoutedElements = (
  nodes: Node[],
  edges: Edge[],
  direction = LAYOUT_DIRECTION
) => {
  // Create a new directed graph
  const dagreGraph = new dagre.graphlib.Graph();
  dagreGraph.setDefaultEdgeLabel(() => ({}));

  // Set graph options and direction
  const isHorizontal = direction === ("LR" as any);
  dagreGraph.setGraph({ rankdir: direction, nodesep: 80, ranksep: 100 });

  // Set nodes with their dimensions
  nodes.forEach((node) => {
    dagreGraph.setNode(node.id, { width: NODE_WIDTH, height: NODE_HEIGHT });
  });

  // Add edges to the graph
  edges.forEach((edge) => {
    dagreGraph.setEdge(edge.source, edge.target);
  });

  // Calculate layout with dagre
  dagre.layout(dagreGraph);

  // Retrieve positions from layout
  const layoutedNodes = nodes.map((node) => {
    const nodeWithPosition = dagreGraph.node(node.id);

    return {
      ...node,
      position: {
        x: nodeWithPosition.x - NODE_WIDTH / 2,
        y: nodeWithPosition.y - NODE_HEIGHT / 2,
      },
      sourcePosition: isHorizontal ? Position.Right : Position.Bottom,
      targetPosition: isHorizontal ? Position.Left : Position.Top,
    };
  });

  return { nodes: layoutedNodes, edges };
};

// Custom node renderer for different definition types
const CustomNode = ({ data }: { data: { label: string; type: string } }) => {
  // Determine color based on type
  let backgroundColor = "#4b8bf4"; // Default blue for Struct
  if (data.type.toLowerCase() === "enum") {
    backgroundColor = "#f48b4b"; // Orange for enum
  } else if (data.type.toLowerCase() === "struct") {
    backgroundColor = "#8b4bf4"; // Purple for Trait
  }

  // Check if layout is horizontal (LR) or vertical (TB)
  const isHorizontal = LAYOUT_DIRECTION === ("LR" as any);

  return (
    <>
      {/* Source handle - for outgoing connections */}
      <Handle
        type="source"
        position={isHorizontal ? Position.Right : Position.Bottom}
        style={{
          background: backgroundColor,
          width: "10px",
          height: "10px",
          border: "2px solid white",
          zIndex: 10,
        }}
        id="source"
      />

      <div className={styles.customNode} style={{ backgroundColor }}>
        <div className={styles.nodeLabel} title={data.label}>
          {data.label.length > 15
            ? `${data.label.substring(0, 12)}...`
            : data.label}
        </div>
        <div className={styles.nodeType}>{data.type}</div>
      </div>

      {/* Target handle - for incoming connections */}
      <Handle
        type="target"
        position={isHorizontal ? Position.Left : Position.Top}
        style={{
          background: backgroundColor,
          width: "10px",
          height: "10px",
          border: "2px solid white",
          zIndex: 10,
        }}
        id="target"
      />
    </>
  );
};

// Define node types for ReactFlow
const nodeTypes: NodeTypes = {
  custom: CustomNode,
};

/**
 * The inner component that renders the flow chart
 */
const DependencyGraphInner: React.FC<{ definitions: Definition[] }> = ({
  definitions,
}) => {
  // Add a ref to track the ReactFlow instance
  const reactFlowWrapper = useRef<HTMLDivElement>(null);
  const [layoutedNodes, setLayoutedNodes] = useState<Node[]>([]);
  const [layoutedEdges, setLayoutedEdges] = useState<Edge[]>([]);
  const [selectedEdgeType, setSelectedEdgeType] = useState<string>("animated");
  const [ready, setReady] = useState<boolean>(false);
  const [hoveredNode, setHoveredNode] = useState<string | null>(null);
  const [incomingNodes, setIncomingNodes] = useState<Set<string>>(new Set());
  const [outgoingNodes, setOutgoingNodes] = useState<Set<string>>(new Set());

  // Create graph data based on definitions
  const graphData = useMemo(() => {
    if (!definitions.length) return { nodes: [], edges: [] };

    // Build a map for node lookup
    const nodeMap = new Map<string, Definition>();
    definitions.forEach((def) => {
      nodeMap.set(def.name, def);
    });

    // Create nodes with initial positions
    const isHorizontal = LAYOUT_DIRECTION === ("LR" as any);

    const nodes: Node[] = definitions.map((def, index) => ({
      id: def.name,
      data: {
        label: def.name,
        type: def.kind,
      },
      type: "custom",
      position: { x: 0, y: index * 100 },
      sourcePosition: isHorizontal ? Position.Right : Position.Bottom,
      targetPosition: isHorizontal ? Position.Left : Position.Top,
    }));

    // Track all possible dependencies to create a fully connected graph
    const allEdges: Edge[] = [];
    const referenceMap = new Map<string, Set<string>>();

    // Initialize reference map
    definitions.forEach((def) => {
      referenceMap.set(def.name, new Set<string>());
    });

    // Fill reference map - A references B if A has a member of type B or has a type_link to B
    definitions.forEach((def) => {
      // Check member type references
      if (def.members) {
        def.members.forEach((member) => {
          if (member.type) {
            // Process potential generic types like Vec<Something>
            const typeNames = extractTypeNames(member.type);

            // Add each referenced type to the map
            typeNames.forEach((baseType) => {
              if (nodeMap.has(baseType) && baseType !== def.name) {
                // def references baseType
                referenceMap.get(def.name)!.add(baseType);
              }
            });
          }

          // Check for type_link in member
          if (
            member.type_link &&
            nodeMap.has(member.type_link) &&
            member.type_link !== def.name
          ) {
            referenceMap.get(def.name)!.add(member.type_link);
          }
        });
      }

      // Check for direct type_links on the definition itself
      if (def.type_links) {
        def.type_links.forEach((typeLink: string) => {
          if (nodeMap.has(typeLink) && typeLink !== def.name) {
            referenceMap.get(def.name)!.add(typeLink);
          }
        });
      }

      // Check for extends/implements relationships
      if (def.extends) {
        const extendedTypes = Array.isArray(def.extends)
          ? def.extends
          : [def.extends];
        extendedTypes.forEach((extendedType: string) => {
          if (nodeMap.has(extendedType) && extendedType !== def.name) {
            referenceMap.get(def.name)!.add(extendedType);
          }
        });
      }

      if (def.implements) {
        const implementedTypes = Array.isArray(def.implements)
          ? def.implements
          : [def.implements];
        implementedTypes.forEach((implementedType: string) => {
          if (nodeMap.has(implementedType) && implementedType !== def.name) {
            referenceMap.get(def.name)!.add(implementedType);
          }
        });
      }
    });

    // Create all edges - from the referencing node to the referenced node
    definitions.forEach((def) => {
      const references = referenceMap.get(def.name)!;
      references.forEach((referencedType) => {
        // Create unique edge ID to avoid duplicates
        const edgeId = `e-${def.name}-${referencedType}`;

        // Only add if edge doesn't already exist
        if (!allEdges.some((edge) => edge.id === edgeId)) {
          allEdges.push({
            id: edgeId,
            source: def.name, // The node that contains the reference
            target: referencedType, // The node being referenced
            type: selectedEdgeType,
            markerEnd: {
              type: MarkerType.ArrowClosed,
            },
            style: {
              stroke: "#555",
              strokeWidth: 2,
            },
            label: "references",
            // Force edges to connect at handles regardless of layout
            sourceHandle: null,
            targetHandle: null,
          });
        }
      });
    });

    // If a node has no connections, connect it to another node
    // This ensures all nodes are part of the connected graph
    const nodesToConnect = definitions.filter((def) => {
      const refs = referenceMap.get(def.name)!;
      const hasReferences = refs.size > 0;
      const isReferenced = definitions.some((otherDef) => {
        if (otherDef.name === def.name) return false;
        return referenceMap.get(otherDef.name)!.has(def.name);
      });

      return !hasReferences && !isReferenced;
    });

    // Connect orphaned nodes
    if (nodesToConnect.length > 0) {
      // Find a suitable node to connect to (preferably one with connections)
      const connectedNodes = definitions.filter((def) => {
        const refs = referenceMap.get(def.name)!;
        const hasReferences = refs.size > 0;
        const isReferenced = definitions.some((otherDef) => {
          if (otherDef.name === def.name) return false;
          return referenceMap.get(otherDef.name)!.has(def.name);
        });

        return hasReferences || isReferenced;
      });

      // Connect orphaned nodes to a common node or to each other
      const targetNode =
        connectedNodes.length > 0
          ? connectedNodes[0]
          : nodesToConnect.length > 1
          ? nodesToConnect[0]
          : null;

      if (targetNode) {
        nodesToConnect.forEach((node) => {
          if (node.name !== targetNode.name) {
            allEdges.push({
              id: `e-orphan-${node.name}-${targetNode.name}`,
              source: node.name,
              target: targetNode.name,
              type: selectedEdgeType,
              markerEnd: {
                type: MarkerType.ArrowClosed,
              },
              style: {
                stroke: "#999",
                strokeWidth: 1.5,
              },
              data: { isArtificial: true }, // Mark as artificial connection
            });
          }
        });
      }
    }

    console.log(
      `Created ${allEdges.length} connections between ${nodes.length} nodes`
    );

    return { nodes, edges: allEdges };
  }, [definitions, selectedEdgeType]);

  // Apply layout to the graph
  useEffect(() => {
    if (graphData.nodes.length > 0) {
      const { nodes: layoutedNodes, edges: layoutedEdges } =
        getLayoutedElements(graphData.nodes, graphData.edges);

      setLayoutedNodes(layoutedNodes);
      setLayoutedEdges(layoutedEdges);

      // Set ready flag after layout is complete
      setTimeout(() => {
        setReady(true);
      }, 50);
    }
  }, [graphData]);

  const [nodes, setNodes, onNodesChange] = useNodesState([]);
  const [edges, setEdges, onEdgesChange] = useEdgesState([]);

  // Update when layouted elements change
  useEffect(() => {
    if (layoutedNodes.length > 0 && layoutedEdges.length > 0) {
      console.log(
        "Setting nodes and edges:",
        layoutedNodes.length,
        layoutedEdges.length
      );

      // Clear existing nodes/edges first
      setNodes([]);
      setEdges([]);

      // Set nodes first
      setTimeout(() => {
        setNodes([...layoutedNodes]);

        // Then set edges after nodes are rendered
        setTimeout(() => {
          setEdges([...layoutedEdges]);

          // Force a re-render of edges after all updates
          setTimeout(() => {
            const viewport = document.querySelector(
              ".react-flow__viewport"
            ) as HTMLElement;
            if (viewport) {
              // Small transform change to force re-render
              const currentTransform = viewport.style.transform;
              viewport.style.transform = currentTransform + " translateZ(0)";

              // Reset back
              setTimeout(() => {
                viewport.style.transform = currentTransform;
              }, 10);
            }
          }, 100);
        }, 50);
      }, 20);
    }
  }, [layoutedNodes, layoutedEdges, setNodes, setEdges]);

  // Apply layout on button click
  const onLayout = useCallback(() => {
    setReady(false);

    const { nodes: layoutedNodes, edges: layoutedEdges } = getLayoutedElements(
      nodes,
      edges
    );

    setNodes([...layoutedNodes]);

    // Set edges after nodes have been updated
    setTimeout(() => {
      setEdges([...layoutedEdges]);
      setReady(true);
    }, 50);

    // Center and fit the view
    setTimeout(() => {
      const flowWrapper = document.querySelector(".react-flow") as HTMLElement;
      if (flowWrapper) {
        const { width, height } = flowWrapper.getBoundingClientRect();

        // Manual calculation of fit view
        const xMin = Math.min(...nodes.map((node) => node.position.x));
        const xMax = Math.max(
          ...nodes.map((node) => node.position.x + NODE_WIDTH)
        );
        const yMin = Math.min(...nodes.map((node) => node.position.y));
        const yMax = Math.max(
          ...nodes.map((node) => node.position.y + NODE_HEIGHT)
        );

        const xScale = width / (xMax - xMin + 200);
        const yScale = height / (yMax - yMin + 200);
        const zoom = Math.min(xScale, yScale, 1.5);

        // Center the graph
        const centerX = (xMin + xMax) / 2;
        const centerY = (yMin + yMax) / 2;

        // Update transform in the DOM
        const viewport = document.querySelector(
          ".react-flow__viewport"
        ) as HTMLElement;
        if (viewport) {
          viewport.style.transform = `translate(${
            width / 2 - centerX * zoom
          }px, ${height / 2 - centerY * zoom}px) scale(${zoom})`;
        }
      }
    }, 100);
  }, [nodes, edges, setNodes, setEdges]);

  // Toggle edge type
  const toggleEdgeType = useCallback(() => {
    setSelectedEdgeType((prevType) =>
      prevType === "animated" ? "floating" : "animated"
    );
  }, []);

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

  // Add onNodeMouseEnter and onNodeMouseLeave handlers
  const onNodeMouseEnter = useCallback(
    (_: React.MouseEvent, node: Node) => {
      setHoveredNode(node.id);

      // Find all nodes that depend on this node (incoming)
      const incoming = new Set<string>();
      // Find all nodes that this node depends on (outgoing)
      const outgoing = new Set<string>();

      edges.forEach((edge) => {
        if (edge.target === node.id) {
          // This edge ends at our hovered node, so the source depends on it
          incoming.add(edge.source);
        }
        if (edge.source === node.id) {
          // This edge starts at our hovered node, so it depends on the target
          outgoing.add(edge.target);
        }
      });

      setIncomingNodes(incoming);
      setOutgoingNodes(outgoing);

      // Dispatch custom event for the structure overview to use
      const event = new CustomEvent("nodeHover", {
        detail: {
          nodeId: node.id,
          incoming: Array.from(incoming),
          outgoing: Array.from(outgoing),
        },
      });
      document.dispatchEvent(event);
    },
    [edges]
  );

  const onNodeMouseLeave = useCallback(() => {
    setHoveredNode(null);
    setIncomingNodes(new Set());
    setOutgoingNodes(new Set());

    // Dispatch event to clear highlighting
    const event = new CustomEvent("nodeHover", {
      detail: {
        nodeId: null,
        incoming: [],
        outgoing: [],
      },
    });
    document.dispatchEvent(event);
  }, []);

  // Update node styles based on hover state
  useEffect(() => {
    if (!ready) return;

    const updatedNodes = nodes.map((node) => {
      let nodeStyle = { ...node.style };
      let className = "";

      if (hoveredNode) {
        if (node.id === hoveredNode) {
          // Hovered node
          className = styles.hoveredNode;
        } else if (incomingNodes.has(node.id)) {
          // Nodes that depend on the hovered node
          className = styles.incomingNode;
        } else if (outgoingNodes.has(node.id)) {
          // Nodes that the hovered node depends on
          className = styles.outgoingNode;
        } else {
          // Fade other nodes
          className = styles.dimmedNode;
        }
      }

      return {
        ...node,
        className,
        style: nodeStyle,
      };
    });

    // Update edges to highlight connections
    const updatedEdges = edges.map((edge) => {
      let className = "";

      if (hoveredNode) {
        if (edge.source === hoveredNode || edge.target === hoveredNode) {
          className =
            edge.source === hoveredNode
              ? styles.outgoingEdge
              : styles.incomingEdge;
        } else {
          className = styles.dimmedEdge;
        }
      }

      return {
        ...edge,
        className,
      };
    });

    setNodes(updatedNodes);
    setEdges(updatedEdges);

    // Also update structure overview items
    if (hoveredNode) {
      // Update structure overview items with appropriate classes
      document.querySelectorAll("[data-node-id]").forEach((item: Element) => {
        const nodeId = (item as HTMLElement).dataset.nodeId;
        if (!nodeId) return;

        // Remove all highlighting classes first
        item.classList.remove("hovered", "incoming", "outgoing", "dimmed");

        if (nodeId === hoveredNode) {
          item.classList.add("hovered");
        } else if (incomingNodes.has(nodeId)) {
          item.classList.add("incoming");
        } else if (outgoingNodes.has(nodeId)) {
          item.classList.add("outgoing");
        } else {
          item.classList.add("dimmed");
        }
      });
    } else {
      // Reset all structure overview items
      document.querySelectorAll("[data-node-id]").forEach((item: Element) => {
        item.classList.remove("hovered", "incoming", "outgoing", "dimmed");
      });
    }
  }, [
    hoveredNode,
    incomingNodes,
    outgoingNodes,
    nodes,
    edges,
    ready,
    setNodes,
    setEdges,
  ]);

  // Listen for hover events from the structure overview
  useEffect(() => {
    const handleOverviewHover = (event: Event) => {
      const customEvent = event as CustomEvent;
      const nodeId = customEvent.detail.nodeId;

      if (nodeId) {
        // Simulate node mouse enter
        const node = nodes.find((n) => n.id === nodeId);
        if (node) {
          onNodeMouseEnter({} as React.MouseEvent, node);
        }
      } else {
        // Simulate node mouse leave
        onNodeMouseLeave();
      }
    };

    document.addEventListener("overviewHover", handleOverviewHover);

    return () => {
      document.removeEventListener("overviewHover", handleOverviewHover);
    };
  }, [nodes, onNodeMouseEnter, onNodeMouseLeave]);

  if (!definitions.length) {
    return (
      <div className={styles.graphContainer}>
        <div className={styles.emptyState}>No definitions to display</div>
      </div>
    );
  }

  return (
    <div className={styles.graphContainer}>
      <div className={styles.layoutControls}>
        <button onClick={onLayout} className={styles.layoutButton}>
          Rearrange Layout
        </button>
        <button onClick={toggleEdgeType} className={styles.layoutButton}>
          {selectedEdgeType === "animated"
            ? "Use Floating Edges"
            : "Use Animated Edges"}
        </button>
        <div className={styles.layoutInfo}>
          {nodes.length} nodes · {edges.length} connections
          {!ready && (
            <span className={styles.loadingIndicator}> (Loading...)</span>
          )}
        </div>
      </div>
      <div ref={reactFlowWrapper} className={styles.reactFlowWrapper}>
        <ReactFlow
          nodes={nodes}
          edges={edges}
          onNodesChange={onNodesChange}
          onEdgesChange={onEdgesChange}
          onNodeClick={onNodeClick}
          onNodeMouseEnter={onNodeMouseEnter}
          onNodeMouseLeave={onNodeMouseLeave}
          nodeTypes={nodeTypes}
          edgeTypes={edgeTypes}
          fitView
          fitViewOptions={{ padding: 0.4 }}
          className={styles.dependencyGraph}
          minZoom={0.2}
          maxZoom={1.5}
          defaultViewport={{ x: 0, y: 0, zoom: 0.8 }}
          connectionLineType={ConnectionLineType.SmoothStep}
          defaultEdgeOptions={{
            type: selectedEdgeType,
            markerEnd: { type: MarkerType.ArrowClosed },
            style: { strokeWidth: 2 },
          }}
          nodesDraggable={true}
          elementsSelectable={true}
          selectNodesOnDrag={false}
          snapToGrid={true}
          snapGrid={[15, 15]}
          nodesConnectable={true}
        >
          <Background color="#aaa" gap={16} variant={BackgroundVariant.Dots} />
          <Controls />
          <Panel position="bottom-right" className={styles.statsPanel}>
            <div className={styles.statItem}>
              <span>{nodes.length}</span> nodes
            </div>
            <div className={styles.statItem}>
              <span>{edges.length}</span> connections
            </div>
          </Panel>
          <MiniMap
            nodeStrokeColor={(n) => {
              if (n.type === "custom") return "#fff";
              return "#555";
            }}
            nodeColor={(n) => {
              if (n.data.type.toLowerCase() === "enum") return "#f48b4b";
              if (n.data.type === "Trait") return "#8b4bf4";
              return "#4b8bf4";
            }}
            maskColor="rgba(0, 0, 0, 0.05)"
          />
        </ReactFlow>
      </div>
    </div>
  );
};

/**
 * Wrapper component that provides the ReactFlow context
 */
const DependencyGraph: React.FC<{ definitions: Definition[] }> = (props) => {
  return (
    <ReactFlowProvider>
      <DependencyGraphInner {...props} />
    </ReactFlowProvider>
  );
};

// Helper function to extract type names from type strings including generics
function extractTypeNames(typeStr: string): string[] {
  const types: string[] = [];

  // Remove whitespace to make parsing simpler
  typeStr = typeStr.replace(/\s+/g, "");

  // Simple cases without generics
  if (!typeStr.includes("<")) {
    types.push(typeStr);
    return types;
  }

  // Extract base type
  const baseType = typeStr.split("<")[0];
  types.push(baseType);

  // Extract generic type parameters
  let genericPart = typeStr.substring(typeStr.indexOf("<") + 1);

  // Remove the closing '>' if it exists
  if (genericPart.endsWith(">")) {
    genericPart = genericPart.substring(0, genericPart.length - 1);
  }

  // Split by commas, but be careful about nested generics
  let depth = 0;
  let currentType = "";

  for (let i = 0; i < genericPart.length; i++) {
    const char = genericPart[i];

    if (char === "<") {
      depth++;
      currentType += char;
    } else if (char === ">") {
      depth--;
      currentType += char;
    } else if (char === "," && depth === 0) {
      if (currentType) {
        types.push(...extractTypeNames(currentType));
        currentType = "";
      }
    } else {
      currentType += char;
    }
  }

  // Add the last type if any
  if (currentType) {
    types.push(...extractTypeNames(currentType));
  }

  return types;
}

export default DependencyGraph;
