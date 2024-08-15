import "./App.css";
import Header from "./components/header/header";
import MainLayout from "./components/layout/main/main.layout";
import MainPage from "./pages/main/main";
import { ApolloClient, InMemoryCache, ApolloProvider } from "@apollo/client";

declare global {
  interface Window {
    Telegram: any;
  }
}

const client = new ApolloClient({
  uri: "https://f374-85-174-205-116.ngrok-free.app/gql",
  cache: new InMemoryCache(),
});

function App() {
  return (
    <ApolloProvider client={client}>
      <MainLayout>
        <Header />
        <MainPage />
      </MainLayout>
    </ApolloProvider>
  );
}

export default App;
