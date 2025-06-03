import { create } from "zustand";

type AuthState = {
  jwt: string | null;
};

type UserInfo = {
  userInfoId: string | null;
  dateOfBirth?: Date | null;
}

type UserState = {
  auth: AuthState;
  firstName: string | null;
  avatarUrl: string | null;
  userId: string | null;
  userInfo: UserInfo;
};

type Actions = {
  setJwt: (jwt: string) => void;
  setFirstName: (firstName: UserState["firstName"]) => void;
  setAvatarUrl: (avatar: string) => void;
  setUserId: (userId: UserState["userId"]) => void;
  setUserInfo: (userInfo: UserInfo) => void;
};

export const useUserStore = create<UserState & Actions>((set) => ({
  auth: {
    jwt: null,
  },
  userInfo: {
    userInfoId: null,
  },
  firstName: null,
  avatarUrl: null,
  userId: null,
  lastNotify: null,
  setJwt: (jwt) => set((state) => ({ ...state, auth: { jwt } })),
  setFirstName: (firstName) => set((state) => ({ ...state, firstName })),
  setAvatarUrl: (avatarUrl) => set((state) => ({ ...state, avatarUrl })),
  setUserId: (userId) => set((state) => ({ ...state, userId })),
  setUserInfo: (userInfo) => set((state) => ({ ...state, userInfo }))
}));
