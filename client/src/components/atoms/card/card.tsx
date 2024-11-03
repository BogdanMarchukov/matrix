import {PropsWithChildren} from "react";
import classes from './card.module.css'

interface CardProps extends PropsWithChildren {
  variant?: 'primary' | 'secondary';
}

export const Card = ({ children, variant = 'primary' }: CardProps) => {
  const variantClass = variant === 'primary' ? classes.primary : classes.secondary;
  return <div className={`${classes.base} ${variantClass}`}>{children}</div>;
};