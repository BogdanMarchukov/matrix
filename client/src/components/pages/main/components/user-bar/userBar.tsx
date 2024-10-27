import {useLogin} from "../../../../../common/hooks/useLogin";
import classes from "./userBar.module.css";
import Payload from "../../../../atoms/message/Payload";
import TelegramLogo from "./svg/telegramLogo";
import Message from "../../../../atoms/message/Message";
import NextButton from "./svg/nextButton";
import {ThreeDots} from "react-loader-spinner";
import {Avatar} from "../../../../atoms/avatar/Avatar";
import {DEFAULT_USER_NAME} from "../../../../../common/constants";

export const UserBar = () => {
  const {data, loading} = useLogin();

  const userFirstName = data?.user?.firstName || data?.user?.lastName || DEFAULT_USER_NAME;
  const avatarUrl = data?.user?.photoUrl || userFirstName[0];
  const loaderConfig = {height: 20, width: 30, color: 'var(--color-white)'};

  return (
    <>
      <div className={classes.container}>
        <div className={classes.item}>
          <Avatar loading={loading} url={avatarUrl}/>
          <div className={classes.usernameBox}>
            <div>
              <div className={classes.username}>
                {loading ? <ThreeDots {...loaderConfig} /> : userFirstName}
              </div>
            </div>
            <div className={classes.userStatus}>Новичок</div>
          </div>
          <div className={classes.nextButton}>
            <NextButton/>
          </div>
        </div>
        <Message/>
        <div className={classes.item}>
          <div className={classes.telegramLogo}>
            <TelegramLogo/>
          </div>
          <p className={classes.telegramText}>
            <span>login</span>
            <span className={classes.telegramTextAccent}>our Tlg - channal</span>
          </p>
        </div>
      </div>
      <Payload/>
    </>
  );
}
