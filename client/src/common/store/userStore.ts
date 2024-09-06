import { create } from "zustand";
import { Notify } from "../../__generated__/graphql";

type AuthState = {
  jwt: string | null;
};

type UserState = {
  auth: AuthState;
  firstName: string | null;
  userId: string | null;
  notify: Notify | null;
};

type Actions = {
  setJwt: (jwt: string) => void;
  setFirstName: (firstName: string) => void;
  setUserId: (userId: string) => void;
  setNotify: (notify: Notify) => void;
};

export const useUserStore = create<UserState & Actions>((set) => ({
  auth: {
    jwt: null,
  },
  firstName: null,
  userId: null,
  notify: null,
  setJwt: (jwt) => set((state) => ({ ...state, auth: { jwt } })),
  setFirstName: (firstName) => set((state) => ({ ...state, firstName })),
  setUserId: (userId) => set((state) => ({ ...state, userId })),
  setNotify: (notify) => set((state) => ({ ...state, notify })),
}));
