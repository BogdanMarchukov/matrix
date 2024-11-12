import styled, { keyframes, css } from "styled-components";
import * as React from "react";

interface SvgProps {
  animate?: boolean;
}

const gradientAnimation = keyframes`
    0% { stop-color: #770DF1; }
    50% { stop-color: #FA55B8; }
    100% { stop-color: #770DF1; }
`;

const Svg = styled.svg<SvgProps>`
  #animatedGradient stop {
    ${({ animate }) => animate ? css`animation: ${gradientAnimation} 3s ease infinite;` : "none"};
  }
`;

export const NextIcon = ({ animate }: SvgProps) => {
  return (
    <Svg animate={animate} width={6} height={12} fill="none">
        <path
          fill="#BCB8D9"
          d="m.157.932 4.611 4.685a.546.546 0 0 1 0 .768L.158 11.07a.55.55 0 0 0 .174.89.53.53 0 0 0 .585-.119l4.61-4.684a1.65 1.65 0 0 0 0-2.312L.917.161a.53.53 0 0 0-.76 0 .55.55 0 0 0 0 .771Z"
          opacity={0.9}
        />
    </Svg>
  )
}
