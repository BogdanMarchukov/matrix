import styled, {keyframes, css} from "styled-components";

interface SvgProps {
  animate: boolean;
}

const gradientAnimation = keyframes`
    0% {
        stop-color: #770DF1
    }
    50% {
        stop-color: #FA55B8
    }
    100% {
        stop-color: #770DF1
    }
`;

const Svg = styled.svg<SvgProps>`
    #animatedGradient stop {
        ${({animate}) => animate ? css`animation: ${gradientAnimation} 3s ease infinite;` : "none"};
    }
`;

export const AboutIcon = ({animate}: SvgProps) => {
  return (
    <Svg animate={animate} width={28} height={28} viewBox="0 0 28 28" fill="none">
      <g filter="url(#filter0_f_31_570)">
        <path
          d="M14 24C8.477 24 4 19.523 4 14C4 8.477 8.477 4 14 4C19.523 4 24 8.477 24 14C24 19.523 19.523 24 14 24ZM14 12C13.7348 12 13.4804 12.1054 13.2929 12.2929C13.1054 12.4804 13 12.7348 13 13V18C13 18.2652 13.1054 18.5196 13.2929 18.7071C13.4804 18.8946 13.7348 19 14 19C14.2652 19 14.5196 18.8946 14.7071 18.7071C14.8946 18.5196 15 18.2652 15 18V13C15 12.7348 14.8946 12.4804 14.7071 12.2929C14.5196 12.1054 14.2652 12 14 12ZM14 11C14.2652 11 14.5196 10.8946 14.7071 10.7071C14.8946 10.5196 15 10.2652 15 10C15 9.73478 14.8946 9.48043 14.7071 9.29289C14.5196 9.10536 14.2652 9 14 9C13.7348 9 13.4804 9.10536 13.2929 9.29289C13.1054 9.48043 13 9.73478 13 10C13 10.2652 13.1054 10.5196 13.2929 10.7071C13.4804 10.8946 13.7348 11 14 11Z"
          fill="#FFA5FE"/>
      </g>
      <g filter="url(#filter1_f_31_570)">
        <path
          d="M14 24C8.477 24 4 19.523 4 14C4 8.477 8.477 4 14 4C19.523 4 24 8.477 24 14C24 19.523 19.523 24 14 24ZM14 12C13.7348 12 13.4804 12.1054 13.2929 12.2929C13.1054 12.4804 13 12.7348 13 13V18C13 18.2652 13.1054 18.5196 13.2929 18.7071C13.4804 18.8946 13.7348 19 14 19C14.2652 19 14.5196 18.8946 14.7071 18.7071C14.8946 18.5196 15 18.2652 15 18V13C15 12.7348 14.8946 12.4804 14.7071 12.2929C14.5196 12.1054 14.2652 12 14 12ZM14 11C14.2652 11 14.5196 10.8946 14.7071 10.7071C14.8946 10.5196 15 10.2652 15 10C15 9.73478 14.8946 9.48043 14.7071 9.29289C14.5196 9.10536 14.2652 9 14 9C13.7348 9 13.4804 9.10536 13.2929 9.29289C13.1054 9.48043 13 9.73478 13 10C13 10.2652 13.1054 10.5196 13.2929 10.7071C13.4804 10.8946 13.7348 11 14 11Z"
          fill="white"/>
      </g>
      <path
        d="M14 23C9.0293 23 5 18.9707 5 14C5 9.0293 9.0293 5 14 5C18.9707 5 23 9.0293 23 14C23 18.9707 18.9707 23 14 23ZM14 12.2C13.7613 12.2 13.5324 12.2948 13.3636 12.4636C13.1948 12.6324 13.1 12.8613 13.1 13.1V17.6C13.1 17.8387 13.1948 18.0676 13.3636 18.2364C13.5324 18.4052 13.7613 18.5 14 18.5C14.2387 18.5 14.4676 18.4052 14.6364 18.2364C14.8052 18.0676 14.9 17.8387 14.9 17.6V13.1C14.9 12.8613 14.8052 12.6324 14.6364 12.4636C14.4676 12.2948 14.2387 12.2 14 12.2ZM14 11.3C14.2387 11.3 14.4676 11.2052 14.6364 11.0364C14.8052 10.8676 14.9 10.6387 14.9 10.4C14.9 10.1613 14.8052 9.93239 14.6364 9.7636C14.4676 9.59482 14.2387 9.5 14 9.5C13.7613 9.5 13.5324 9.59482 13.3636 9.7636C13.1948 9.93239 13.1 10.1613 13.1 10.4C13.1 10.6387 13.1948 10.8676 13.3636 11.0364C13.5324 11.2052 13.7613 11.3 14 11.3Z"
        fill="white"/>
      <defs>
        <filter id="filter0_f_31_570" x="0" y="0" width="28" height="28" filterUnits="userSpaceOnUse"
                colorInterpolationFilters="sRGB">
          <feFlood floodOpacity="0" result="BackgroundImageFix"/>
          <feBlend mode="normal" in="SourceGraphic" in2="BackgroundImageFix" result="shape"/>
          <feGaussianBlur stdDeviation="2" result="effect1_foregroundBlur_31_570"/>
        </filter>
        <filter id="filter1_f_31_570" x="2" y="2" width="24" height="24" filterUnits="userSpaceOnUse"
                colorInterpolationFilters="sRGB">
          <feFlood floodOpacity="0" result="BackgroundImageFix"/>
          <feBlend mode="normal" in="SourceGraphic" in2="BackgroundImageFix" result="shape"/>
          <feGaussianBlur stdDeviation="1" result="effect1_foregroundBlur_31_570"/>
        </filter>
      </defs>
    </Svg>
  )
}
