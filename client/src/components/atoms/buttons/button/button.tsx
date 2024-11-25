import React, {CSSProperties, ButtonHTMLAttributes} from "react";
import classes from './button.module.css';

interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
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

export const Button = ({children, fullWidth, style, onClick, variant = 'primary', ...rest}: ButtonProps) => {
  return (
    <button
      className={`${classes.root} ${classes[buttonClassNames[variant]]}`}
      onClick={onClick}
      style={{...style, ...(fullWidth && ({width: '100%'}))}}
      {...rest}
    >
      {children}
    </button>
  )
}