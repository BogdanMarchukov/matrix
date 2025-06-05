import { DEFAULT_USER_NAME } from "../../../common/constants";
import { useUserStore } from "../../../common/store/userStore";
import { Avatar } from "../../atoms/avatar";
import { Accordion } from "./components/accordion";
import { BonusCount } from "./components/bonus-count";
import { BonusInfo } from "./components/bonus-info";
import { Delivery } from "./components/delivery";
import { LinkPhone } from "./components/link-phone";
import { Resale } from "./components/resale";
import classes from './user-profile.module.css';

const {
  userNameBox,
  userName,
  settingsBox,
} = classes;

export const UserProfilePage = () => {
  const { avatarUrl, firstName } = useUserStore((state) => state);
  return (
    <>
      <div className={userNameBox}>
        <Avatar loading={false} url={avatarUrl || (firstName || DEFAULT_USER_NAME)?.[0]} />
        <p className={userName}>{firstName || DEFAULT_USER_NAME}</p>
        <div className={settingsBox}>
          <Accordion />
          <BonusInfo />
          <BonusCount />
          <LinkPhone />
          <Delivery />
          <Resale />
        </div>
      </div>
    </>
  );
}
