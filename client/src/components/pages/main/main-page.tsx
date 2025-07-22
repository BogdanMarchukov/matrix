import { News } from "./components/news";
import { PriceCalc } from "./components/price-calc";
import { Products } from "./components/products";
import { Scores } from "./components/scores";
import { UserBar } from "./components/user-bar";
import classes from './main-page.module.css';

export const MainPage = () => {
  return (
    <div className={classes.rootBox}>
      <UserBar />
      <News />
      <Scores />
      <PriceCalc />
      <Products />
    </div>
  );
}
