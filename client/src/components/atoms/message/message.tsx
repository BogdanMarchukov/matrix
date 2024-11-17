import {useCallback, useEffect, useMemo, useRef, useState} from "react";
import {useNotify} from "../../../common/hooks/useNotify";
import {useUserStore} from "../../../common/store/userStore";
import {NotifyType} from "../../../__generated__/graphql";
import {AnimatedText} from "../animation-text";
import classes from "./message.module.css";
import {CardIcon} from "./svg/card-icon";
import {Card} from "../card";
import {AnimatePresence, motion} from "motion/react"

export const Message = () => {
  const [showNotify, setShowNotify] = useState(false)
  const {userInfo} =
    useUserStore((state) => state);

  const popupRef = useRef<HTMLDivElement>(null);

  const {loading, data, setNotifyIsRead} = useNotify({
    userId: userInfo?.userId,
    isRead: false,
    notifyType: NotifyType.Daly,
  });

  const notifies = useMemo(() => data?.notify?.findByUserId || [], [data])
  const {notifyId, payload} = useMemo(() => ({...notifies?.[0]}), [notifies]);

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
    if (notifies?.length && !showNotify) {
      setShowNotify(true)
    }
  }, [notifies, showNotify])

  return (
    <div className={classes.root} onTouchStart={onMessageClick} ref={popupRef}>
      <CardIcon animate={loading || !!notifies?.length}/>
      <AnimatePresence>
        {showNotify && (
          <motion.div
            initial={{height: 0, opacity: 0}}
            animate={{height: 'auto', opacity: 1}}
            exit={{height: 0, opacity: 0}}
            style={{overflow: 'hidden'}}
            transition={{duration: 0.2}}
          >
            <div className={classes.popup}>
              <Card>
                {`«${payload}»`}
              </Card>
            </div>
          </motion.div>
        )}
      </AnimatePresence>
      <p className={classes.title}>
        <AnimatedText animation={!!notifies?.length}>{loading ? 'загрузка' : 'тебе'}</AnimatedText>
        {!loading && <AnimatedText animation={!!notifies?.length}>послание</AnimatedText>}
      </p>
    </div>
  );
}
