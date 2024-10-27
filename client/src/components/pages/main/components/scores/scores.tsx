import React from "react";
import classes from "./scores.module.css";
import ButtonSvg from "./svg/button";
import MiniBtnSvg from "./svg/miniBtn";
import {IconButton} from "../../../../atoms/buttons/icon-button/icon-button";
import {Card} from "../../../../atoms/card/card";
import {Currency} from "../../../../atoms/currency/currency";
import {DateInput} from "../../../../atoms/date-input/date-input";

export const Scores = () => {
  return (
    <div className={classes.container}>
      <DateInput className={classes.input} placeholder="Введите дату рождения"/>
      <div className={classes.contentBox}>
        <div className={classes.horizontalBox}>
          <div className={classes.verticalBox}>
            <Card>
              <div className={classes.titleBox}>
                <p className={classes.scoreTitle}>Баллы</p>
                <IconButton><MiniBtnSvg/></IconButton>
              </div>
              <div className={classes.sumBox}>
                <Currency/>
                <p className={classes.sum}>0</p>
              </div>
            </Card>
            <Card>
              <div className={classes.titleBox}>
                <p className={classes.friendsBonusTitle}>Зови друзей</p>
                <IconButton><MiniBtnSvg/></IconButton>
              </div>
              <p className={classes.friendsBonusText}>
                Дарим по 500 ₽ - тебе и приведенному другу
              </p>
            </Card>
          </div>
          <Card variant="secondary">
            <p className={classes.orderTitle}>Выбрать и заказать</p>
            <IconButton variant="secondary" size="large">
              <ButtonSvg/>
            </IconButton>
          </Card>
        </div>
      </div>
    </div>
  );
}
