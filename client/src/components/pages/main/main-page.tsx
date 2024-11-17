import {Scores} from "./components/scores";
import {PriceCalc} from "./components/price-calc";
import {Products} from "./components/products";
import classes from './main-page.module.css'
import {News} from "./components/news";
import {motion} from "motion/react"
import {routerMotionVariants} from "../../../common/routerMotionVariants";

export const MainPage = () => {

  return (
    <motion.div initial="out" animate="in" exit="out" variants={routerMotionVariants}>
      <div className={classes.rootBox}>
        <News/>
        <Scores/>
        <PriceCalc/>
        <Products/>
      </div>
    </motion.div>
  );
}
