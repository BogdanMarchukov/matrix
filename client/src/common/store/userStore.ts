import {create} from "zustand";

type AuthState = {
  jwt: string | null;
};

type UserInfoState = {
  firstName: string | null;
  avatarUrl: string | null;
  userId: string | null;
}

type UserState = {
  auth: AuthState;
  userInfo: UserInfoState;
};

type UserInfoActions = {
  firstName: UserInfoState["firstName"],
  avatarUrl: UserInfoState["avatarUrl"],
  userId: UserInfoState["userId"],
}

type Actions = {
  setJwt: (jwt: string) => void;
  setUserInfo: ({firstName, avatarUrl, userId}: UserInfoActions) => void;
};

export const useUserStore = create<UserState & Actions>((set) => ({
  auth: {
    jwt: null,
  },
  userInfo: {
    firstName: null,
    avatarUrl: null,
    userId: null,
  },
  setJwt: (jwt) => set((state) => ({...state, auth: {jwt}})),
  setUserInfo: (userInfo) => set((state) => ({
    ...state,
    userInfo: {...state.userInfo, ...userInfo}
  })),
}));
