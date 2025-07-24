import { motion } from "framer-motion";
import React, { useMemo } from "react";
import { CirclePoints, Point, SquarePoints } from "../../common/types/astrology-cart";
import AnimatedLine from "../atoms/svg/animation-line/animation-lene";
import AnimatedPolygonPath from "../atoms/svg/animation-polygon/animation-polygon";
import DelayedText from "../atoms/svg/delayed-text/delayed-text";


type AnimatedLineProps = {
  show: boolean
  pointValue: number[][]
};


type Sector = {
  angle: number;
  outer: Point;
  labelPos: Point;
  value: number;
};

const degreesToRadians = (deg: number): number => (deg * Math.PI) / 180;

const polarToCartesian = (centerX: number, centerY: number, radius: number, angleInDegrees: number): Point => {
  const angleInRadians = degreesToRadians(angleInDegrees);
  return {
    x: centerX + radius * Math.cos(angleInRadians),
    y: centerY + radius * Math.sin(angleInRadians),
  };
};

const AstrologyChart: React.FC<AnimatedLineProps> = ({ show, pointValue }: AnimatedLineProps) => {
  const center: Point = { x: 300, y: 320 };
  const circleRadius = 240;

  const values = [
    20, 13, 14, 30, 16, 17, 40, 19, 21, 50, 1, 61,
    60, 2, 3, 70, 5, 6, 0, 8, 9, 10, 11, 12
  ];

  const sectors: Sector[] = Array.from({ length: 64 }, (_, i) => {
    const angle = i * 5.625 - 90;
    const outer = polarToCartesian(center.x, center.y, circleRadius, angle);
    const labelPos = polarToCartesian(center.x, center.y, circleRadius + 20, angle);
    return {
      angle,
      outer,
      labelPos,
      value: values[i] || -1,
    };
  });

  const getSquarePoints = (angle: number, collors?: string[], values?: number[]): SquarePoints[] => {
    const polygonRadius = 210;
    const squareAngles = [0, 90, 180, 270];
    const squarePoints = squareAngles.map((an, i) => {
      return {
        ...polarToCartesian(center.x, center.y, polygonRadius, an - angle),
        collor: collors ? collors[i] : '#ffffff',
        value: values ? values[i] : 0
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
        r: "14",
        value: pointValue[i * 2 + 2][0],
        dtx: 0,
        dty: 5
      })
      points.push({
        x: p.x + delta[1][i].x,
        y: p.y + delta[1][i].y,
        fill: delta[1][i].c,
        stroke: "#455568",
        r: "12",
        value: pointValue[i * 2 + 2][1],
        dtx: -1,
        dty: 4
      })
      if (i === 0) {
        points.push({
          x: p.x,
          y: p.y + 120,
          fill: "#2bee14",
          stroke: "#2bee14",
          r: "12",
          value: pointValue[2][2],
          dtx: 0,
          dty: 5
        })
      }
      if (i === 3) {
        points.push({
          x: p.x + 120,
          y: p.y,
          fill: "#2bee14",
          stroke: "#2bee14",
          r: "12",
          value: pointValue[8][2],
          dtx: 0,
          dty: 5
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
        r: "14",
        value: pointValue[i * 2 + 3][0],
        dtx: -2,
        dty: 4
      })
      points.push({
        x: p.x + delta[1][i].x,
        y: p.y + delta[1][i].y,
        fill: p.collor,
        stroke: "#455568",
        r: "12",
        value: pointValue[i * 2 + 3][1],
        dtx: -1,
        dty: 4
      })
      if (i === 1) {
        points.push({
          x: p.x - 59,
          y: p.y + - 59,
          fill: p.collor,
          stroke: "#4A5568",
          r: "10",
          value: pointValue[5][2],
          dtx: -1,
          dty: 3
        })
        points.push({
          x: p.x - 74,
          y: p.y + - 74,
          fill: p.collor,
          stroke: "#4A5568",
          r: "10",
          value: pointValue[5][3],
          dtx: -1,
          dty: 3
        })
      }
      if (i === 3) {
        points.push({
          x: p.x + 74,
          y: p.y + 74,
          fill: p.collor,
          stroke: "#4A556",
          r: "10",
          value: pointValue[9][2],
          dtx: 0,
          dty: 3
        })

      }
      return points
    });
    return [...points90, ...points45].flat(1)
  }

  const circlePoints90 = useMemo(() => getCirclePoints(), []);

  const showIndex = [0, 8, 16, 24, 32, 40, 48, 56]

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

        {sectors.map((s, idx) =>
          showIndex.includes(idx) ? (
            < AnimatedLine
              key={'line' + idx}
              from={center}
              to={s.outer}
              stroke={idx === 8 || idx === 24 ? "#e1112a" : idx === 40 || idx === 56 ? "#196fec" : "#718096"}
              strokeWidth={1}
              duration={1500}
              delayedText={100}
              delay={([0, 16, 32, 48].includes(idx) ? 0 : 1500) + 4400}
              circlePoints={circlePoints90}
            />
          ) : null
        )}

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

        {sectors.map((s, idx) => {
          if (true) {
            return (
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
                  лет
                </text>
              </g>
            )
          } else {
            return null
          }
        })}

        < DelayedText
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
          x={center.x + 40}
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
          points={getSquarePoints(90, ['#8E61EF', '#e1112a', '#e1112a', '#8E61EF'], pointValue[0])}
          duration={2000}
          delay={2400}
          textDelay={200}
        />

        <AnimatedPolygonPath
          points={getSquarePoints(45, undefined, pointValue[1])}
          duration={2000}
          delay={200}
          textDelay={200}
        />
      </svg>
    </div >
  ) : null
};

export default AstrologyChart;

