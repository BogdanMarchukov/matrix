import React, { useEffect, useRef, useState } from "react";
import { CirclePoints, Point } from "../../../../common/types/astrology-cart";
import DelayedText from "../delayed-text/delayed-text";

type AnimatedLineProps = {
  from: Point;
  to: Point;
  stroke?: string;
  strokeWidth?: number;
  duration?: number;
  delay?: number;
  circlePoints?: CirclePoints[];
  strokeDasharray?: string;
};

const AnimatedLine: React.FC<AnimatedLineProps> = ({
  from,
  to,
  stroke = "#718096",
  strokeWidth = 1,
  duration = 1000,
  delay = 0,
  circlePoints = [],
  strokeDasharray = ""
}) => {
  const [end, setEnd] = useState<Point>(from);
  const currentCircles = useRef<CirclePoints[]>([]);

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
      const x = from.x + (to.x - from.x) * progress;
      const y = from.y + (to.y - from.y) * progress;

      const threshold = 3;
      circlePoints.forEach((p) => {
        const dx = p.x - x;
        const dy = p.y - y;
        const distance = Math.sqrt(dx * dx + dy * dy);
        if (distance <= threshold && !currentCircles.current.some(cp => cp.x === p.x && cp.y === p.y)) {
          currentCircles.current.push(p);
        }
      });

      setEnd({ x, y });

      if (progress < 1) {
        raf = requestAnimationFrame(animate);
      }
    };

    raf = requestAnimationFrame(animate);
    return () => cancelAnimationFrame(raf);
  }, [from, to, delay, duration]);

  return (
    <>
      <g id="lines-layer">
        <line
          x1={from.x}
          y1={from.y}
          x2={end.x}
          y2={end.y}
          stroke={stroke}
          strokeWidth={strokeWidth}
          strokeDasharray={strokeDasharray}
        />
      </g >
      {
        currentCircles.current.map((p, i) => (
          <>
            <g id="circles-layer">
              <circle
                key={i}
                cx={p.x}
                cy={p.y}
                stroke={p.stroke}
                fill={p.fill}
                r={p.r}
              />
              <DelayedText delay={0} content={p.value} x={p.x + p.dtx} y={p.y + p.dty} />
            </g>
          </>
        ))
      }
    </>
  );
};

export default AnimatedLine;
