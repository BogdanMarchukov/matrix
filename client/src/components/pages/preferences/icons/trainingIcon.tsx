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

export const TrainingIcon = ({animate}: SvgProps) => {
  return (
    <Svg animate={animate} width={36} height={31} fill="none">
      <g filter="url(#filter0_f_31_524)">
        <path d="M29.157 12.4772V21.6049H30.6504V11.5963L17.3252 4L4 11.5963L17.3252 19.1925L29.157 12.4772Z"
              fill="#46E9FF"/>
        <path d="M8.36507 21.677L17.3252 26.7887L26.2853 21.677V15.2862L17.3252 20.3995L8.36507 15.2862V21.677Z"
              fill="#46E9FF"/>
        <path d="M32 24.8592L29.9605 22.5689L27.7786 24.7097L29.818 27L32 24.8592Z" fill="#46E9FF"/>
      </g>
      <g filter="url(#filter1_f_31_524)">
        <path d="M27.5801 13.6067V20.7205H28.8018V12.9202L17.9009 7L7 12.9202L17.9009 18.8403L27.5801 13.6067Z"
              fill="white"/>
        <path d="M10.5709 20.7767L17.9009 24.7605L25.2309 20.7767V15.796L17.9009 19.781L10.5709 15.796V20.7767Z"
              fill="white"/>
        <path d="M29.9058 23.2567L28.2374 21.4717L26.4524 23.1402L28.1209 24.9252L29.9058 23.2567Z" fill="white"/>
      </g>
      <path d="M27.5801 13.6067V20.7205H28.8018V12.9202L17.9009 7L7 12.9202L17.9009 18.8403L27.5801 13.6067Z"
            fill="white"/>
      <path d="M10.5709 20.7767L17.9009 24.7605L25.2309 20.7767V15.796L17.9009 19.781L10.5709 15.796V20.7767Z"
            fill="white"/>
      <path d="M29.9058 23.2567L28.2374 21.4717L26.4524 23.1402L28.1209 24.9252L29.9058 23.2567Z" fill="white"/>
      <defs>
        <filter id="filter0_f_31_524" x="0" y="0" width="36" height="31" filterUnits="userSpaceOnUse"
                colorInterpolationFilters="sRGB">
          <feFlood floodOpacity="0" result="BackgroundImageFix"/>
          <feBlend mode="normal" in="SourceGraphic" in2="BackgroundImageFix" result="shape"/>
          <feGaussianBlur stdDeviation="2" result="effect1_foregroundBlur_31_524"/>
        </filter>
        <filter id="filter1_f_31_524" x="5" y="5" width="26.9058" height="21.925" filterUnits="userSpaceOnUse"
                colorInterpolationFilters="sRGB">
          <feFlood floodOpacity="0" result="BackgroundImageFix"/>
          <feBlend mode="normal" in="SourceGraphic" in2="BackgroundImageFix" result="shape"/>
          <feGaussianBlur stdDeviation="1" result="effect1_foregroundBlur_31_524"/>
        </filter>
      </defs>
    </Svg>
  )
}
