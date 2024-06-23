import SecondBlock from "../../components/cesond-block/SecondBlock";
import PriceCalc from "../../components/price-calc/priceCalc";
import Products from "../../components/products/Products";
import UserBar from "../../components/user-bar/userBar";

function MainPage() {
  return (
    <>
      <UserBar />
      <SecondBlock />
      <PriceCalc />
      <Products />
    </>
  );
}

export default MainPage;
