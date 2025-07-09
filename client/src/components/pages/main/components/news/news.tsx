import { useQuery } from "@apollo/client";
import { gql } from "../../../../../__generated__";
import { useUserStore } from "../../../../../common/store/userStore";
import { New } from "./components/new";
import classes from './news.module.css'
import newsFirst from './png/news-first.png';
import newsSecond from './png/news-second.png';
import newsThird from './png/news-third.png';
import { Swiper, SwiperSlide } from 'swiper/react';
import { useMemo } from "react";

const newsList = [
  {
    id: '1',
    title: 'Новост 1',
    img: newsFirst,
  },
  {
    id: '2',
    title: 'Новост 2',
    img: newsSecond,
  },
  {
    id: '3',
    title: 'Новост 3',
    img: newsThird,
  }
]

const GET_ALL_NEWS = gql(/* GraphQl */ `
  query FindByUserId($userId: UUID!) {
    userNews {
      findByUserId(userId: $userId) {
        img
        newsId
        payload
        title
        userNewsId
      }
    }
  }
`);


export const News = () => {
  const { userId } = useUserStore((state) => state);
  const { data } = useQuery(GET_ALL_NEWS, { variables: { userId }, skip: !userId });
  const news = useMemo(() => data?.userNews?.findByUserId || [], [data]);

  return (
    <div className={classes.root}>
      <div className={classes.content}>
        <Swiper
          slidesPerView={2.5}
        >
          {news?.length ? news.map((newItem) => (
            <SwiperSlide key={newItem.userNewsId}>
              <New newItem={newItem} key={newItem.userNewsId} />
            </SwiperSlide>
          )) : null}
        </Swiper>
      </div>
    </div >
  )
}
