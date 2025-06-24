import { useQuery } from '@apollo/client';
import { gql } from '../../../__generated__';
import { useUserStore } from '../../../common/store/userStore';
import classes from './content-block.module.css';
import matrixImg from './img/content-img.png';
import { useMemo } from 'react';
import { Puff } from 'react-loader-spinner';

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
            <>
              <div className={imageWrapper}>
                <img src={matrixImg} alt="Matrix Preview" className={image} />
              </div>
              <h1>Title</h1>
              <br />
              <p>{`Lorem ipsum dolor sit amet, consectetur adipiscing elit. In ut congue est. Etiam luctus consectetur lectus eget laoreet. Vivamus eu lectus non nisl pellentesque accumsan. Duis vestibulum purus sed nisl interdum malesuada. Sed molestie lectus mattis, ultricies tortor eget, malesuada eros. Donec malesuada blandit mi ac gravida. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed ex eros, aliquet a dignissim pretium, dapibus nec dolor. Maecenas a ante metus. Nulla a accumsan lacus.

        Integer lacinia sodales convallis. Nulla vulputate, nulla porttitor lacinia lobortis, ante nunc mattis magna, vitae lobortis elit ligula nec quam. Nam non ullamcorper nunc. In iaculis, turpis a venenatis rhoncus, ex lectus aliquet ligula, et iaculis libero turpis nec dui. Quisque velit libero, fringilla ac magna pellentesque, fermentum ornare ipsum. Sed iaculis tempor turpis eu lacinia. Suspendisse volutpat nunc mauris, vitae tempus metus mollis et. Curabitur lacinia molestie ligula id rhoncus. Donec quis dui scelerisque ante tincidunt vehicula non id ex. Donec feugiat accumsan ipsum eget viverra. Mauris a diam ac lorem suscipit auctor.

        Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Donec sollicitudin nisl hendrerit urna sollicitudin, nec rhoncus velit mattis. Integer suscipit pulvinar ex. Nullam purus odio, volutpat eu rhoncus vitae, convallis at ipsum. Praesent varius eros eu nisi laoreet, in pretium leo tempor. Sed vestibulum dapibus lacus ut semper. Donec nec volutpat sem. Cras nec facilisis velit, vel eleifend ex. Vestibulum suscipit leo id lorem aliquam, et pretium nisi pulvinar. Aenean sagittis lectus diam, nec auctor ex euismod vel. Duis fringilla, massa vitae mollis mollis, erat diam varius turpis, ut bibendum mi ex ut tellus. Proin ac ipsum odio. Nam iaculis, sapien eget sagittis dapibus, nunc erat condimentum diam, vel pharetra lacus neque ac quam. Pellentesque sit amet sem et nibh hendrerit facilisis eu ac nisl.

        Nulla vehicula laoreet massa elementum lobortis. Quisque nec leo vitae metus facilisis dignissim vitae non felis. Quisque elementum et erat ac tincidunt. Cras tristique mi ut lacinia tempor. In non scelerisque tortor. Vivamus a varius leo, a egestas tellus. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Vestibulum sed maximus enim. Donec ut faucibus est, non vestibulum nibh.

        Integer sit amet cursus erat. Nunc ultrices diam eu magna sagittis commodo. Proin consequat lectus fermentum nisi ullamcorper posuere. Ut faucibus, nibh sit amet pulvinar sagittis, enim lacus dignissim diam, quis rutrum arcu elit ac leo. Nullam non ornare elit, vestibulum tristique ex. Morbi nisl felis, elementum porta elit quis, laoreet vestibulum neque. Aenean purus nulla, pretium non neque in, tristique elementum quam. Pellentesque vestibulum, libero eleifend sagittis pulvinar, leo mauris maximus tellus, sed consequat magna justo euismod ex. Sed pellentesque, tellus ut dignissim vulputate, magna nisl lobortis purus, hendrerit congue nulla lorem in lectus. Morbi sit amet nibh in justo auctor gravida. Integer ultricies massa nulla, sed vehicula felis ornare et. Aenean maximus mauris vel lorem aliquet, quis dictum orci placerat. Donec ac interdum neque.`
              }</p>
            </>) : <p>плати бабки сука</p>}
        </>
      ) : <div className={loader} ><Puff {...loaderConfig} /></div>}
    </div>
  )
}
