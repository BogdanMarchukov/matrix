import React, {CSSProperties, PropsWithChildren, useMemo} from "react";
import classes from './icon-button.module.css';

interface IconButtonProps extends PropsWithChildren {
  onClick?: () => void;
  variant?: 'primary' | 'secondary';
  size?: 'small' | 'large' | 'medium';
  style?: CSSProperties;
}

const buttonClassNames = {
  primary: {
    small: 'primarySmallIconButton',
    medium: 'primaryMediumIconButton',
    large: 'primaryLargeIconButton'
  },
  secondary: {
    small: 'secondarySmallIconButton',
    medium: 'secondaryMediumIconButton',
    large: 'secondaryLargeIconButton'
  }
}

export const IconButton = ({children, style, onClick, variant = 'primary', size = 'small'}: IconButtonProps) => {
  return (
    <button className={classes[buttonClassNames[variant][size]]} onClick={onClick} style={style}>{children}</button>
  )
}