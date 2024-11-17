import {CSSProperties, PropsWithChildren} from "react";
import classes from './card.module.css'

interface CardProps extends PropsWithChildren {
  variant?: 'primary' | 'secondary';
  style?: CSSProperties | undefined;
}

export const Card = ({children, variant = 'primary', ...rest}: CardProps) => {
  const variantClass = variant === 'primary' ? classes.primary : classes.secondary;
  return <div className={`${classes.base} ${variantClass}`} {...rest}>{children}</div>;
};