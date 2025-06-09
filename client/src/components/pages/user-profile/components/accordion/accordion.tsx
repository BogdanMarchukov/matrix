import { useMutation, useQuery } from '@apollo/client';
import { useUserStore } from '../../../../../common/store/userStore';
import { ButtonOutlined } from '../../../../atoms/outlined-button/outlined-button';
import TextField from '../../../../atoms/tixt-field/text-field';
import classes from './accordion.module.css';
import { useEffect, useState } from 'react';
import { UserInfoQuery, UserInfoUpdateInput } from '../../../../../__generated__/graphql';
import { gql } from '../../../../../__generated__';


const { root, accordion } = classes;

// const navigateList = [
//   {
//     icon: BonusIcon,
//     text: "Бонусы",
//     url: "Бонусы"
//   },
//   {
//     icon: TrainingIcon,
//     text: "Обучение",
//     url: "Обучение"
//   },
//   {
//     icon: TestsIcon,
//     text: "Тесты",
//     url: "Тесты"
//   },
//   {
//     icon: CommunityIcon,
//     text: "Сообщество",
//     url: "Сообщество"
//   },
//   {
//     icon: TeamIcon,
//     text: "Команда",
//     url: "Команда"
//   },
//   {
//     icon: NewsIcon,
//     text: "Новости",
//     url: "Новости"
//   },
//   {
//     icon: AboutIcon,
//     text: "О нас",
//     url: "О нас"
//   }
// ]
//
const GET_USER_INFO_BY_USER_ID = gql(/* GraphQl */
  `query GetUserInfoByUserId($userId: UUID!) {
    userInfo {
      fundByUserId(userId: $userId) {
        city
        dateOfBirth
        hourOfBirth
        minOfBirth
      }
    }
  }
`);


const UPDATE_USER_INFO = gql(/* GraphQl */
  `mutation UserInfoUpdateOne($userInfoId: UUID!, $data: UserInfoUpdateInput!) {
    userInfo {
      updateOne(userInfoId: $userInfoId, data: $data) {
        city
        dateOfBirth
        hourOfBirth
        minOfBirth
    }
  }
}`);

export const Accordion = () => {
  const { userId, userInfo: { userInfoId } } = useUserStore((state) => state);
  const [disable, setDisable] = useState(true);
  const { error, data, loading, refetch } = useQuery(GET_USER_INFO_BY_USER_ID, { variables: { userId }, skip: !userId });
  const [updateUserInfo] = useMutation(UPDATE_USER_INFO);
  const [city, setCity] = useState<string>('');
  const [dateOfBirth, setDateOfBirth] = useState<string>('');
  const [hourOfBirth, setHourOfBirth] = useState<string>('');
  const [minOfBirth, setMinOfBirth] = useState<string>('');

  useEffect(() => {
    if (loading === false && !error) {
      setDateOfBirth(data?.userInfo?.fundByUserId?.dateOfBirth || '');
      setCity(data?.userInfo?.fundByUserId?.city || '');
      setHourOfBirth(data?.userInfo?.fundByUserId?.hourOfBirth?.toString() || '');
      setMinOfBirth(data?.userInfo?.fundByUserId?.minOfBirth?.toString() || '');

    }
  }, [loading, error, setHourOfBirth, setMinOfBirth, setCity, setHourOfBirth]);

  const handelClick = () => {
    setDisable((prev) => !prev)
    if (!disable && (city || dateOfBirth || hourOfBirth || minOfBirth)) {
      const updateDate: UserInfoUpdateInput = {};
      if (city) {
        updateDate.city = city;
      }
      if (dateOfBirth) {
        updateDate.dateOfBirth = dateOfBirth;
      }
      if (hourOfBirth) {
        updateDate.hourOfBirth = parseInt(hourOfBirth);
      }
      if (minOfBirth) {
        updateDate.minOfBirth = parseInt(minOfBirth);
      }
      updateUserInfo({ variables: { userInfoId, data: updateDate } }).then(() => refetch())
    }
  }

  return (
    <div className={root}>
      <ul className={accordion}>
        {/* {navigateList.map((item) => <AccordionItem item={item} key={item.url}/>)} */}
        <TextField value={city} setValue={setCity} disable={disable} type='text' label={"Место рождения"} />
        <TextField value={dateOfBirth} setValue={setDateOfBirth} disable={disable} type='date' label={"Дата рождения"} />
        <TextField value={hourOfBirth} setValue={setHourOfBirth} disable={disable} max={24} type='number' label={'Час рождения'} />
        <TextField value={minOfBirth} setValue={setMinOfBirth} disable={disable} type='number' label={'Минуты рождения'} />
        <ButtonOutlined onClick={handelClick}>{disable ? 'Редактировать' : 'Сохранить'}</ButtonOutlined>
      </ul>
    </div>
  )
}
