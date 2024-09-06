import React from "react";
import styled, { keyframes } from "styled-components";

// Ключевая анимация для переливания цветов текста
const letterAnimation = keyframes`
  0% {
    color: #770DF1;
  }
  50% {
    color: #FA55B8;
  }
  100% {
    color: #770DF1;
  }
`;

const Letter = styled.span<{ delay: number }>`
  display: inline-block;
  animation: ${letterAnimation} 2s ease-in-out infinite;
  animation-delay: ${({ delay }) => delay}s;
`;

const AnimatedText: React.FC<{ text: string; animation: boolean }> = ({
  text,
  animation,
}) => {
  return (
    <span>
      {animation
        ? text.split("").map((char, index) => (
            <Letter key={index} delay={index * 0.2}>
              {char}
            </Letter>
          ))
        : text}
    </span>
  );
};

export default AnimatedText;
