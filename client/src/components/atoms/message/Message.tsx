import {useEffect, useState} from "react";
import {useNotify} from "../../../common/hooks/useNotify";
import {useUserStore} from "../../../common/store/userStore";
import {NotifyType} from "../../../__generated__/graphql";
import {AnimatedText} from "../animation-text/AnimationText";
import classes from "./message.module.css";
import Card from "./svg/card";

export default function () {
  const [animate, setAnimate] = useState(false);
  const {userId, setNotify, notify, setShowNotifyPayload, showNotifyPayload} =
    useUserStore((state) => state);
  const {loading, error, data} = useNotify({
    userId,
    isRead: false,
    notifyType: NotifyType.Daly,
  });

  useEffect(() => {
    if (!loading && data?.notify?.findByUserId?.length) {
      setNotify(data.notify.findByUserId[0]);
    }
  }, [useNotify, loading, error, data]);

  useEffect(() => {
    if (notify?.isRead === false) {
      setAnimate(true);
    } else {
      setAnimate(false);
    }
  }, [notify]);

  return (
    <div
      onTouchStart={() => setShowNotifyPayload(!showNotifyPayload)}
      className={classes.container}
    >
      <Card animate={animate}/>
      <div className={classes.messageBox}>
        <AnimatedText animation={animate}>тебе</AnimatedText>
        <AnimatedText animation={animate}>послание</AnimatedText>
      </div>
    </div>
  );
}
