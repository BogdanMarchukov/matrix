import React from "react";
import classes from "./scores.module.css";
import ButtonSvg from "./svg/button";
import MiniBtnSvg from "./svg/miniBtn";
import {IconButton} from "../../../../atoms/buttons/icon-button";
import {Card} from "../../../../atoms/card";
import {Currency} from "../../../../atoms/currency";
import {DateInput} from "../../../../atoms/date-input";

const {root, input, content, horizontalBox, verticalBox, titleBox, scoreTitle, sumBox, sum, friendsBonusTitle, friendsBonusText, orderTitle} = classes;

export const Scores = () => {
  return (
    <div className={root}>
      <DateInput className={input} placeholder="Введите дату рождения"/>
      <div className={content}>
        <div className={horizontalBox}>
          <div className={verticalBox}>
            <Card>
              <div className={titleBox}>
                <p className={scoreTitle}>Баллы</p>
                <IconButton><MiniBtnSvg/></IconButton>
              </div>
              <div className={sumBox}>
                <Currency/>
                <p className={sum}>0</p>
              </div>
            </Card>
            <Card>
              <div className={titleBox}>
                <p className={friendsBonusTitle}>Зови друзей</p>
                <IconButton><MiniBtnSvg/></IconButton>
              </div>
              <p className={friendsBonusText}>
                Дарим по <span>500 ₽</span> - тебе и приведенному другу
              </p>
            </Card>
          </div>
          <Card variant="secondary">
            <p className={orderTitle}>Выбрать и заказать</p>
            <IconButton variant="secondary" size="large">
              <ButtonSvg/>
            </IconButton>
          </Card>
        </div>
      </div>
    </div>
  );
}
