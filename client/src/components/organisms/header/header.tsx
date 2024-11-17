import {useCallback} from "react";
import classes from "./header.module.css";
import {useLocation, useNavigate} from "react-router-dom";
import {BACK_INDEX, PATHS} from "../../../common/constants";
import MenuIcon from "./svg/menu-icon";
import {IconButton} from "../../atoms/buttons/icon-button";
import {Button} from "../../atoms/buttons/button";
import {UserBar} from "./components/user-bar";
import {motion, useTransform, useScroll} from "motion/react"

export const Header = () => {
  const navigate = useNavigate();
  const location = useLocation();

  const isHomePage = location.pathname === PATHS.HOME

  const onMenuBtnClick = useCallback(() => {
    if (isHomePage) navigate(PATHS.PREFERENCES);
    else navigate(PATHS.HOME)
  }, [isHomePage, navigate]);

  const {scrollY} = useScroll();

  const offsetY = [0, 100];

  const opacity = useTransform(scrollY, offsetY, [1, 0])
  const marginTop = useTransform(scrollY, offsetY, [0, -34])

  return (
    <header className={classes.root}>
      <div className={classes.main}>
        <div className={classes.item}>
          {!isHomePage && <Button variant="text" onClick={() => navigate(BACK_INDEX)}>Назад</Button>}
        </div>
        <div className={classes.item}>
          <div className={classes.title}>KYMATICA</div>
          <div className={classes.subtitle}>бот</div>
        </div>
        <div className={classes.item}>
          <div className={classes.icon}>
            {isHomePage && <IconButton onClick={onMenuBtnClick} size="medium"><MenuIcon/></IconButton>}
          </div>
        </div>
      </div>
      <motion.div style={{opacity, marginTop, zIndex: 998, position: 'relative'}}><UserBar/></motion.div>
    </header>
  );
}
