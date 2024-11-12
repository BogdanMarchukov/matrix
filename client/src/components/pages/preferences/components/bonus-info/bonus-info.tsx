import classes from './bonus-info.module.css'
import {DEFAULT_BONUS_VALUE} from "../../../../../common/constants";

const {root, bonusInfo, bonusInfoAccent, bonusInfoValue, bonusInfoBold} = classes;

export const BonusInfo = () => {
  return (
    <div className={root}>
      <p className={bonusInfo}>Ты получаешь
        <span className={bonusInfoAccent}>{`+${DEFAULT_BONUS_VALUE} бонусов`}</span>
        за каждый заказ
      </p>
      <p className={bonusInfoValue}>
        У тебя нет завершенных заказов в KYMATICA
      </p>
      <p className={bonusInfoBold}>
        Закажи и получи еще 3 товара, чтобы получить статус «Модник».
      </p>
    </div>
  )
}