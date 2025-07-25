import { ReactNode, useEffect } from "react";
import { CloseIcon } from "../../pages/main/components/scores/svg/close";
import { IconButton } from "../buttons/icon-button";

interface ModalProps {
  children: ReactNode;
  onClose: () => void;
  show: boolean;
  maxWidth?: string;
  maxHeight?: string;
}

export const Modal = ({
  children,
  onClose,
  show,
  maxWidth = "100%",
  maxHeight = "90vh"
}: ModalProps) => {
  useEffect(() => {
    if (show) {
      document.body.style.overflow = 'hidden';
    } else {
      document.body.style.overflow = 'unset';
    }

    return () => {
      document.body.style.overflow = 'unset';
    };
  }, [show]);

  if (!show) return null;

  return (
    <div style={{
      position: 'fixed',
      top: 0,
      left: 0,
      right: 0,
      bottom: 0,
      backgroundColor: 'rgba(0, 0, 0, 0.5)',
      display: 'flex',
      justifyContent: 'center',
      alignItems: 'center',
      zIndex: 1000,
      padding: '16px',
      boxSizing: 'border-box'
    }}>
      <div style={{
        position: 'relative',
        width: '100%',
        maxWidth: maxWidth,
        maxHeight: maxHeight,
        overflow: 'auto',
        backgroundColor: 'black',
        borderRadius: '8px',
      }}>
        {/* Кнопка закрытия */}
        <div style={{
          position: 'absolute',
          top: '8px',
          right: '8px',
          zIndex: 1001
        }}>
          <IconButton
            onClick={onClose}
            style={{
              backgroundColor: 'transparent',
              padding: '4px'
            }}
            size="medium"
            aria-label="Close modal"
          >
            <CloseIcon />
          </IconButton>
        </div>

        {children}
      </div>
    </div>
  );
};
