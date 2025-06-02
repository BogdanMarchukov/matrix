import {
  ApolloClient,
  InMemoryCache,
  createHttpLink,
  ApolloProvider,
  split,
} from "@apollo/client";
import { GraphQLWsLink } from "@apollo/client/link/subscriptions";
import { createClient } from "graphql-ws";
import { setContext } from "@apollo/client/link/context";
import { getMainDefinition } from "@apollo/client/utilities";
import { RouterProvider, createBrowserRouter } from 'react-router-dom';
import { ROUTES } from "./common/routes";

declare global {
  interface Window {
    Telegram: any;
  }
}

const httpLink = createHttpLink({
  uri: "http://localhost:5000/gql",
});

const wsLink = new GraphQLWsLink(createClient({
  url: "ws://localhost:5000/gql",
  connectionParams: () => {
    const token = localStorage.getItem("jwt");
    return {
      "x-api-key": "secret-api-key",
      Authorization: token || "",
    };
  },
}));

const authLink = setContext((_, { headers }) => {
  const token = localStorage.getItem("jwt");
  return {
    headers: {
      ...headers,
      "x-api-key": "secret-api-key",
      Authorization: token || "",
    },
  };
});

const splitLink = split(
  ({ query }) => {
    console.log('query', query);
    const definition = getMainDefinition(query);
    return (
      definition.kind === "OperationDefinition" &&
      definition.operation === "subscription"
    );
  },
  wsLink,
  authLink.concat(httpLink)
);

const client = new ApolloClient({
  link: splitLink,
  cache: new InMemoryCache(),
});

const router = createBrowserRouter(ROUTES);

export const App = () => {
  return (
    <ApolloProvider client={client}>
      <RouterProvider router={router} />
    </ApolloProvider>
  );
};

