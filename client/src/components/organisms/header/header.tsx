import React from "react";
import classes from "./header.module.css";
import {useLocation, useNavigate} from "react-router-dom";
import {PATHS} from "../../../common/constants";
import MenuIcon from "./svg/menu-icon";
import {IconButton} from "../../atoms/buttons/icon-button/icon-button";
import {Button} from "../../atoms/buttons/button/button";

export const Header = () => {
  const navigate = useNavigate();
  const location = useLocation();
  const onMenuBtnClick = () => {
    if (location.pathname === PATHS.HOME) navigate(PATHS.PROFILE);
    else navigate(PATHS.HOME)
  };

  return (
    <header className={classes.root}>
      <div className={classes.item}>
        <Button variant="text">Назад</Button>
      </div>
      <div className={classes.item}>
        <div className={classes.title}>KYMATICA</div>
        <div className={classes.subtitle}>бот</div>
      </div>
      <div className={classes.item}>
        <div className={classes.icon}>
          <IconButton onClick={onMenuBtnClick} size="medium"><MenuIcon/></IconButton>
        </div>
      </div>
    </header>
  );
}
