import { useEffect, useLayoutEffect, useState } from "react";
import { Puff, ThreeDots } from "react-loader-spinner";
import { useLogin } from "../../common/hooks/useLogin";
import { useUserStore } from "../../common/store/userStore";
import Message from "../message/Message";
import Payload from "../message/Payload";
import NextButton from "./svg/nextButton";
import TelegramLogo from "./svg/telegramLogo";
import classes from "./userBar.module.css";

function UserBar() {
  const { login, loading, data } = useLogin();
  const { setJwt, setFirstName, setUserId } = useUserStore((state) => state);
  const [showPayload, setShowPayload] = useState(false);

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
      setUserId(data.auth.login.user.userId);
      localStorage.setItem("jwt", data.auth.login.jwt);
    }
  }, [loading, data?.auth?.login?.jwt, setJwt, firstName, setFirstName]);

  return (
    <>
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
        <Message />
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
      <Payload />
    </>
  );
}

export default UserBar;
