import {create} from "zustand";
import {Notify} from "../../__generated__/graphql";

type AuthState = {
  jwt: string | null;
};

type UserState = {
  auth: AuthState;
  firstName: string | null;
  avatarUrl: string | null;
  userId: string | null;
};

type Actions = {
  setJwt: (jwt: string) => void;
  setFirstName: (firstName: UserState["firstName"]) => void;
  setAvatarUrl: (avatar: string) => void;
  setUserId: (userId: UserState["userId"]) => void;
};

export const useUserStore = create<UserState & Actions>((set) => ({
  auth: {
    jwt: null,
  },
  firstName: null,
  avatarUrl: null,
  userId: null,
  setJwt: (jwt) => set((state) => ({...state, auth: {jwt}})),
  setFirstName: (firstName) => set((state) => ({...state, firstName})),
  setAvatarUrl: (avatarUrl) => set((state) => ({...state, avatarUrl})),
  setUserId: (userId) => set((state) => ({...state, userId})),
}));
