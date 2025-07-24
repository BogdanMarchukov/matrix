export type Point = {
  x: number;
  y: number;
};

export type CirclePoints = Point & { dtx: number, dty: number, stroke: string, fill: string, r: string; value: number };;

export type SquarePoints = Point & { collor: string, value: number }
