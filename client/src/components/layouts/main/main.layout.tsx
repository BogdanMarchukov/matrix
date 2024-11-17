import React from "react";
import classes from "./mainLayout.module.css";
import {Outlet} from 'react-router-dom';
import {Header} from "../../organisms/header";
import {IconButton} from "../../atoms/buttons/icon-button";
import {HomeIcon} from "../icons/home-icon";
import {AnimatePresence} from "motion/react";


export const MainLayout = () => {

  return (
    <div className={classes.app}>
      <Header/>
      <div className={classes.main}>
        <AnimatePresence>
          <Outlet/>
        </AnimatePresence>
      </div>
      <div className={classes.homeButton}>
        <IconButton variant="primary" size="large">
          <HomeIcon/>
        </IconButton>
      </div>
    </div>
  )
}
