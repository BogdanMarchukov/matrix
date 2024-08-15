import { create } from "zustand";

type AuthState = {
  jwt: string | null;
};

type Actions = {
  setJwt: (jwt: string) => void;
};

export const useAuthStore = create<AuthState & Actions>((set) => ({
  jwt: null,
  setJwt: (jwt) => set((_) => ({ jwt })),
}));
