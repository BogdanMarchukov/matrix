import {PropsWithChildren} from "react";
import classes from './card.module.css'

interface CardProps extends PropsWithChildren {
  variant?: 'primary' | 'secondary';
}

export const Card = ({children, variant = 'primary'}: CardProps) => {
  return <div className={variant === 'primary' ? classes.primary : classes.secondary}>{children}</div>
}