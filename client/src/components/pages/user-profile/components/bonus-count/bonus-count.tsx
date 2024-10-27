import classes from './bonus-count.module.css'
import {DEFAULT_BONUS_VALUE} from "../../../../../common/constants";

const {root, flexJustifyBox, bonusCountTitle, bonusCountOrder, bonusCountBonus} = classes;

const bonusesCount = [
  {
    status: "Новичек",
    bonus: `+${DEFAULT_BONUS_VALUE}`,
    order: 0,
  },
  {
    status: "Освоенный",
    bonus: `+${DEFAULT_BONUS_VALUE}`,
    order: 3,
  },
  {
    status: "Профессионал",
    bonus: `+${DEFAULT_BONUS_VALUE}`,
    order: 10,
  },
  {
    status: "Профессионал +",
    bonus: `+${DEFAULT_BONUS_VALUE}`,
    order: 20,
  },
  {
    status: "Супер профессионал",
    bonus: `+${DEFAULT_BONUS_VALUE}`,
    order: 30,
  },
]

export const BonusCount = () => {
  return (
    <div className={root}>
      <div className={flexJustifyBox}>
        <div className={flexJustifyBox}>
          <p className={bonusCountTitle}>Статус</p>
          <p className={bonusCountTitle}>Балов за заказ</p>
        </div>
        <p className={`${bonusCountTitle} ${bonusCountOrder}`}>
          Заказов
        </p>
      </div>
      {bonusesCount.map((bonusCountItem) => (
        <div className={flexJustifyBox} key={bonusCountItem.status}>
          <div className={flexJustifyBox}>
            <p>{bonusCountItem.status}</p>
            <p className={bonusCountBonus}>{bonusCountItem.bonus}</p>
          </div>
          <p className={bonusCountOrder}>
            {bonusCountItem.order}
          </p>
        </div>
      ))}
    </div>
  )
}