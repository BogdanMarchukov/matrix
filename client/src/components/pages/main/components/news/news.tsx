import { useQuery } from "@apollo/client";
import { useMemo } from "react";
import { Swiper, SwiperSlide } from 'swiper/react';
import { gql } from "../../../../../__generated__";
import { useUserStore } from "../../../../../common/store/userStore";
import { New } from "./components/new";
import classes from './news.module.css';

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
