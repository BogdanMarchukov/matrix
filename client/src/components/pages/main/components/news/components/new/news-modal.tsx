import React, { useState } from "react";
import Liked from "../../../../../../atoms/liked/liked";
import { CloseIcon } from "../../../scores/svg/close";
import styles from "./news-modal.module.css";
import { gql } from "../../../../../../../__generated__";
import { useMutation, useQuery } from "@apollo/client";
import { useStore } from "zustand";
import { useUserStore } from "../../../../../../../common/store/userStore";

interface NewsModalProps {
  onClose: () => void;
  payload: string;
  img?: string | null;
  title: string;
  newsId: string
}

const LIKE = gql(/* GraphQl */ `
    mutation Like($newsId: UUID!) {
      newsLike {
        like(newsId: $newsId) {
          newsLikeId
        }
      }
    }
`);

const LIKE_COUNT = gql(/* GraphQl */ `
    query NewsLike($newsId: UUID!) {
      newsLike {
        findCountByNewsId(newsId: $newsId)
      }
    }
`);

const FIND_LIKE = gql(/* GraphQl */ `
  query NewsLikeFindByUserId($data: NewsLikeFindByUserId!) {
    newsLike {
      findByUserId(data: $data) {
        newsId
      }
    }
  }
`);

const NewsModal: React.FC<NewsModalProps> = ({ newsId, onClose, payload, img, title }) => {
  const userId = useUserStore((state) => state.userId);
  const [showBigHeart, setShowBigHeart] = useState(false);
  const [like, { loading, error, data }] = useMutation(LIKE, { variables: { newsId } });
  const { data: likeCount, refetch } = useQuery(LIKE_COUNT, { variables: { newsId }, skip: !newsId });
  const { data: likeData, refetch: refetchLike } = useQuery(FIND_LIKE, { variables: { data: { userId, newsId } }, skip: !userId });
  const liked = typeof likeData?.newsLike?.findByUserId?.newsId === 'string';

  const handleDoubleClick = () => {
    like()
      .then(() => refetch())
      .then(() => refetchLike());
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
        <p>{likeCount?.findCountByNewsId ?? 0}</p>
      </div>
    </div >
  );
};

export default NewsModal;

