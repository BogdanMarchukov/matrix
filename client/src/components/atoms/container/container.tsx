import classes from './container.module.css';
import { PropsWithChildren } from 'react';

export const Container = ({children}: PropsWithChildren) => {
  return (
    <div className={classes.root}>{children}</div>
  )
}