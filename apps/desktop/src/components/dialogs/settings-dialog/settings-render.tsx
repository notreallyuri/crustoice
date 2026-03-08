import { SettingsRoutes } from "./nav-hook";
import { AccountSettings } from "./tabs/account";
import { ProfileSettings } from "./tabs/profile";

type Props = {
  currentRoute: SettingsRoutes;
};

export function SettingsRender({ currentRoute }: Props) {
  switch (currentRoute) {
    case "Account":
      return <AccountSettings />;
    case "Profile":
      return <ProfileSettings />;
    case "Notifications":
      return <></>;
    case "Appearance":
      return <></>;
    case "Advanced":
      return <div>Advanced settings comming soon!</div>;
    default:
      return <div>Select a settings category</div>;
  }
}
