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
import { useUserStore } from "./common/store/userStore";

declare global {
  interface Window {
    Telegram: any;
  }
}

const httpLink = createHttpLink({
  uri: "https://6f19-217-61-23-85.ngrok-free.app/gql",
});

const authLink = setContext((_, { headers }) => {
  const jwt = useUserStore((state) => state.auth.jwt);
  return {
    headers: {
      ...headers,
      Authorization: jwt || "",
    },
  };
});
const client = new ApolloClient({
  link: httpLink.concat(authLink),
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
