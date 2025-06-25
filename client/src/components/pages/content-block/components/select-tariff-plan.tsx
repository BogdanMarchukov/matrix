import { Currency } from '../../../atoms/currency';
import { CurrencySvg } from '../svg/currency';
import classes from './select-tariff-plan.module.css'

const { pointsScreen, pointsCard, pointsValue, pointsLabel, pointsCurrency, infoText, highlight, history, historyTitle, historyEmpty, orderButton } = classes;

export const SelectTariffPlan = () => {
  return (
    <div className={pointsScreen}>
      <div className={pointsCard}>
        <h3 className={pointsLabel}>Полный доступ</h3>
        <div className={pointsValue}>99 <span>₽/мес</span></div>
      </div>

      <div className={infoText}>
        <p>
          Дальше ты будешь получать <span className={highlight}>+50 баллов</span> за каждый завершённый заказ. Подробности в профиле.
        </p>
        <p>
          При оформлении заказа ты можешь потратить накопленные баллы
          <br />
          <span className={highlight}>1 балл = 1 ₽</span>
        </p>
      </div>

      <div className={history}>
        <div className={historyTitle}>История начислений баллов</div>
        <div className={historyEmpty}>Пока что у тебя нет истории начислений баллов</div>
      </div>

      <button className={orderButton}>
        Подписаться
      </button>
    </div >
  );
}

