import {CSSProperties, forwardRef, PropsWithChildren, Ref} from "react";
import classes from './outlined-button.module.css'

interface ButtonOutlinedProps extends PropsWithChildren {
  styles?: CSSProperties;
  fullWidth?: boolean;
  onClick?: () => void;
}

export const ButtonOutlined = forwardRef((props: ButtonOutlinedProps, ref: Ref<HTMLButtonElement>) => {
  const {children, fullWidth, styles, onClick} = props;
  return (
    <button
      ref={ref}
      className={classes.outlinedButton}
      onClick={onClick}
      style={{...styles, ...(fullWidth && ({width: '100%'}))}}
    >
      {children}
    </button>
  )
})
