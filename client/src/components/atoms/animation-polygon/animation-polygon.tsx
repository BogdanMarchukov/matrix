import React, { useEffect, useRef, useState } from "react";

type Point = { x: number; y: number };
type SquarePoints = Point & { collor: string }

type AnimatedPolygonPathProps = {
  points: SquarePoints[];
  stroke?: string;
  strokeWidth?: number;
  duration?: number;
  delay?: number;
  onVertexReach?: (index: number) => void;
};

const AnimatedPolygonPath: React.FC<AnimatedPolygonPathProps> = ({
  points,
  stroke = "#4A5568",
  strokeWidth = 1,
  duration = 1000,
  delay = 0,
  onVertexReach
}) => {
  const [pathData, setPathData] = useState("");
  const [reachedVertices, setReachedVertices] = useState<number[]>([]);
  const prevSegmentRef = useRef<number>(-1);

  useEffect(() => {
    let raf: number;
    let startTime: number | null = null;

    const totalLength = points.length;
    const segmentDuration = duration / totalLength;

    const animate = (time: number) => {
      if (!startTime) startTime = time + delay;

      const elapsed = time - startTime;

      if (elapsed < 0) {
        raf = requestAnimationFrame(animate);
        return;
      }

      const segmentIndex = Math.floor(elapsed / segmentDuration);
      const localProgress = (elapsed % segmentDuration) / segmentDuration;

      if (segmentIndex !== prevSegmentRef.current && segmentIndex < points.length) {
        prevSegmentRef.current = segmentIndex;
        setReachedVertices((prev) =>
          prev.includes(segmentIndex) ? prev : [...prev, segmentIndex]
        );
        onVertexReach?.(segmentIndex);
      }

      if (segmentIndex >= points.length) {
        const fullPath = points
          .map((p, i) => `${i === 0 ? "M" : "L"} ${p.x} ${p.y}`)
          .join(" ") + " Z";
        setPathData(fullPath);
        return;
      }

      const currentPath = points.slice(0, segmentIndex + 1);
      const start = points[segmentIndex];
      const end = points[(segmentIndex + 1) % points.length];

      const interpolated: Point = {
        x: start.x + (end.x - start.x) * localProgress,
        y: start.y + (end.y - start.y) * localProgress,
      };

      const d =
        currentPath.map((p, i) => `${i === 0 ? "M" : "L"} ${p.x} ${p.y}`).join(" ") +
        ` L ${interpolated.x} ${interpolated.y}`;

      setPathData(d);
      raf = requestAnimationFrame(animate);
    };

    raf = requestAnimationFrame(animate);
    return () => cancelAnimationFrame(raf);
  }, [points, duration, delay, onVertexReach]);

  return (
    <>
      <path d={pathData} fill="none" stroke={stroke} strokeWidth={strokeWidth} />
      {reachedVertices.map((index) => {
        const point = points[index];
        return (
          <circle
            key={index}
            cx={point.x}
            cy={point.y}
            r={18}
            fill={point.collor}
          />
        );
      })}
    </>
  );
};

export default AnimatedPolygonPath;

