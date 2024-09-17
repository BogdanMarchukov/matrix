import { useEffect, useState } from "react";
import { useNotify } from "../../common/hooks/useNotify";
import { useUserStore } from "../../common/store/userStore";
import { NotifyType } from "../../__generated__/graphql";
import AnimatedText from "../animation-text/AnimationText";
import classes from "./message.module.css";
import Card from "./svg/card";

export default function () {
  const [animal, setAnimal] = useState(false);
  const { userId, setNotify, notify, setShowNotifyPayload, showNotifyPayload } =
    useUserStore((state) => state);
  const { loading, error, data } = useNotify({
    userId,
    isRead: false,
    notifyType: NotifyType.Daly,
  });

  useEffect(() => {
    if (loading === false && data?.notify?.findByUserId?.length) {
      setNotify(data.notify.findByUserId[0]);
    }
  }, [useNotify, loading, error, data]);

  useEffect(() => {
    if (notify?.isRead === false) {
      setAnimal(true);
    } else {
      setAnimal(false);
    }
  }, [notify]);

  return (
    <div
      onClick={() => setShowNotifyPayload(!showNotifyPayload)}
      className={classes.message}
    >
      <div className={classes.card}>
        <Card animate={animal} />
      </div>
      <div className={classes.messageTextBlock}>
        <span className={classes.messageTop}>
          <AnimatedText text={"тебе"} animation={animal}></AnimatedText>
        </span>
        <span className={classes.messageBottom}>
          <AnimatedText text={"послание"} animation={animal}></AnimatedText>
        </span>
      </div>
    </div>
  );
}
