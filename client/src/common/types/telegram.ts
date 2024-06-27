export interface User {
  id: number;
  is_bot?: boolean;
  first_name?: string;
  last_name?: string;
  username?: string;
  language_code?: string;
  is_premium?: boolean;
  photo_url?: string;
}

enum ChatType {
  GROUP = "group",
  SUPERGROUP = "supergroup",
  CHANNEL = "channel",
}

export interface Chat {
  id: number;
  type: ChatType;
  title: string;
  username?: string;
  photo_url?: string;
}
