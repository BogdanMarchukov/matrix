import { useQuery } from '@apollo/client';
import { gql } from '../../../__generated__';
import { useUserStore } from '../../../common/store/userStore';
import classes from './content-block.module.css';
import matrixImg from './img/content-img.png';
import { useMemo } from 'react';
import { Puff } from 'react-loader-spinner';
import { Content } from './components/content';

const { root, imageWrapper, image, loader } = classes;

export const ContentBlockPage = () => {
  const { currentOfferId, userId } = useUserStore((state) => state);

  const GET_OFFER = gql(/* GraphQl */ `
    query OfferFindById($offerId: UUID!) {
      offer {
        findById(offerId: $offerId) {
          img
          isActive
          offerId
          tariffIds
        }
      }
    }
`)

  const FIND_USER_TARIFF_PLAN = gql(/* GraphQl */ `
    query UserFindById($userId: UUID!) {
      user {
        findByPk(userId: $userId) {
          userId
          userTariffPlan {
            tariffPlanId
          }
        }
      }
    }
`)

  const { data, loading: offerLoading } = useQuery(GET_OFFER, {
    variables: {
      offerId: currentOfferId
    },
    skip: !currentOfferId
  });

  const { data: userTariffPlan, loading: userTariffPlanLoading } = useQuery(FIND_USER_TARIFF_PLAN, {
    variables: {
      userId
    },
    skip: !userId
  });
  const offer = useMemo(() => data?.offer?.findById, [data]);
  const tariffPlans = useMemo(() => userTariffPlan?.user?.findByPk?.userTariffPlan, [userTariffPlan]);
  const globalLoading = offerLoading || userTariffPlanLoading;
  const loaderConfig = { height: 80, width: 80, color: "#ffffff" };

  const checkTariffPlan = () => {
    if (!offer) {
      return false;
    };
    for (const id of offer.tariffIds as string[]) {
      if (tariffPlans?.some((plan) => plan.tariffPlanId === id)) {
        return true;
      }
    }
    return false;
  };

  return (
    <div className={root}>
      {globalLoading === false ? (
        <>
          {checkTariffPlan() ? (
            < Content />
          ) : <p>плати бабки сука</p>}
        </>
      ) : <div className={loader} ><Puff {...loaderConfig} /></div>
      }
    </div >
  )
}
