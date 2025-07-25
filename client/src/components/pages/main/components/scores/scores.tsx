import { useState, useEffect } from "react";
import AstrologyChart from "../../../../astrology-chart/astrology-chart";
import { IconButton } from "../../../../atoms/buttons/icon-button";
import { Card } from "../../../../atoms/card";
import { Currency } from "../../../../atoms/currency";
import { DateInput } from "../../../../atoms/date-input";
import classes from "./scores.module.css";
import ButtonSvg from "./svg/button";
import MiniBtnSvg from "./svg/miniBtn";
import { CloseIcon } from "./svg/close";
import { Modal } from "../../../../atoms/modal/modal";

const { root, input, content, horizontalBox, verticalBox, titleBox, scoreTitle, sumBox, sum, friendsBonusTitle, friendsBonusText, orderTitle } = classes;

export const Scores = () => {
  const data = [[1, 6, 8, 1], [7, 14, 9, 2], [18, 17, 6], [19, 12], [10, 22, 5], [6, 19, 6, 10], [14, 6], [5, 14], [18, 17, 6], [9, 7, 7], [12],
  [5, 17, 2, 21, 13, 7, 1, 8, 16, 10, 22, 3, 19, 14, 6, 4,
    12, 11, 9, 18, 20, 15, 13, 7, 6, 22, 3, 1, 17, 14, 8, 5,
    9, 4, 19, 2, 18, 11, 10, 6, 3, 20, 21, 7, 13, 15, 12, 8,
    5, 1, 22, 19, 14, 17, 9, 6, 4, 10, 11, 2, 3, 7, 12, 16]];
  const [show, setShow] = useState(false);

  useEffect(() => {
    if (show) {
      document.body.style.overflow = 'hidden';
    } else {
      document.body.style.overflow = 'unset';
    }

    return () => {
      document.body.style.overflow = 'unset';
    };
  }, [show]);

  return (
    <div className={root}>
      <DateInput className={input} placeholder="Введите дату рождения" />
      <div className={content}>
        <div className={horizontalBox}>
          <div className={verticalBox}>
            <Card>
              <div className={titleBox}>
                <p className={scoreTitle}>Баллы</p>
                <IconButton><MiniBtnSvg /></IconButton>
              </div>
              <div className={sumBox}>
                <Currency />
                <p className={sum}>0</p>
              </div>
            </Card>
            <Card>
              <div className={titleBox}>
                <p className={friendsBonusTitle}>Зови друзей</p>
                <IconButton><MiniBtnSvg /></IconButton>
              </div>
              <p className={friendsBonusText}>
                Дарим по <span>500 ₽</span> - тебе и приведенному другу
              </p>
            </Card>
          </div>
          <Card variant="secondary">
            <p className={orderTitle}>Выбрать и заказать</p>
            <IconButton variant="secondary" size="large" onClick={() => setShow((prev) => !prev)}>
              <ButtonSvg />
            </IconButton>
          </Card>
        </div>
      </div>

      <Modal show={show} onClose={() => setShow(false)}>
        <AstrologyChart show={show} pointValue={data} />
      </Modal>
    </div>
  );
};
