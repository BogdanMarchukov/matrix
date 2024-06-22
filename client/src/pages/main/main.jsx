import SecondBlock from "../../components/cesond-block/SecondBlock";
import PriceCalc from "../../components/price-calc/priceCalc";
import UserBar from "../../components/user-bar/userBar";

function MainPage() {
  return (
    <>
      <UserBar />
      <SecondBlock />
      <PriceCalc />
    </>
  );
}

export default MainPage;
