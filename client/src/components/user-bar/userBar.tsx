import React from "react";
import { User } from "../../common/types/tegelram";
import Card from "./svg/card";
import NextButton from "./svg/nextButton";
import TelegramLogo from "./svg/telegramLogo";
import classes from "./userBar.module.css";

function UserBar() {
  const user = window?.Telegram?.WebApp?.initDataUnsafe?.user as User;
  const avatar = user?.photo_url;
  const userName =
    user?.username || user?.first_name || user?.last_name || "Имя пользователя";

  return (
    <div className={classes.container}>
      <div className={classes.leftItem}>
        <div className={classes.avatar}>
          <img src={avatar} alt="U" />
        </div>
        <div>
          <div>
            <div className={classes.username}>{userName}</div>
          </div>
          <div className={classes.userStatus}>
            <span>Новичок</span>
          </div>
        </div>
        <div className={classes.nextButton}>
          <NextButton />
        </div>
      </div>
      <div className={classes.message}>
        <div className={classes.card}>
          <Card />
        </div>
        <div className={classes.messageTextBlock}>
          <span className={classes.messageTop}>тебе</span>
          <span className={classes.messageBottom}>послание</span>
        </div>
      </div>
      <div className={classes.rightItem}>
        <div className={classes.telegramLogo}>
          <div className={classes.logoBg} />
          <div className={classes.logoWrap}>
            <TelegramLogo />
          </div>
        </div>
        <div className={classes.tgContentWrap}>
          <span className={classes.tgContentTop}>@teleganame</span>
          <span className={classes.tgContentBottom}>our Tlg - channal</span>
        </div>
      </div>
    </div>
  );
}

export default UserBar;
