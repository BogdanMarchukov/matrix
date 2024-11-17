import classes from "./avatar.module.css";
import {Puff} from "react-loader-spinner";

interface AvatarProps {
  loading: boolean;
  url: string;
  size?: "normal" | "large";
}

export const Avatar = ({loading, url, size}: AvatarProps) => {
  const isLarge = size === 'large';
  const sizeValue = isLarge ? 64 : 24;
  const loaderConfig = {height: sizeValue * 85 / 100, width: 30 * 1.25, color: "#ffffff"};

  return (
    <div className={`${classes.avatar} ${classes[isLarge ? "large" : "normal"]}`}>
      {loading ? (
        <Puff {...loaderConfig} height={sizeValue} width={sizeValue}/>
      ) : (url)}
    </div>
  )
}