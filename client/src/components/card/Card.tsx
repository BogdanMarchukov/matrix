import React from "react";
import classes from "./card.module.css";
import wheel from "./png/wheel.png";

function Card() {
  return (
    <div className={classes.container}>
      <img src={wheel} />
      <h3>Основной калькулятор</h3>
      <span>100 ₽</span>
      <div className={classes.btn}>
        <p>Заказать</p>
      </div>
    </div>
  );
}

export default Card;
