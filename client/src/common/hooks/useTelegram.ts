import { Chat, User } from "../types/telegram";

export const useTelegram = () => {
  const initDataUnsafe = window?.Telegram?.WebApp?.initDataUnsafe as any;
  const user = initDataUnsafe.user as User;
  const chat = initDataUnsafe.chat as Chat;

  return {
    user,
    chat,
  };
};
