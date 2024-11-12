import {Scores} from "./components/scores";
import {PriceCalc} from "./components/price-calc";
import {Products} from "./components/products";
import {UserBar} from "../../organisms/header/components/user-bar";
import classes from './main-page.module.css'
import {News} from "./components/news";

export const MainPage = () => {

  return (
    <div className={classes.rootBox}>
      <News />
      <Scores/>
      <PriceCalc/>
      <Products/>
      <div style={{ height: '300dvh' }} />
    </div>
  );
}
