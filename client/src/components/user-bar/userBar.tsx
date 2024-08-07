import { useEffect } from "react";
import { useLogin } from "../../common/hooks/useLogin";
import useTelegramInitData from "../../common/hooks/useTelegramInitData";
import Card from "./svg/card";
import NextButton from "./svg/nextButton";
import TelegramLogo from "./svg/telegramLogo";
import classes from "./userBar.module.css";

function UserBar() {
  const init = useTelegramInitData();
  const { login, loading, error, data } = useLogin();

  useEffect(() => {
    login();
  }, [login, init]);

  if (loading) return "....loading";
  if (error && init) return `error ${JSON.stringify(error)}`;

  return (
    <div className={classes.container}>
      <div className={classes.leftItem}>
        <div className={classes.avatar}></div>
        <div>
          <div>
            <div className={classes.username}>{data?.auth?.login?.jwt}</div>
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
          <span className={classes.tgContentTop}>login</span>
          <span className={classes.tgContentBottom}>our Tlg - channal</span>
        </div>
      </div>
    </div>
  );
}

export default UserBar;
