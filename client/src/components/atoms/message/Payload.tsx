import { useUserStore } from "../../../common/store/userStore";
import classes from "./payload.module.css";

export default function () {
  const { notify, showNotifyPayload } = useUserStore((state) => state);
  if (!notify || !showNotifyPayload) {
    return null;
  }
  return (
    <div className={classes.wrapper}>
      <div className={classes.union}>
        <span>«{notify?.payload}»</span>
      </div>
    </div>
  );
}
