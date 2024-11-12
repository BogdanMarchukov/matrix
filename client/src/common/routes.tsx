import {PATHS} from "./constants";
import {MainPage} from "../components/pages/main/main-page";
import {PreferencesPage} from "../components/pages/preferences";
import {MainLayout} from "../components/layouts/main/main.layout";
import {ProfilePage} from "../components/pages/profile/profile";

export const ROUTES = [
  {
    path: PATHS.HOME,
    element: <MainLayout/>,
    children: [
      {
        index: true,
        element: <MainPage/>,
      },
      {
        path: `${PATHS.HOME}${PATHS.PREFERENCES}`,
        element: <PreferencesPage/>,
      },
      {
        path: `${PATHS.HOME}${PATHS.PROFILE}`,
        element: <ProfilePage/>,
      },
    ],
  },
]