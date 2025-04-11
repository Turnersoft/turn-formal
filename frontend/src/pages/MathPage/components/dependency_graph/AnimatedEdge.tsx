import React, { useEffect, useState, useMemo } from "react";
import { EdgeProps, getSmoothStepPath, BaseEdge } from "reactflow";

import styles from "./dependency_graph.module.scss";

// This component creates an animated edge with a moving dash pattern
export default function AnimatedEdge({
  id,
  source,
  target,
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
  const [edgePath, labelX, labelY] = getSmoothStepPath({
    sourceX,
    sourceY,
    sourcePosition,
    targetX,
    targetY,
    targetPosition,
  });

  const [dashOffset, setDashOffset] = useState(0);

  // Calculate path length once the component is mounted
  useEffect(() => {
    // Increase animation speed for better visibility
    const interval = setInterval(() => {
      setDashOffset((offset) => (offset - 2) % 40);
    }, 30);

    return () => {
      clearInterval(interval);
    };
  }, []);

  // Determine styles based on whether this is an artificial connection
  const isArtificial = data?.isArtificial;

  // Use different dash patterns for normal vs artificial connections
  const dashArray = useMemo(
    () => (isArtificial ? "5,10" : "10,15"),
    [isArtificial]
  );

  // Apply different colors for regular vs artificial connections
  const strokeColor = useMemo(
    () => (isArtificial ? "#999" : style.stroke || "#555"),
    [isArtificial, style.stroke]
  );

  // Slightly thicker lines for better visibility
  const strokeWidth = useMemo(
    () => (isArtificial ? 1.5 : style.strokeWidth || 2.5),
    [isArtificial, style.strokeWidth]
  );

  return (
    <>
      <path
        id={id}
        className={styles.animatedEdgePath}
        d={edgePath}
        markerEnd={markerEnd}
        style={{
          ...style,
          stroke: strokeColor,
          strokeWidth,
          strokeDasharray: dashArray,
          strokeDashoffset: dashOffset,
        }}
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
