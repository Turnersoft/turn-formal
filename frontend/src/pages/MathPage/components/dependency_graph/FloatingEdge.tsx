import { useMemo } from "react";
import { EdgeProps, getSmoothStepPath } from "reactflow";

import styles from "./dependency_graph.module.scss";

// This component creates a floating edge that attaches to the closest point on each node
export default function FloatingEdge({
  id,
  // Unused but required by the EdgeProps interface
  source: _source,
  // Unused but required by the EdgeProps interface
  target: _target,
  sourceX,
  sourceY,
  targetX,
  targetY,
  sourcePosition,
  targetPosition,
  style = {},
  markerEnd,
  label,
  data,
}: EdgeProps) {
  // Using smoothstep path for better visibility
  const [edgePath, labelX, labelY] = getSmoothStepPath({
    sourceX,
    sourceY,
    sourcePosition,
    targetX,
    targetY,
    targetPosition,
  });

  const isArtificial = data?.isArtificial;

  // Determine styling based on whether it's an artificial connection
  const strokeColor = useMemo(
    () => (isArtificial ? "#999" : style.stroke || "#555"),
    [isArtificial, style.stroke]
  );

  const strokeWidth = useMemo(
    () => (isArtificial ? 1.5 : style.strokeWidth || 2),
    [isArtificial, style.strokeWidth]
  );

  const strokeDasharray = useMemo(
    () => (isArtificial ? "5,5" : undefined),
    [isArtificial]
  );

  return (
    <>
      <path
        id={id}
        style={{
          ...style,
          stroke: strokeColor,
          strokeWidth,
          strokeDasharray,
        }}
        className={styles.floatingEdgePath}
        d={edgePath}
        markerEnd={markerEnd}
      />
      {label && (
        <g className={styles.edgeLabelContainer}>
          <foreignObject
            width={100}
            height={40}
            x={labelX - 50}
            y={labelY - 20}
            className={styles.edgeLabelForeignObject}
            requiredExtensions="http://www.w3.org/1999/xhtml"
          >
            <div className={styles.edgeLabel}>
              <span>{label}</span>
            </div>
          </foreignObject>
        </g>
      )}
    </>
  );
}
