import { useEffect, useLayoutEffect } from "react";
import { Puff, ThreeDots } from "react-loader-spinner";
import { useLogin } from "../../common/hooks/useLogin";
import { useNotify } from "../../common/hooks/useNotify";
import { useUserStore } from "../../common/store/userStore";
import AnimatedText from "../animation-text/AnimationText";
import Card from "./svg/card";
import NextButton from "./svg/nextButton";
import TelegramLogo from "./svg/telegramLogo";
import classes from "./userBar.module.css";

function UserBar() {
  const { login, loading, error, data } = useLogin();
  const { setJwt, setFirstName, auth } = useUserStore((state) => state);
  const firstName =
    data?.auth?.login?.user?.firstName ||
    data?.auth?.login?.user?.lastName ||
    "имя ";
  const avatarUrl = data?.auth?.login?.user?.photoUrl || firstName[0];
  const loaderConfig = { height: 20, width: 30, color: "#ffffff	" };

  useLayoutEffect(() => {
    login();
  }, [login]);

  useEffect(() => {
    if (loading === false && data?.auth?.login?.jwt) {
      setJwt(data.auth.login.jwt);
      setFirstName(firstName);
    }
  }, [loading, data?.auth?.login?.jwt, setJwt, firstName, setFirstName]);

  return (
    <div className={classes.container}>
      <div className={classes.leftItem}>
        <div className={classes.avatar}>
          {loading ? (
            <Puff {...loaderConfig} height={24} width={24} />
          ) : (
            avatarUrl
          )}
        </div>
        <div>
          <div>
            <div className={classes.username}>
              {loading ? <ThreeDots {...loaderConfig} /> : firstName}
            </div>
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
          <Card animate={true} />
        </div>
        <div className={classes.messageTextBlock}>
          <span className={classes.messageTop}>
            <AnimatedText text={"тебе"} animation={true}></AnimatedText>
          </span>
          <span className={classes.messageBottom}>
            <AnimatedText text={"послание"} animation={true}></AnimatedText>
          </span>
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
