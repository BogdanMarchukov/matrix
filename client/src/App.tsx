import "./App.css";
import Header from "./components/header/header";
import MainLayout from "./components/layout/main/main.layout";
import MainPage from "./pages/main/main";

declare global {
  interface Window {
    Telegram: any;
  }
}

const user = window.Telegram.WebApp.initDataUnsafe?.user;

function App() {
  return (
    <MainLayout>
      <Header />
      <MainPage />
    </MainLayout>
  );
}

export default App;
