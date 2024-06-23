import React from "react";
import Card from "../card/Card";
import classes from "./products.module.css";

function Products() {
  return (
    <div className={classes.wrapper}>
      <h3>Все продукты</h3>
      <div className={classes.cardsWrapper}>
        <Card />
        <Card />
      </div>
    </div>
  );
}

export default Products;
