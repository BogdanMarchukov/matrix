import {Avatar} from "../../atoms/avatar";
import {useUserStore} from "../../../common/store/userStore";
import {DEFAULT_USER_NAME} from "../../../common/constants";
import classes from './preferences.module.css'
import {Accordion} from "./components/accordion";
import {LinkPhone} from "./components/link-phone";
import {Delivery} from "./components/delivery";
import {Resale} from "./components/resale";
import {BonusCount} from "./components/bonus-count";
import {BonusInfo} from "./components/bonus-info";
import {motion} from "motion/react"

import {routerMotionVariants} from "../../../common/routerMotionVariants";

const {
  userNameBox,
  userName,
  settingsBox,
} = classes;

export const PreferencesPage = () => {
  const {userInfo} = useUserStore((state) => state);
  return (
    <motion.div initial="out" animate="in" exit="out" variants={routerMotionVariants}>
      <div className={userNameBox}>
        <Avatar loading={false} url={userInfo?.avatarUrl || (userInfo?.firstName || DEFAULT_USER_NAME)?.[0]}/>
        <p className={userName}>{userInfo?.firstName || DEFAULT_USER_NAME}</p>
        <div className={settingsBox}>
          <Accordion/>
          <BonusInfo/>
          <BonusCount/>
          <LinkPhone/>
          <Delivery/>
          <Resale/>
        </div>
      </div>
    </motion.div>
  );
}
