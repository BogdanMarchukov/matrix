import { useQuery } from "@apollo/client";
import 'swiper/css';
import { Swiper, SwiperSlide } from 'swiper/react';
import { gql } from "../../../../../__generated__";
import { useUserStore } from "../../../../../common/store/userStore";
import { Product } from "./components/product";
import classes from "./products.module.css";
import { useMemo } from "react";

const { root, swiperBox } = classes;

const OFFER_GET_ALL = gql(/* GraphQl */ `
  query FindMany {
    offer {
      findMany {
        img
        offerId
        title
        tariffIds
        description
      }
    }
  }
`);

export const Products = () => {
  const { userId } = useUserStore((state) => state);
  const { error, data, loading } = useQuery(OFFER_GET_ALL, { skip: !userId })
  const offer = useMemo(() => data?.offer?.findMany || [], [data])

  return (
    <div>
      <div className={root}>
        <h3>Все продукты</h3>
      </div>
      {offer?.length ?
        <div>
          <div className={swiperBox}>
            <Swiper
              slidesOffsetBefore={16}
              slidesPerView={2.5}
            >
              {offer?.length ? offer.map((offer) => (
                <SwiperSlide key={offer.offerId}>
                  <Product offer={offer} />
                </SwiperSlide>
              )) : null}
            </Swiper>
          </div>
        </div>
        : null
      }
    </div>
  )
    ;
}
