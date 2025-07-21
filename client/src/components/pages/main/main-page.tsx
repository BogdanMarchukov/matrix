import { Scores } from "./components/scores";
import { PriceCalc } from "./components/price-calc";
import { Products } from "./components/products";
import { UserBar } from "./components/user-bar";
import classes from './main-page.module.css'
import { News } from "./components/news";
import AstrologyChart from "../../astrology-chart/astrology-chart";

export const MainPage = () => {
  return (
    <div className={classes.rootBox}>
      <UserBar />
      <News />
      <Scores />
      <AstrologyChart />
      <PriceCalc />
      <Products />
    </div>
  );
}
