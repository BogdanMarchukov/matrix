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
  notify: Partial<Notify> | null;
  showNotifyPayload: boolean;
};

type Actions = {
  setJwt: (jwt: string) => void;
  setFirstName: (firstName: UserState["firstName"]) => void;
  setAvatarUrl: (avatar: string) => void;
  setUserId: (userId: UserState["userId"]) => void;
  setNotify: (notify: UserState["notify"]) => void;
  setShowNotifyPayload: (
    showNotifyPayload: UserState["showNotifyPayload"],
  ) => void;
};

export const useUserStore = create<UserState & Actions>((set) => ({
  auth: {
    jwt: null,
  },
  firstName: null,
  avatarUrl: null,
  userId: null,
  notify: null,
  showNotifyPayload: false,
  setJwt: (jwt) => set((state) => ({...state, auth: {jwt}})),
  setFirstName: (firstName) => set((state) => ({...state, firstName})),
  setAvatarUrl: (avatarUrl) => set((state) => ({...state, avatarUrl})),
  setUserId: (userId) => set((state) => ({...state, userId})),
  setNotify: (notify) => set((state) => ({...state, notify})),
  setShowNotifyPayload: (showNotifyPayload) =>
    set((state) => ({...state, showNotifyPayload})),
}));
