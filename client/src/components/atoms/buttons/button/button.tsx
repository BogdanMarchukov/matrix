import React, {CSSProperties, PropsWithChildren} from "react";
import classes from './button.module.css';

interface ButtonProps extends PropsWithChildren {
  onClick?: () => void;
  variant?: 'text' | 'primary' | 'secondary';
  style?: CSSProperties;
  fullWidth?: boolean;
}

const buttonClassNames = {
  text: 'textButton',
  primary: 'primaryButton',
  secondary: 'secondaryButton',
}

export const Button = ({children, fullWidth, style, onClick, variant = 'primary'}: ButtonProps) => {
  return (
    <button
      className={`${classes.root} ${classes[buttonClassNames[variant]]}`}
      onClick={onClick}
      style={{...style, ...(fullWidth && ({width: '100%'}))}}
    >
      {children}
    </button>
  )
}