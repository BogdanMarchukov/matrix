import "./App.css";
import {
  ApolloClient,
  InMemoryCache,
  createHttpLink,
  ApolloProvider,
} from "@apollo/client";
import {setContext} from "@apollo/client/link/context";
import {RouterProvider, createBrowserRouter} from 'react-router-dom';
import {ROUTES} from "./common/routes";

declare global {
  interface Window {
    Telegram: any;
  }
}

const httpLink = createHttpLink({
  uri: "http://localhost:5000/gql",
});

const authLink = setContext((_, {headers}) => {
  const token = localStorage.getItem("jwt");
  return {
    headers: {
      ...headers,
      ["x-api-key"]: "secret-api-key",
      Authorization: token || "",
    },
  };
});

const client = new ApolloClient({
  link: authLink.concat(httpLink),
  cache: new InMemoryCache(),
});

const router = createBrowserRouter(ROUTES);

export const App = () => {
  return (
    <ApolloProvider client={client}>
      <RouterProvider router={router}/>
    </ApolloProvider>
  );
}
