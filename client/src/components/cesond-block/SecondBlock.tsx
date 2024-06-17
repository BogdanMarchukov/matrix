import React from "react";
import classes from "./second.module.css";
import ButtonSvg from "./svg/button";
import MiniBtnSvg from "./svg/miniBtn";

function SecondBlock() {
  return (
    <div className={classes.container}>
      <input className={classes.input} placeholder="Введите дату рождения" />
      <div className={classes.selectWrap}>
        <div className={classes.selectLeft}>
          <div className={classes.leftPoints}>
            <div className={classes.leftPointsBtn}>
              <MiniBtnSvg />
            </div>
            <div className={classes.currency}>
              <p>₽</p>
              <div className={classes.sum}>
                <p >0</p>
              </div>
              <div className={classes.leftPointsText}>
                <p>Баллы</p>
              </div>
            </div>
            <div className={classes.inviteWrap}></div>
          </div>
        </div>
        <div className={classes.selectRight}>
          <div className={classes.buttonWrap}>
            <div className={classes.btn}>
              <ButtonSvg />
            </div>
          </div>
          <div className={classes.selectRightText}>
            <p>Выбрать и заказать</p>
          </div>
        </div>
      </div>
    </div>
  );
}

export default SecondBlock;
