import { create } from "zustand";
import { GetAllNotifyQuery } from "../../__generated__/graphql";

type AuthState = {
  jwt: string | null;
};

type UserState = {
  auth: AuthState;
  firstName: string | null;
  avatarUrl: string | null;
  userId: string | null;
  lastNotify: GetAllNotifyQuery['notify']['findByUserId'][number] | null
};

type Actions = {
  setJwt: (jwt: string) => void;
  setFirstName: (firstName: UserState["firstName"]) => void;
  setAvatarUrl: (avatar: string) => void;
  setUserId: (userId: UserState["userId"]) => void;
  setLastNotify: (lastNotify: UserState['lastNotify']) => void;
};

export const useUserStore = create<UserState & Actions>((set) => ({
  auth: {
    jwt: null,
  },
  firstName: null,
  avatarUrl: null,
  userId: null,
  lastNotify: null,
  setJwt: (jwt) => set((state) => ({ ...state, auth: { jwt } })),
  setFirstName: (firstName) => set((state) => ({ ...state, firstName })),
  setAvatarUrl: (avatarUrl) => set((state) => ({ ...state, avatarUrl })),
  setUserId: (userId) => set((state) => ({ ...state, userId })),
  setLastNotify: (lastNotify) => set((state) => ({ ...state, lastNotify })),
}));
