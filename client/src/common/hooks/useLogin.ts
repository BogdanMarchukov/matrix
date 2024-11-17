import {useMutation} from "@apollo/client";
import {gql} from "../../__generated__";
import {useUIStore} from "../store/UIStore";
import {
  UserDevLoginMutation,
  UserDevLoginMutationVariables,
  UserLoginMutation,
  UserLoginMutationVariables
} from "../../__generated__/graphql";
import {useEffect, useLayoutEffect, useMemo} from "react";
import {useUserStore} from "../store/userStore";
import {DEFAULT_USER_NAME} from "../constants";

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
        }
      }
    }
  }
`);

const getVariables = (isDev: boolean): UserLoginMutationVariables | UserDevLoginMutationVariables => {
  if (isDev) {
    return {data: {userId: '233e2a86-fea6-43ff-bf22-17b8fbb9a088'}};
  } else {
    return {data: {initData: window?.Telegram?.WebApp?.initData}};
  }
}

export const useLogin = () => {
  const {isDev} = useUIStore((state) => state);
  const LOGIN_QUERY = isDev ? DEV_LOGIN : LOGIN;
  const {userInfo, setUserInfo} = useUserStore((state) => state);

  const [login, {loading, error, data}] = useMutation<
    UserLoginMutation | UserDevLoginMutation,
    UserLoginMutationVariables | UserDevLoginMutationVariables
  >(LOGIN_QUERY, {variables: getVariables(isDev)});

  useLayoutEffect(() => {
    if (!data) {
      login()
    }
  }, [login, data]);

  const userData = useMemo(() => isDev
    ? (data?.auth as UserDevLoginMutation['auth'])?.devLogin
    : (data?.auth as UserLoginMutation['auth'])?.login, [data, isDev]);

  useEffect(() => {
    const firstName = userData?.user?.firstName || userData?.user?.lastName || DEFAULT_USER_NAME;

    setUserInfo({
      userId: userData?.user?.userId || null,
      avatarUrl: userData?.user?.photoUrl || firstName[0] || null,
      firstName: firstName || null,
    });
  }, [userData]);

  return {
    userInfo,
    error,
    loading,
  };
};
