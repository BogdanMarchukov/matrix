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
  const { userId, setLastNotify, lastNotify } =
    useUserStore((state) => state);

  const popupRef = useRef<HTMLDivElement>(null);

  const { loading, data, setNotifyIsRead } = useNotify({
    userId,
    isRead: false,
    notifyType: NotifyType.Daly,
  });


  const notifies = useMemo(() => data?.notify?.findByUserId || [], [data])
  const { notifyId, payload, title } = useMemo(() => ({ ...lastNotify }), [lastNotify]);

  useEffect(() => {
    if (notifies.length) {
      setLastNotify(notifies[0])
    }
  }, [notifies, setLastNotify])

  useEffect(() => {
    const handleClickOutside = (event: TouchEvent) => {
      if (showNotify && popupRef?.current && !popupRef?.current?.contains(event.target as Node)) {
        setNotifyIsRead(notifyId);
        setShowNotify(false);
      }
    };

    document.addEventListener("touchstart", handleClickOutside);
    return () => {
      document.removeEventListener("touchstart", handleClickOutside);
    };
  }, [showNotify, notifyId, setNotifyIsRead]);

  const onMessageClick = useCallback(() => {
    if (notifyId && !showNotify) {
      setShowNotify(true)
    }
  }, [notifyId, showNotify])

  return (
    <div className={classes.root} onTouchStart={onMessageClick} ref={popupRef}>
      <CardIcon animate={loading || !!notifies?.length} />
      <p className={classes.title}>
        <AnimatedText animation={!!notifies?.length}>{loading ? 'загрузка' : 'тебе'}</AnimatedText>
        {!loading && <AnimatedText animation={!!notifies?.length}>послание</AnimatedText>}
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
