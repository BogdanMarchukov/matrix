import {ButtonOutlined} from "../../../../atoms/outlined-button/outlined-button";
import classes from './delivery.module.css'

const {root, title, text, flexBox, param, accentParam} = classes;
export const Delivery = () => {
  return (
    <div className={root}>
      <p className={title}>Данные доставки</p>
      <p className={text}>Чтобы не вводить каждый раз после заказа. Но если нужно будет заказать какой-то заказ в другое
        место - не проблема. </p>
      <div className={flexBox}>
        <p className={param}>ФИО получателя</p>
        <p className={accentParam}>Не указан</p>
      </div>
      <div className={flexBox} style={{ marginBottom: 'var(--spaces-mdm)'}}>
        <p className={param}>Телефон</p>
        <p className={accentParam}>Не указан</p>
      </div>
      <ButtonOutlined fullWidth>Редактировать</ButtonOutlined>
    </div>
  )
}