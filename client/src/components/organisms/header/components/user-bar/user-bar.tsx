import {useLogin} from "../../../../../common/hooks/useLogin";
import classes from "./user-bar.module.css";
import TelegramLogo from "./svg/telegramLogo";
import {Message} from "../../../../atoms/message";
import NextButton from "./svg/nextButton";
import {ThreeDots} from "react-loader-spinner";
import {Avatar} from "../../../../atoms/avatar";
import {DEFAULT_USER_NAME, PATHS} from "../../../../../common/constants";
import {useNavigate} from "react-router-dom";

const {root, item, usernameBox, username, userStatus, telegramLogo, telegramText, telegramTextAccent} = classes;

export const UserBar = () => {
  const navigate = useNavigate();
  const {userInfo, loading} = useLogin();

  const loaderConfig = {height: 20, width: 30, color: 'var(--color-white)'};

  return (
    <div className={root}>
      <div className={item} onClick={() => navigate(PATHS.PROFILE)}>
        <Avatar loading={loading} url={userInfo?.avatarUrl || ''}/>
        <div className={usernameBox}>
          <div>
            <div className={username}>
              {loading ? <ThreeDots {...loaderConfig} /> : userInfo?.firstName || ''}
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
