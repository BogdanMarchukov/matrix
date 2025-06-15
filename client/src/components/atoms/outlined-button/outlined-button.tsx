import { CSSProperties, forwardRef, PropsWithChildren, Ref, MouseEvent } from "react";
import classes from './outlined-button.module.css';

interface ButtonOutlinedProps extends PropsWithChildren {
  styles?: CSSProperties;
  fullWidth?: boolean;
  onClick?: () => void;
}

export const ButtonOutlined = forwardRef((props: ButtonOutlinedProps, ref: Ref<HTMLButtonElement>) => {
  const { children, fullWidth, styles, onClick } = props;

  const handleClick = (e: MouseEvent<HTMLButtonElement>) => {
    const button = e.currentTarget;
    const ripple = document.createElement("span");

    const rect = button.getBoundingClientRect();
    const size = Math.max(button.offsetWidth, button.offsetHeight);
    const x = e.clientX - rect.left - size / 2;
    const y = e.clientY - rect.top - size / 2;

    ripple.className = classes.ripple;
    ripple.style.width = ripple.style.height = `${size}px`;
    ripple.style.left = `${x}px`;
    ripple.style.top = `${y}px`;

    button.appendChild(ripple);

    setTimeout(() => {
      ripple.remove();
    }, 600);

    onClick?.();
  };
  return (
    <button
      ref={ref}
      className={classes.outlinedButton}
      onClick={handleClick}
      style={{ ...styles, ...(fullWidth && { width: '100%' }) }}
    >
      {children}
    </button>
  );
});

