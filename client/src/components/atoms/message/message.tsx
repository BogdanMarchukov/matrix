import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { NotifyType } from "../../../__generated__/graphql";
import { useNotify } from "../../../common/hooks/useNotify";
import { useUserStore } from "../../../common/store/userStore";
import { AnimatedText } from "../animation-text/animation-text";
import { Card } from "../card";
import classes from "./message.module.css";
import { CardIcon } from "./svg/card-icon";

export const Message = () => {
  const [showNotify, setShowNotify] = useState(false)
  const { userId } =
    useUserStore((state) => state);

  const popupRef = useRef<HTMLDivElement>(null);

  const { loading, data, setNotifyIsRead } = useNotify({
    userId,
    notifyType: NotifyType.Daly,
  });


  const notifies = useMemo(() => data?.notify?.findByUserId || [], [data])
  const { notifyId, payload, title, isRead } = useMemo(() => ({ ...notifies[0] }), [notifies]);
  const isAnimal = !!(notifies?.length && !notifies[0].isRead);

  useEffect(() => {
    const handleClickOutside = (event: TouchEvent) => {
      if (showNotify && popupRef?.current && !popupRef?.current?.contains(event.target as Node)) {
        if (!isRead) {
          setNotifyIsRead(notifyId);
        }
        setShowNotify(false);
      }
    };

    document.addEventListener("touchstart", handleClickOutside);
    return () => {
      document.removeEventListener("touchstart", handleClickOutside);
    };
  }, [showNotify, notifyId, setNotifyIsRead, isRead]);

  const onMessageClick = useCallback(() => {
    if (notifyId && !showNotify) {
      setShowNotify(true)
    }
  }, [notifyId, showNotify])

  return (
    <div className={classes.root} onTouchStart={onMessageClick} ref={popupRef}>
      <CardIcon animate={loading || isAnimal} />
      <p className={classes.title}>
        <AnimatedText animation={isAnimal}>{loading ? 'загрузка' : 'тебе'}</AnimatedText>
        {!loading && <AnimatedText animation={isAnimal}>послание</AnimatedText>}
      </p>
      {showNotify && (
        <div className={classes.popup}>
          <Card>
            {`«${title}»
              ${payload}`}
          </Card>
        </div>
      )}
    </div>
  );
}
