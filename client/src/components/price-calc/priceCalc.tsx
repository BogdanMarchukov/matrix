import React from "react";
import MiniBtnSvg from "../cesond-block/svg/miniBtn";
import classes from "./price-calc.module.css";

function PriceCalc() {
  return (
    <div className={classes.wrapper}>
      <div className={classes.contentWrapper}>
        <h3>Расчитать стоимость</h3>
        <h4>Расчитать стоимость</h4>
        <div className={classes.btn}>
          <MiniBtnSvg />
        </div>
      </div>
    </div>
  );
}

export default PriceCalc;
