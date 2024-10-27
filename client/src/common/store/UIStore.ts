import { create } from "zustand";

type UserState = {
  isDev: boolean;
};

export const useUIStore = create<UserState>(() => ({
  isDev: process.env.NODE_ENV === 'development',
}));
