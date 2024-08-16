import { create } from "zustand";

type AuthState = {
  jwt: string | null;
};

type UserState = {
  auth: AuthState;
  firstName: string | null;
};

type Actions = {
  setJwt: (jwt: string) => void;
  setFirstName: (firstName: string) => void;
};

export const useUserStore = create<UserState & Actions>((set) => ({
  auth: {
    jwt: null,
  },
  firstName: null,
  setJwt: (jwt) => set((state) => ({ ...state, auth: { jwt } })),
  setFirstName: (firstName) => set((state) => ({ ...state, firstName })),
}));
