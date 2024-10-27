import {PropsWithChildren} from "react";
import styled, { keyframes } from "styled-components";
import classes from './animation-text.module.css'

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

interface AnimatedTextProps extends PropsWithChildren {
  animation: boolean;
}

export const AnimatedText = ({animation, children}: AnimatedTextProps) => {
  return (
    <span className={classes.text}>
      {animation
        ? String(children).split("").map((char, index) => (
            <Letter key={index} delay={index * 0.2}>
              {char}
            </Letter>
          ))
        : children}
    </span>
  );
};
