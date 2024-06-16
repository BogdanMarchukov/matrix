import React from "react";
import classes from "./mainLayout.module.css";

function MainLayout({ children }: any) {
  return <div className={classes.fullScreen}>{children}</div>;
}

export default MainLayout;
