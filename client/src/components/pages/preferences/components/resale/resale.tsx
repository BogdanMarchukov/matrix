import {ButtonOutlined} from "../../../../atoms/outlined-button/outlined-button";
import classes from './resale.module.css'

const conditions = [
  "Скидка на комиссию за заказ: 50%",
  "Нет начислений баллов",
  "Нет прогресса по статусу"
]
const {root, title, accentText, conditionsBox, link, text} = classes;
export const Resale = () => {
  return (
    <div className={root}>
      <p className={title}>Ресейл-аккаунт</p>
      <p className={accentText}>Для оптовых покупателей. Оформи 50+ заказов и получи самый выгодный тариф</p>
      <div className={conditionsBox}>
        <a className={link}>Условия</a>
        {conditions.map((condition) => <p className={text} key={condition}>{condition}</p>)}
      </div>
      <ButtonOutlined fullWidth>Редактировать</ButtonOutlined>
    </div>
  )
}