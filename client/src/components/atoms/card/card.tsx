import { PropsWithChildren } from "react";
import classes from './card.module.css'

interface CardProps extends PropsWithChildren {
  variant?: 'primary' | 'secondary';
  className?: string;
  style?: React.CSSProperties;
}

export const Card = ({ children, variant = 'primary', className, style }: CardProps) => {
  const variantClass = variant === 'primary' ? classes.primary : classes.secondary;
  return <div className={`${classes.base} ${variantClass} ${className || ''}`} style={style}>{children}</div>;
};