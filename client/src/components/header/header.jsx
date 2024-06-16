import React from "react";
import classes from "./header.module.css";
import MenuIcon from "./svg/menu-icon";

function Header() {
  return (
    <header className={classes.container}>
      <div className={classes.item}>Назад</div>
      <div className={classes.labileContainer}>
        <div className={classes.labile}>KYMATICA</div>
        <div className={classes.bot}>бот</div>
      </div>
      <div className={classes.item}>
        <div className={classes.icon}>
          <MenuIcon />
        </div>
      </div>
    </header>
  );
}

export default Header;
