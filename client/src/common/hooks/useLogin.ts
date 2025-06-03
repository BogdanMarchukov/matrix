import { useMutation } from "@apollo/client";
import { gql } from "../../__generated__";
import { useUIStore } from "../store/UIStore";
import {
  UserDevLoginMutation,
  UserDevLoginMutationVariables,
  UserLoginMutation,
  UserLoginMutationVariables
} from "../../__generated__/graphql";
import { useEffect, useLayoutEffect, useMemo } from "react";
import { useUserStore } from "../store/userStore";

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
            userInfo {
              dateOfBirth
              userInfoId
            }
        }
      }
    }
  }
`);

const DEV_LOGIN = gql(/* GraphQl */ `
  mutation userDevLogin ($data: DevLoginInput!) {
    auth {
      devLogin(data: $data) {
        jwt
        user {
          userId
          firstName
          lastName
          photoUrl
            userInfo {
              dateOfBirth
              userInfoId
            }
        }
      }
    }
  }
`);

const getVariables = (isDev: boolean): UserLoginMutationVariables | UserDevLoginMutationVariables => {
  if (isDev) {
    return { data: { userId: 'afc79ab8-6dc8-4eea-baa7-6a98f48ade92' } };
  } else {
    return { data: { initData: window?.Telegram?.WebApp?.initData } };
  }
}

export const useLogin = () => {
  const { isDev } = useUIStore((state) => state);
  const LOGIN_QUERY = isDev ? DEV_LOGIN : LOGIN;
  const { setUserId, setJwt, setUserInfo } = useUserStore((state) => state);

  const [login, { loading, error, data }] = useMutation<
    UserLoginMutation | UserDevLoginMutation,
    UserLoginMutationVariables | UserDevLoginMutationVariables
  >(LOGIN_QUERY, { variables: getVariables(isDev) });

  useLayoutEffect(() => {
    if (!data) {
      login()
    }
  }, [login, data]);

  const userData = useMemo(() => isDev
    ? (data?.auth as UserDevLoginMutation['auth'])?.devLogin
    : (data?.auth as UserLoginMutation['auth'])?.login, [data, isDev]);

  useEffect(() => {
    if (userData?.user?.userId) {
      setUserId(userData?.user?.userId);
    }
    if (userData?.jwt) {
      setJwt(userData.jwt);
      localStorage.setItem('jwt', userData.jwt);
    }
    if (userData?.user?.userInfo) {
      console.log('userInfo:', userData.user.userInfo);
      setUserInfo(userData.user.userInfo);
    }
  }, [userData, setUserId, setUserInfo, setJwt]);

  return {
    data: userData,
    error,
    loading,
  };
};
