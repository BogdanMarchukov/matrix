import classes from "./link-phone.module.css";
import {ButtonOutlined} from "../../../../atoms/outlined-button/outlined-button";

const {
  root,
  title,
  text
} = classes;

export const LinkPhone  = () => {
  return (
    <div className={root}>
      <p className={title}>Телефон</p>
      <p className={text}>Если привяжешь свой телефон, мы сможем перенести
        твои старые заказы в приложение</p>
      <ButtonOutlined fullWidth>Привязать телефон</ButtonOutlined>
    </div>
  )
}