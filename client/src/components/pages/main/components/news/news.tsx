import { useQuery } from "@apollo/client";
import { useCallback, useMemo, useState } from "react";
import { Swiper, SwiperSlide } from 'swiper/react';
import { gql } from "../../../../../__generated__";
import { useUserStore } from "../../../../../common/store/userStore";
import { New } from "./components/new";
import classes from './news.module.css';
import NewsModal from "./components/new/news-modal";
import { FindByUserIdQuery } from "../../../../../__generated__/graphql";

const GET_ALL_NEWS = gql(/* GraphQl */ `
  query FindByUserId($userId: UUID!) {
    userNews {
      findByUserId(userId: $userId) {
        img
        newsId
        payload
        title
        userNewsId
        news {
          countNewsLike
        }
      }
    }
  }
`);

export const News = () => {
  const userId = useUserStore((state) => state.userId);
  const [shwModal, setShwModal] = useState(false);
  const [currentNews, setCurrentNews] = useState<ReturnType<typeof selectNews>>({
    payload: '',
    img: '',
    title: '',
    newsId: '',
    news: { countNewsLike: 0 }
  })
  const { data } = useQuery(GET_ALL_NEWS, { variables: { userId }, skip: !userId });
  const news = useMemo(() => data?.userNews?.findByUserId || [], [data]);
  const selectNews = useCallback((news: FindByUserIdQuery['userNews']['findByUserId'][number]) => {
    return {
      payload: news.payload,
      img: news.img,
      title: news.title,
      news: { countNewsLike: news.news.countNewsLike },
      newsId: news.newsId
    }
  }, []);

  const handleClick = useCallback((news: FindByUserIdQuery['userNews']['findByUserId'][number]) => {
    setCurrentNews(selectNews(news));
    setShwModal(true);
  }, [selectNews])

  return (
    <div className={classes.root}>
      <div className={classes.content}>
        {
          shwModal ?
            <NewsModal
              countLikes={currentNews.news.countNewsLike}
              payload={currentNews.payload}
              img={currentNews.img}
              title={currentNews.title}
              onClose={() => setShwModal(false)}
              newsId={currentNews.newsId}
            />
            : null
        }
        <Swiper
          slidesPerView={3.5}
        >
          {news?.length ? news.map((newItem) => (
            <SwiperSlide key={newItem.userNewsId}>
              <New onClick={() => handleClick(newItem)} newItem={newItem} key={newItem.userNewsId} />
            </SwiperSlide>
          )) : null}
        </Swiper>
      </div>
    </div >
  )
}
