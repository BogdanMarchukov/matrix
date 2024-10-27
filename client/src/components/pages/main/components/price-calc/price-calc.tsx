import React from "react";
import MiniBtnSvg from "../scores/svg/miniBtn";
import classes from "./price-calc.module.css";
import {Card} from "../../../../atoms/card/card";
import {IconButton} from "../../../../atoms/buttons/icon-button/icon-button";

export const PriceCalc = () => {
  return (
    <div className={classes.rootBox}>
      <Card>
        <div className={classes.contentBox}>
          <div>
            <h3>Расчитать стоимость</h3>
            <h4>Расчитать стоимость</h4>
          </div>
          <IconButton>
            <MiniBtnSvg/>
          </IconButton>
        </div>
      </Card>
    </div>
  );
}
