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

export const TestsIcon = ({animate}: SvgProps) => {
  return (
    <Svg animate={animate} width={28} height={28} viewBox="0 0 28 28" fill="none">
      <g filter="url(#filter0_f_31_533)">
        <path fillRule="evenodd" clipRule="evenodd"
              d="M14 24C15.3132 24 16.6136 23.7413 17.8268 23.2388C19.0401 22.7362 20.1425 21.9997 21.0711 21.0711C21.9997 20.1425 22.7362 19.0401 23.2388 17.8268C23.7413 16.6136 24 15.3132 24 14C24 12.6868 23.7413 11.3864 23.2388 10.1732C22.7362 8.95991 21.9997 7.85752 21.0711 6.92893C20.1425 6.00035 19.0401 5.26375 17.8268 4.7612C16.6136 4.25866 15.3132 4 14 4C11.3478 4 8.8043 5.05357 6.92893 6.92893C5.05357 8.8043 4 11.3478 4 14C4 16.6522 5.05357 19.1957 6.92893 21.0711C8.8043 22.9464 11.3478 24 14 24ZM13.7422 18.0444L19.2978 11.3778L17.5911 9.95556L12.8133 15.6878L10.3411 13.2144L8.77 14.7856L12.1033 18.1189L12.9633 18.9789L13.7422 18.0444Z"
              fill="#81FF9D"/>
      </g>
      <g filter="url(#filter1_f_31_533)">
        <path fillRule="evenodd" clipRule="evenodd"
              d="M14 23C15.1819 23 16.3522 22.7672 17.4442 22.3149C18.5361 21.8626 19.5282 21.1997 20.364 20.364C21.1997 19.5282 21.8626 18.5361 22.3149 17.4442C22.7672 16.3522 23 15.1819 23 14C23 12.8181 22.7672 11.6478 22.3149 10.5558C21.8626 9.46392 21.1997 8.47177 20.364 7.63604C19.5282 6.80031 18.5361 6.13738 17.4442 5.68508C16.3522 5.23279 15.1819 5 14 5C11.6131 5 9.32387 5.94821 7.63604 7.63604C5.94821 9.32387 5 11.6131 5 14C5 16.3869 5.94821 18.6761 7.63604 20.364C9.32387 22.0518 11.6131 23 14 23ZM13.768 17.64L18.768 11.64L17.232 10.36L12.932 15.519L10.707 13.293L9.293 14.707L12.293 17.707L13.067 18.481L13.768 17.64Z"
              fill="white"/>
      </g>
      <path fillRule="evenodd" clipRule="evenodd"
            d="M14 23C15.1819 23 16.3522 22.7672 17.4442 22.3149C18.5361 21.8626 19.5282 21.1997 20.364 20.364C21.1997 19.5282 21.8626 18.5361 22.3149 17.4442C22.7672 16.3522 23 15.1819 23 14C23 12.8181 22.7672 11.6478 22.3149 10.5558C21.8626 9.46392 21.1997 8.47177 20.364 7.63604C19.5282 6.80031 18.5361 6.13738 17.4442 5.68508C16.3522 5.23279 15.1819 5 14 5C11.6131 5 9.32387 5.94821 7.63604 7.63604C5.94821 9.32387 5 11.6131 5 14C5 16.3869 5.94821 18.6761 7.63604 20.364C9.32387 22.0518 11.6131 23 14 23ZM13.768 17.64L18.768 11.64L17.232 10.36L12.932 15.519L10.707 13.293L9.293 14.707L12.293 17.707L13.067 18.481L13.768 17.64Z"
            fill="white"/>
      <defs>
        <filter id="filter0_f_31_533" x="0.34638" y="0.34638" width="27.3072" height="27.3072"
                filterUnits="userSpaceOnUse" colorInterpolationFilters="sRGB">
          <feFlood floodOpacity="0" result="BackgroundImageFix"/>
          <feBlend mode="normal" in="SourceGraphic" in2="BackgroundImageFix" result="shape"/>
          <feGaussianBlur stdDeviation="1.82681" result="effect1_foregroundBlur_31_533"/>
        </filter>
        <filter id="filter1_f_31_533" x="3" y="3" width="22" height="22" filterUnits="userSpaceOnUse"
                colorInterpolationFilters="sRGB">
          <feFlood floodOpacity="0" result="BackgroundImageFix"/>
          <feBlend mode="normal" in="SourceGraphic" in2="BackgroundImageFix" result="shape"/>
          <feGaussianBlur stdDeviation="1" result="effect1_foregroundBlur_31_533"/>
        </filter>
      </defs>
    </Svg>
  )
}
