import {Avatar} from "../../atoms/avatar";
import {useUserStore} from "../../../common/store/userStore";
import {DEFAULT_USER_NAME} from "../../../common/constants";
import classes from './user-profile.module.css'
import {Accordion} from "./components/accordion";
import {LinkPhone} from "./components/link-phone";
import {Delivery} from "./components/delivery";
import {Resale} from "./components/resale";
import {BonusCount} from "./components/bonus-count";
import {BonusInfo} from "./components/bonus-info";

const {
  userNameBox,
  userName,
  settingsBox,
} = classes;

export const UserProfilePage = () => {
  const {avatarUrl, firstName} = useUserStore((state) => state);
  return (
    <>
      <div className={userNameBox}>
        <Avatar loading={false} url={avatarUrl || (firstName || DEFAULT_USER_NAME)?.[0]}/>
        <p className={userName}>{firstName || DEFAULT_USER_NAME}</p>
        <div className={settingsBox}>
          <Accordion/>
          <BonusInfo/>
          <BonusCount/>
          <LinkPhone/>
          <Delivery/>
          <Resale/>
        </div>
      </div>
    </>
  );
}
