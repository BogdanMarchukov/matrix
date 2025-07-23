import React, { useMemo } from "react";
import { motion } from "framer-motion";
import { CirclePoints } from "../../common/types/astrology-cart";
import AnimatedLine from "../atoms/svg/animation-line/animation-lene";
import AnimatedPolygonPath from "../atoms/svg/animation-polygon/animation-polygon";
import DelayedText from "../atoms/svg/delayed-text/delayed-text";

type AnimatedLineProps = {
  show: boolean
};

type Point = {
  x: number;
  y: number;
};

type SquarePoints = Point & { collor: string }

type Sector = {
  angle: number;
  outer: Point;
  labelPos: Point;
  value: number;
  smallText: number;
};

const degreesToRadians = (deg: number): number => (deg * Math.PI) / 180;

const polarToCartesian = (centerX: number, centerY: number, radius: number, angleInDegrees: number): Point => {
  const angleInRadians = degreesToRadians(angleInDegrees);
  return {
    x: centerX + radius * Math.cos(angleInRadians),
    y: centerY + radius * Math.sin(angleInRadians),
  };
};

const AstrologyChart: React.FC<AnimatedLineProps> = ({ show }: AnimatedLineProps) => {
  const center: Point = { x: 300, y: 320 };
  const circleRadius = 240;

  const values = [
    12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 0, 1,
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12
  ];

  const sectors: Sector[] = Array.from({ length: 24 }, (_, i) => {
    const angle = i * 15 - 90;
    const outer = polarToCartesian(center.x, center.y, circleRadius, angle);
    const labelPos = polarToCartesian(center.x, center.y, circleRadius + 20, angle);
    return {
      angle,
      outer,
      labelPos,
      value: values[i],
      smallText: i * 5,
    };
  });

  const getSquarePoints = (angle: number, collors?: string[]): SquarePoints[] => {
    const polygonRadius = 210;
    const squareAngles = [0, 90, 180, 270];
    const squarePoints = squareAngles.map((an, i) => {
      return {
        ...polarToCartesian(center.x, center.y, polygonRadius, an - angle),
        collor: collors ? collors[i] : '#ffffff'
      }
    });
    return squarePoints;
  }

  const getCirclePoints = (): CirclePoints[] => {
    const points90 = getSquarePoints(90, ['#8E61EF', '#e1112a', '#e1112a', '#8E61EF']).map((p, i, arr) => {
      const delta = [
        [{ x: -0, y: 33, c: "#196fec" }, { x: -33, y: 0, c: "#f3a834" }, { x: 0, y: -33, c: "#f3a834" }, { x: 33, y: 0, c: "#196fec" }],
        [{ x: 0, y: 62, c: "#14b9ee" }, { x: -62, y: 0, c: "#ffffff" }, { x: 0, y: -62, c: "#ffffff" }, { x: 62, y: 0, c: "#14b9ee" }]
      ];
      const points: CirclePoints[] = [];
      points.push({
        x: p.x + delta[0][i].x,
        y: p.y + delta[0][i].y,
        fill: delta[0][i].c,
        stroke: "#455568",
        r: "14"
      })
      points.push({
        x: p.x + delta[1][i].x,
        y: p.y + delta[1][i].y,
        fill: delta[1][i].c,
        stroke: "#455568",
        r: "12"
      })
      if (i === 0) {
        points.push({
          x: p.x,
          y: p.y + 120,
          fill: "#2bee14",
          stroke: "#2bee14",
          r: "12"
        })
      }
      if (i === 3) {
        points.push({
          x: p.x + 120,
          y: p.y,
          fill: "#2bee14",
          stroke: "#2bee14",
          r: "12"
        })

      }
      return points
    });

    const points45 = getSquarePoints(45).map((p, i, arr) => {
      const delta = [
        [{ x: -23, y: 23 }, { x: -23, y: -23 }, { x: 23, y: -23 }, { x: 23, y: 23 }],
        [{ x: -43, y: 43 }, { x: -43, y: -43 }, { x: 43, y: -43 }, { x: 43, y: 43 }]
      ];
      const points: CirclePoints[] = [];
      points.push({
        x: p.x + delta[0][i].x,
        y: p.y + delta[0][i].y,
        fill: p.collor,
        stroke: "#455568",
        r: "14"
      })
      points.push({
        x: p.x + delta[1][i].x,
        y: p.y + delta[1][i].y,
        fill: p.collor,
        stroke: "#455568",
        r: "12"
      })
      if (i === 1) {
        points.push({
          x: p.x - 59,
          y: p.y + - 59,
          fill: p.collor,
          stroke: "#4A5568",
          r: "10"
        })
        points.push({
          x: p.x - 74,
          y: p.y + - 74,
          fill: p.collor,
          stroke: "#4A5568",
          r: "10"
        })
      }
      if (i === 3) {
        points.push({
          x: p.x + 74,
          y: p.y + 74,
          fill: p.collor,
          stroke: "#4A556",
          r: "10"
        })

      }
      return points
    });
    return [...points90, ...points45].flat(1)
  }

  const circlePoints90 = useMemo(() => getCirclePoints(), []);

  const showIndex = [0, 3, 6, 9, 12, 15, 18, 21]

  return show ? (
    <div>
      <svg viewBox="0 0 600 660">
        <defs>
          <filter id="dropShadow" x="-50%" y="-50%" width="200%" height="200%">
            <feDropShadow dx="0" dy="2" stdDeviation="4" floodColor="#000" floodOpacity="0.4" />
          </filter>
        </defs>

        <circle
          cx={center.x}
          cy={center.y}
          r={circleRadius}
          stroke="#FFD700"
          strokeWidth={2}
          filter="url(#dropShadow)"
        />

        {sectors.map((s, idx) =>
          showIndex.includes(idx) ? (
            < AnimatedLine
              key={idx}
              from={center}
              to={s.outer}
              stroke={idx === 3 || idx === 15 ? "#e1112a" : idx === 9 || idx === 21 ? "#196fec" : "#718096"}
              strokeWidth={1}
              duration={1500}
              delay={([3, 15, 9, 21].includes(idx) ? 1500 : 0) + 4400}
              circlePoints={circlePoints90}
            />
          ) : null
        )}
        <AnimatedLine
          from={{
            x: circlePoints90[1].x,
            y: circlePoints90[1].y
          }}
          to={{
            x: circlePoints90[8].x,
            y: circlePoints90[8].y
          }}
          delay={6900}
          strokeDasharray="20 10"
          stroke="#455568"
          duration={1500}
        />

        <AnimatedLine
          from={{
            x: circlePoints90[4].x,
            y: circlePoints90[4].y
          }}
          to={{
            x: circlePoints90[6].x,
            y: circlePoints90[6].y
          }}
          delay={6900}
          strokeDasharray="20 10"
          stroke="#455568"
          duration={1500}
        />

        <AnimatedLine
          from={{
            x: circlePoints90[5].x,
            y: circlePoints90[5].y
          }}
          to={{
            x: circlePoints90[3].x,
            y: circlePoints90[3].y
          }}
          delay={6900}
          strokeDasharray="20 10"
          stroke="#455568"
          duration={1500}
        />

        <motion.circle
          cx={center.x}
          cy={center.y}
          r={30}
          fill="#ecc94b"
          animate={{ r: [30, 36, 30], opacity: [1, 0.6, 1] }}
          transition={{ duration: 1.5, repeat: 2, ease: "easeInOut" }}
        />
        <text x={center.x} y={center.y + 6} textAnchor="middle" fontSize="20" fontWeight="bold" fill="#1A202C">
          12
        </text>

        {sectors.map((s, idx) => (
          <g key={`text-${idx}`}>
            <text
              x={s.labelPos.x}
              y={s.labelPos.y}
              textAnchor="middle"
              fontSize="14"
              fontWeight="bold"
              fill="#E2E8F0"
            >
              {s.value}
            </text>
            <text
              x={s.labelPos.x}
              y={s.labelPos.y + 12}
              textAnchor="middle"
              fontSize="10"
              fill="#A0AEC0"
            >
              {s.smallText} лет
            </text>
          </g>
        ))}

        <DelayedText
          x={center.x - 40}
          y={center.y - 35}
          fontSize="25"
          textAnchor="middle"
          fill="#196fec"
          delay={8400}
          content="♂"
        />

        <DelayedText
          x={center.x + 50}
          y={center.y - 40}
          fontSize="25"
          textAnchor="middle"
          fill="#E53E3E"
          delay={8400}
          content="♀"

        />

        <DelayedText
          x={center.x - 40}
          y={center.y + 115}
          fontSize="20"
          textAnchor="middle"
          fill="#E53E3E"
          delay={8400}
          content="❤️"
        />

        <DelayedText
          x={center.x + 125}
          y={center.y + 60}
          fontSize="25"
          textAnchor="middle"
          fill="#E53E3E"
          delay={8400}
          content="💰"
        />

        <AnimatedPolygonPath
          points={getSquarePoints(90, ['#8E61EF', '#e1112a', '#e1112a', '#8E61EF'])}
          duration={2000}
          delay={2400}
        />
        <AnimatedPolygonPath
          points={getSquarePoints(45)}
          duration={2000}
          delay={200}
        />
      </svg>
    </div >
  ) : null
};

export default AstrologyChart;

