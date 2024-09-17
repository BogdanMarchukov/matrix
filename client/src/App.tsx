import "./App.css";
import Header from "./components/header/header";
import MainLayout from "./components/layout/main/main.layout";
import MainPage from "./pages/main/main";
import {
  ApolloClient,
  InMemoryCache,
  createHttpLink,
  ApolloProvider,
} from "@apollo/client";
import { setContext } from "@apollo/client/link/context";

declare global {
  interface Window {
    Telegram: any;
  }
}

const httpLink = createHttpLink({
  uri: "https://3767-217-61-23-85.ngrok-free.app/gql",
});

const authLink = setContext((_, { headers }) => {
  const token = localStorage.getItem("jwt");
  return {
    headers: {
      ...headers,
      Authorization: token || "",
    },
  };
});
const client = new ApolloClient({
  link: authLink.concat(httpLink),
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
