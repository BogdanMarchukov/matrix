import React, { useState } from "react";
import Liked from "../../../../../../atoms/liked/liked";
import { CloseIcon } from "../../../scores/svg/close";
import styles from "./news-modal.module.css";

interface NewsModalProps {
  onClose: () => void;
  payload: string;
  img?: string | null;
  title: string;
}

const NewsModal: React.FC<NewsModalProps> = ({ onClose, payload, img, title }) => {
  const [liked, setLiked] = useState(false);
  const [showBigHeart, setShowBigHeart] = useState(false);

  const handleDoubleClick = () => {
    setLiked((prev) => !prev);
    setShowBigHeart(true);

    setTimeout(() => {
      setShowBigHeart(false);
    }, 800);
  };
  return (
    <div
      onDoubleClick={handleDoubleClick}
      style={{
        backgroundImage: "linear-gradient(to right top, #d16ba5, #c777b9, #ba83ca, #aa8fd8, #9a9ae1, #8aa7ec, #79b3f4, #69bff8, #52cffe, #41dfff, #46eefa, #5ffbf1)"
      }} className={styles.overlay}>
      {showBigHeart && <div className={styles.bigHeart}>❤️</div>}
      <div className={styles.closeButton} onClick={onClose}>
        <CloseIcon />
      </div>

      <div className={styles.imageWrapper}>
        <img
          src={img || ""}
          alt="Монета"
          className={styles.image}
        />
      </div>

      <div className={styles.textBlock}>
        <h2 className={styles.title}>{title}</h2>
        <p className={styles.description}>
          {payload}
        </p>
      </div>

      <div className={styles.footer}>
        <Liked liked={liked} onClick={handleDoubleClick} />
      </div>
    </div >
  );
};

export default NewsModal;

