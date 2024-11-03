import React from "react";
import MiniBtnSvg from "../scores/svg/miniBtn";
import classes from "./price-calc.module.css";
import {Card} from "../../../../atoms/card";
import {IconButton} from "../../../../atoms/buttons/icon-button";

const { root, content, title, subtitle } = classes;

export const PriceCalc = () => {
  return (
    <div className={root}>
      <Card>
        <div className={content}>
          <div>
            <h3 className={title}>Расчитать стоимость</h3>
            <h4 className={subtitle}>Расчитать стоимость</h4>
          </div>
          <IconButton>
            <MiniBtnSvg/>
          </IconButton>
        </div>
      </Card>
    </div>
  );
}
