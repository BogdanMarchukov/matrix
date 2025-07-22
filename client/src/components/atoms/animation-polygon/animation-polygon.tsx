import React, { useEffect, useState } from "react";

type Point = { x: number; y: number };

type AnimatedPolygonPathProps = {
  points: Point[];
  stroke?: string;
  strokeWidth?: number;
  duration?: number; // duration for full path
  delay?: number;
};

const AnimatedPolygonPath: React.FC<AnimatedPolygonPathProps> = ({
  points,
  stroke = "#4A5568",
  strokeWidth = 1,
  duration = 1000,
  delay = 0,
}) => {
  const [pathData, setPathData] = useState("");

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

      if (segmentIndex >= points.length) {
        // completed
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
  }, [points, duration, delay]);

  return (
    <path
      d={pathData}
      fill="none"
      stroke={stroke}
      strokeWidth={strokeWidth}
    />
  );
};

export default AnimatedPolygonPath;
