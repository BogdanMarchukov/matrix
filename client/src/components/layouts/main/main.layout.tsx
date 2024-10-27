import React from "react";
import classes from "./mainLayout.module.css";
import {Outlet} from 'react-router-dom';
import {Header} from "../../organisms/header/header";
import {IconButton} from "../../atoms/buttons/icon-button/icon-button";
import {HomeIcon} from "../icons/home-icon";

export const MainLayout = () => {
  return (
    <div className={classes.app}>
      <Header/>
      <div className={classes.main}>
        <Outlet/>
      </div>
      <div className={classes.homeButton}>
        <IconButton variant="primary" size="large">
          <HomeIcon />
        </IconButton>
      </div>
    </div>
  )
}
