import {Scores} from "./components/scores";
import {PriceCalc} from "./components/price-calc";
import {Products} from "./components/products";
import {UserBar} from "./components/user-bar";
import classes from './main-page.module.css'
import {News} from "./components/news";

export const MainPage = () => {
  return (
    <div className={classes.rootBox}>
      <UserBar/>
      <News />
      <Scores/>
      <PriceCalc/>
      <Products/>
    </div>
  );
}
