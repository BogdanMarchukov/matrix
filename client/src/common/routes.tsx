import { PATHS } from "./constants";
import { MainPage } from "../components/pages/main/main-page";
import { UserProfilePage } from "../components/pages/user-profile/user-profile";
import { MainLayout } from "../components/layouts/main/main.layout";
import { ContentBlockPage } from "../components/pages/content-block";

export const ROUTES = [
  {
    path: PATHS.HOME,
    element: <MainLayout />,
    children: [
      {
        index: true,
        element: <MainPage />,
      },
      {
        path: `${PATHS.HOME}${PATHS.PROFILE}`,
        element: <UserProfilePage />,
      },
      {
        path: `${PATHS.HOME}${PATHS.CONTENT}`,
        element: <ContentBlockPage />,
      },
    ],
  },
]
