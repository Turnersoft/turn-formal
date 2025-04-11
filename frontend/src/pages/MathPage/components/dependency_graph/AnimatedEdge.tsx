import { useEffect, useState, useMemo } from "react";
import { EdgeProps, getSmoothStepPath } from "reactflow";

interface AnimatedEdgeProps extends EdgeProps {
  animated?: boolean;
}

const AnimatedEdge = ({
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
  animated = false,
}: AnimatedEdgeProps) => {
  const [animation, setAnimation] = useState(0);

  // Create animation effect
  useEffect(() => {
    if (animated) {
      const interval = setInterval(() => {
        setAnimation((a) => (a + 1) % 10);
      }, 50);
      return () => clearInterval(interval);
    }
  }, [animated]);

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  const [edgePath, _labelX, _labelY] = useMemo(
    () =>
      getSmoothStepPath({
        sourceX,
        sourceY,
        sourcePosition,
        targetX,
        targetY,
        targetPosition,
      }),
    [sourceX, sourceY, sourcePosition, targetX, targetY, targetPosition]
  );

  const dashOffset = useMemo(() => {
    return animated ? 10 - animation : 0;
  }, [animated, animation]);

  const animatedStyle = {
    ...style,
    strokeDasharray: animated ? "5,5" : "none",
    strokeDashoffset: dashOffset,
  };

  return (
    <path
      id={id}
      className="react-flow__edge-path"
      d={edgePath}
      style={animatedStyle}
      markerEnd={markerEnd}
    />
  );
};

export default AnimatedEdge;
