import styles from './liked.module.css';

interface LikedProps {
  onClick: () => void;
  liked: boolean;
}

export default function Liked({ onClick, liked }: LikedProps) {

  return (
    <button
      className={styles.iconButton}
      onClick={onClick}
    >
      {liked ? '❤️' : '🤍'}
    </button>
  );
}

