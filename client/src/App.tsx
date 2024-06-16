import "./App.css";
import Header from "./components/header/header";
import MainLayout from "./components/layout/main/main.layout";
import MainPage from "./pages/main/main";

function App() {
  return (
    <MainLayout>
      <Header/>
      <MainPage/>
    </MainLayout>
  );
}

export default App;
