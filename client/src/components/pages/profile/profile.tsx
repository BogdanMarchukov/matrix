import {useProfile} from "./useProfile";
import classes from './profile.module.css';

import {motion} from "motion/react"

import {routerMotionVariants} from "../../../common/routerMotionVariants";
import {Avatar} from "../../atoms/avatar";
import {Card} from "../../atoms/card";
import {Container} from "../../atoms/container/container";
import {TextInput} from "../../atoms/text-input";
import {DateInput} from "../../atoms/date-input";
import React, {useCallback} from "react";
import {Select} from "../../atoms/select";

interface Option {
  id: number;
  value: string;
}


const {input, title, flexBox, userName, cardTitle, select} = classes;

export const ProfilePage = () => {
  const {data, loading, firstName, avatarUrl} = useProfile()
  console.log('data', data, '>>>', avatarUrl)

  const getOptions = useCallback(
    (length: number): Option[] => {
      return (Array.from(Array(length).keys()).map((i) =>
        ({id: i, value: i < 10 ? `0${i}` : i})) || []) as Option[]
    }, [])

  return (
    <motion.div initial="out" animate="in" exit="out" variants={routerMotionVariants}>
      <Container>
        <h2 className={title}>Профиль</h2>

        <Card style={{marginBottom: 'var(--spaces-md)'}}>
          <div className={flexBox}>
            <Avatar loading={loading} url={avatarUrl} size="large"/>
            <p className={userName}>{firstName}</p>
          </div>
        </Card>

        <Card style={{marginBottom: 'var(--spaces-md)'}}>
          <div className={flexBox}>
            <p className={cardTitle}>Город</p>
            <TextInput className={input} placeholder="Введите свой город"/>
          </div>
        </Card>

        <Card style={{marginBottom: 'var(--spaces-md)'}}>
          <div className={flexBox}>
            <p className={cardTitle}>Дата</p>
            <DateInput className={input} placeholder="Введите дату рождения"/>
          </div>
        </Card>

        <Card style={{marginBottom: 'var(--spaces-md)'}}>
          <div className={flexBox}>
            <p className={cardTitle}>Время</p>
            <div className={flexBox} style={{width: '100%'}}>
              <Select className={select} options={getOptions(24)} optionKey="value"/>
              <Select className={select} options={getOptions(60)} optionKey="value"/>
            </div>
          </div>
        </Card>
      </Container>
    </motion.div>
  )
}