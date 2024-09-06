import { useMutation } from "@apollo/client";
import { gql } from "../../__generated__/gql";

const LOGIN = gql(/* GraphQl */ `
  mutation userLogin ($data: LoginInput!) {
    auth {
      login(data: $data) {
        jwt
        user {
          userId
          firstName
          lastName
          photoUrl
        }
      }
    }
  }
`);

export const useLogin = () => {
  const initData = window?.Telegram?.WebApp?.initData;

  const [login, { loading, error, data }] = useMutation(LOGIN, {
    variables: { data: { initData } },
  });

  return {
    login,
    data,
    error,
    loading,
  };
};
