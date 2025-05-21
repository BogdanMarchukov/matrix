import classes from "./avatar.module.css";
import { Puff } from "react-loader-spinner";

interface AvatarProps {
  loading: boolean;
  url: string;
}

export const Avatar = ({ loading, url }: AvatarProps) => {
  const loaderConfig = { height: 20, width: 30, color: "#ffffff" };

  return (
    <div className={classes.avatar}>
      {loading ? (
        <Puff {...loaderConfig} height={24} width={24} />
      ) : (<img src={url} alt="A" />)}
    </div>
  )
}
