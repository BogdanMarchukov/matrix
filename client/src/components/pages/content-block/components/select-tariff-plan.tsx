import { useQuery } from '@apollo/client';
import { gql } from '../../../../__generated__';
import classes from './select-tariff-plan.module.css';
import { Puff } from 'react-loader-spinner';
import { useMemo } from 'react';

const { pointsScreen, pointsCard, pointsValue, pointsLabel, infoText, highlight, history, historyTitle, historyEmpty, orderButton } = classes;

const GET_TARIFF_PLAN = gql(/* GraphQl */ `
  query FindTariffPlanByOffer($offerId: UUID!) {
    offer {
      findById(offerId: $offerId) {
        tariffs {
          price
          tariffPlanId
          title
          description
          expiryDays
        }
        offerId
      }
    }
  }
`
)

type SelectTariffProps = {
  offerId: string | null
}

export const SelectTariffPlan = ({ offerId }: SelectTariffProps) => {
  const loaderConfig = { height: 80, width: 80, color: "#ffffff" };
  const { data, loading } = useQuery(GET_TARIFF_PLAN, {
    variables: {
      offerId
    },
    skip: !offerId
  })
  const tariffPlan = useMemo(() => data?.offer?.findById?.tariffs?.sort((a, b) => b.price - a.price)?.[0], [data]);

  return (
    <div className={pointsScreen}>
      {loading || !tariffPlan ? <Puff {...loaderConfig} /> : (
        <>
          <div className={pointsCard}>
            <h3 className={pointsLabel}>{tariffPlan.title}</h3>
            <div className={pointsValue}>{tariffPlan.price} <span>₽/мес</span></div>
          </div>

          <div className={infoText}>
            <p>
              {tariffPlan.description}
            </p>
          </div>
          <button className={orderButton}>
            Подписаться
          </button>
        </>
      )
      }
    </div >
  );
}

