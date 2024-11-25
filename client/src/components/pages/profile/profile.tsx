import {useProfile} from "./useProfile";
import classes from './profile.module.css';

import {motion} from "motion/react"

import {routerMotionVariants} from "../../../common/routerMotionVariants";
import {Avatar} from "../../atoms/avatar";
import {Card} from "../../atoms/card";
import {Container} from "../../atoms/container/container";
import {TextInput} from "../../atoms/text-input";
import {DateInput} from "../../atoms/date-input";
import {ChangeEvent, SyntheticEvent, useCallback, useEffect, useState} from "react";
import {Select} from "../../atoms/select";
import {Button} from "../../atoms/buttons/button";

interface Option {
  id: number;
  value: string;
}

interface FormData {
  city: string;
  dateOfBirth: string;
  hourOfBirth: number;
  minOfBirth: number;
}


const {form, input, title, flexBox, userName, cardTitle, select} = classes;

export const ProfilePage = () => {
  const {data, loading, refetch, firstName, avatarUrl, updateUserInfo} = useProfile();

  const [formData, setFormData] = useState<FormData>({
    city: '',
    dateOfBirth: '',
    hourOfBirth: 0,
    minOfBirth: 0,
  });

  useEffect(() => {
    if (data) {
      setFormData({
        city: data.city || '',
        dateOfBirth: data.dateOfBirth || '',
        hourOfBirth: Number(data?.hourOfBirth),
        minOfBirth: Number(data?.minOfBirth),
      });
    }
  }, [data]);

  const handleChange = (e: ChangeEvent<HTMLInputElement | HTMLSelectElement>) => {
    const {name, value} = e.target;
    setFormData({...formData, [name]: value});
  };


  const getOptions = useCallback(
    (length: number): Option[] => {
      return (Array.from(Array(length).keys()).map((i) =>
        ({id: i, value: i < 10 ? `0${i}` : i})) || []) as Option[]
    }, [])

  const onSubmit = (e: SyntheticEvent) => {
    e.preventDefault();

    updateUserInfo({
      variables: {
        data: {...formData},
        userInfoId: data?.userInfoId
      }
    }).then((res) => {
      if (res?.data?.userInfo?.updateOne?.userId) {
        refetch()
      }
    })
  }

  return (
    <motion.div initial="out" animate="in" exit="out" variants={routerMotionVariants}>
      <Container>
        <form className={form} onSubmit={onSubmit}>
          <h2 className={title}>Профиль</h2>

          <Card>
            <div className={flexBox}>
              <Avatar loading={loading} url={avatarUrl} size="large"/>
              <p className={userName}>{firstName}</p>
            </div>
          </Card>

          <Card>
            <div className={flexBox}>
              <p className={cardTitle}>Город</p>
              <TextInput
                className={input}
                name="city"
                value={formData.city}
                onChange={handleChange}
                placeholder="Введите свой город"/>
            </div>
          </Card>

          <Card>
            <div className={flexBox}>
              <p className={cardTitle}>Дата</p>
              <DateInput
                className={input}
                name="dateOfBirth"
                value={formData.dateOfBirth}
                onChange={handleChange}
                placeholder="Введите дату рождения"/>
            </div>
          </Card>

          <Card>
            <div className={flexBox}>
              <p className={cardTitle}>Время</p>
              <div className={flexBox} style={{width: '100%'}}>
                <Select
                  className={select}
                  name="hourOfBirth"
                  value={formData.hourOfBirth}
                  onChange={handleChange}
                  options={getOptions(24)}
                  optionKey="value"
                />
                <Select
                  className={select}
                  name="minOfBirth"
                  value={formData.minOfBirth}
                  onChange={handleChange}
                  options={getOptions(60)}
                  optionKey="value"
                />
              </div>
            </div>
          </Card>

          <Button type="submit">
            Сохранить
          </Button>
        </form>
      </Container>
    </motion.div>
  )
}