import {useLogin} from "../../../../../common/hooks/useLogin";
import classes from "./user-bar.module.css";
import TelegramLogo from "./svg/telegramLogo";
import {Message} from "../../../../atoms/message";
import NextButton from "./svg/nextButton";
import {ThreeDots} from "react-loader-spinner";
import {Avatar} from "../../../../atoms/avatar";
import {DEFAULT_USER_NAME} from "../../../../../common/constants";

const {root, item, usernameBox, username, userStatus, telegramLogo, telegramText, telegramTextAccent} = classes;

export const UserBar = () => {
  const {data, loading} = useLogin();

  const userFirstName = data?.user?.firstName || data?.user?.lastName || DEFAULT_USER_NAME;
  const avatarUrl = data?.user?.photoUrl || userFirstName[0];
  const loaderConfig = {height: 20, width: 30, color: 'var(--color-white)'};

  return (
    <div className={root}>
      <div className={item}>
        <Avatar loading={loading} url={avatarUrl}/>
        <div className={usernameBox}>
          <div>
            <div className={username}>
              {loading ? <ThreeDots {...loaderConfig} /> : userFirstName}
            </div>
          </div>
          <div className={userStatus}>Новичок</div>
        </div>
        <div>
          <NextButton/>
        </div>
      </div>
      <Message/>
      <div className={item}>
        <div className={telegramLogo}>
          <TelegramLogo/>
        </div>
        <p className={telegramText}>
          <span>@teleganame</span>
          <span className={telegramTextAccent}>our Tlg - channal</span>
        </p>
      </div>
    </div>
  );
}
