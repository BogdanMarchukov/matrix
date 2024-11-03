import React from "react";
import {Product} from "./components/product";
import classes from "./products.module.css";
import {Swiper, SwiperSlide} from 'swiper/react';
import 'swiper/css';
import main from "./png/main.png";
import compatibility from './png/compatibility.png';
import finance from './png/finance.png';
import destiny from './png/destiny.png';

const { root, swiperBox } = classes;

const productsList: Product[] = [
  {
    id: '0',
    img: main,
    title: 'Основной калькулятор',
    price: 100
  },
  {
    id: '1',
    img: compatibility,
    title: 'Калькулятор совместимости',
    price: 100
  },
  {
    id: '2',
    img: finance,
    title: 'Финансовый калькулятор',
    price: 100
  },
  {
    id: '3',
    img: destiny,
    title: 'Калькулятор предназначения',
    price: 100
  }
]

export const Products = () => {
  return (
    <div>
      <div className={root}>
        <h3>Все продукты</h3>
      </div>
      <div>
        <div className={swiperBox}>
          <Swiper
            slidesOffsetBefore={16}
            slidesPerView={2.5}
          >
            {productsList.map((product) => (
              <SwiperSlide key={product.id}>
                <Product product={product}/>
              </SwiperSlide>
            ))}
          </Swiper>
        </div>
      </div>
    </div>
  )
    ;
}
