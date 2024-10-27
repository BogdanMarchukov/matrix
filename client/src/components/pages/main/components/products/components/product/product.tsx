import React from "react";
import classes from "./product.module.css";
import {Card} from "../../../../../../atoms/card/card";
import {Button} from "../../../../../../atoms/buttons/button/button";

export interface Product {
  id: string;
  img: string;
  title: string;
  price: number;
}

interface ProductProps {
  product: Product;
}

export const Product = ({product}: ProductProps) => {
  const {img, title, price} = product;
  return (
    <div className={classes.rootBox}>
      <Card>
        <div className={classes.contentBox}>
          <img className={classes.img} src={img} alt={title}/>
          <h3 className={classes.title}>{title}</h3>
          <p className={classes.price}>{`${price} ₽`}</p>
          <Button fullWidth>Заказать</Button>
        </div>
      </Card>
    </div>
  );
}
