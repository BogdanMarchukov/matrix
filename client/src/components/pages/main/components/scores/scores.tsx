import {ChangeEvent, FocusEvent, useEffect, useState} from "react";
import classes from "./scores.module.css";
import ButtonSvg from "./svg/button";
import MiniBtnSvg from "./svg/miniBtn";
import {IconButton} from "../../../../atoms/buttons/icon-button";
import {Card} from "../../../../atoms/card";
import {Currency} from "../../../../atoms/currency";
import {DateInput} from "../../../../atoms/date-input";
import {useProfile} from "../../../profile/useProfile";

const {
  root,
  input,
  content,
  horizontalBox,
  verticalBox,
  titleBox,
  scoreTitle,
  sumBox,
  sum,
  friendsBonusTitle,
  friendsBonusText,
  orderTitle
} = classes;

export const Scores = () => {
  const {updateUserInfo, data, refetch} = useProfile();

  const [dateOfBirth, setDateOfBirth] = useState('');

  useEffect(() => {
    setDateOfBirth((prev) => prev !== data?.dateOfBirth ? data?.dateOfBirth : prev)
  }, [data]);

  useEffect(() => {
    if (dateOfBirth && data?.userInfoId && dateOfBirth !== data?.dateOfBirth) {
      updateUserInfo({
        variables: {
          data: {
            dateOfBirth
          },
          userInfoId: data?.userInfoId
        }
      }).then((res) => {
        if (res?.data?.userInfo?.updateOne?.userId) {
          refetch()
        }
      })
    }
  }, [dateOfBirth]);


  const onDateChange = (e: ChangeEvent<HTMLInputElement>) => {
    e.preventDefault();
    e.stopPropagation();
    const {value} = e.target;
    if (value) {
      setDateOfBirth(value);
    }
  }

  return (
    <div className={root}>
      <DateInput className={input} value={dateOfBirth} onChange={onDateChange} placeholder="Введите дату рождения"/>
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
