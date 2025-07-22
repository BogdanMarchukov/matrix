import React, { useEffect, useState } from "react";

type Point = {
  x: number;
  y: number;
};

type AnimatedLineProps = {
  from: Point;
  to: Point;
  stroke?: string;
  strokeWidth?: number;
  duration?: number;
  delay?: number;
};

const AnimatedLine: React.FC<AnimatedLineProps> = ({
  from,
  to,
  stroke = "#718096",
  strokeWidth = 1,
  duration = 1000,
  delay = 0,
}) => {
  const [end, setEnd] = useState<Point>(from);

  useEffect(() => {
    let startTime: number;
    let raf: number;
    const animate = (timestamp: number) => {
      if (!startTime) startTime = timestamp + delay;
      const elapsed = timestamp - startTime;
      if (elapsed < 0) {
        raf = requestAnimationFrame(animate);
        return;
      }

      const progress = Math.min(elapsed / duration, 1);

      setEnd({
        x: from.x + (to.x - from.x) * progress,
        y: from.y + (to.y - from.y) * progress,
      });

      if (progress < 1) {
        raf = requestAnimationFrame(animate);
      }
    };

    raf = requestAnimationFrame(animate);
    return () => cancelAnimationFrame(raf);
  }, [from, to, delay, duration]);

  return (
    <line
      x1={from.x}
      y1={from.y}
      x2={end.x}
      y2={end.y}
      stroke={stroke}
      strokeWidth={strokeWidth}
    />
  );
};

export default AnimatedLine;
