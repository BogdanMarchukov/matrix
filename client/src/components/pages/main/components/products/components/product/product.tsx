import React from "react";
import classes from "./product.module.css";
import { Card } from "../../../../../../atoms/card";
import { Button } from "../../../../../../atoms/buttons/button";
import { FindManyQuery } from "../../../../../../../__generated__/graphql";

export interface Product {
  offerId: string;
  img: string;
  title: string;
  price: number;
}

interface ProductProps {
  offer: FindManyQuery['offer']['findMany'][number];
}

export const Product = ({ offer }: ProductProps) => {
  return (
    <div className={classes.rootBox}>
      <Card>
        <div className={classes.contentBox}>
          <img className={classes.img} src={offer.img!} alt={offer.title} />
          <h3 className={classes.title}>{offer.title}</h3>
          <Button fullWidth>Заказать</Button>
        </div>
      </Card>
    </div>
  );
}
