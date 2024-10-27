import {Scores} from "./components/scores/scores";
import {PriceCalc} from "./components/price-calc/price-calc";
import {Products} from "./components/products/products";
import {UserBar} from "./components/user-bar/userBar";
import classes from './main-page.module.css'
import {News} from "./components/news/news";

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
