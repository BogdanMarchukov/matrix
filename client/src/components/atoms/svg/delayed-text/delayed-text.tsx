import React, { useState, useEffect } from "react";

type DelayedTextProps = {
  x: number;
  y: number;
  fontSize?: string | number;
  textAnchor?: "start" | "middle" | "end";
  fill?: string;
  delay?: number;
  content: string;
};

const DelayedText: React.FC<DelayedTextProps> = ({
  x,
  y,
  fontSize = 16,
  textAnchor = "middle",
  fill = "#000",
  delay = 1000,
  content,
}) => {
  const [visible, setVisible] = useState(false);

  useEffect(() => {
    const timer = setTimeout(() => setVisible(true), delay);
    return () => clearTimeout(timer);
  }, [delay]);

  if (!visible) return null;

  return (
    <text x={x} y={y} fontSize={fontSize} textAnchor={textAnchor} fill={fill}>
      {content}
    </text>
  );
};

export default DelayedText;
