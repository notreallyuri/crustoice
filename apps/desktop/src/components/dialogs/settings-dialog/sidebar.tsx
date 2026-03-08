import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem
} from "@/components/ui/sidebar";
import type {
  SidebarData,
  SidebarItem
} from "@/components/dialogs/settings-dialog/nav-hook";
import { LogOut } from "lucide-react";
import { User } from "@/types";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { useNavigate } from "@tanstack/react-router";
import { useAppStore } from "@/store/app-store";

type Props = {
  user: User;
  data: SidebarData;
  currentRoute: SidebarItem;
  onRouteChange: (r: SidebarItem) => void;
};

export function SettingsSidebar({
  data,
  currentRoute,
  onRouteChange,
  user
}: Props) {
  const navigate = useNavigate();
  const logoutAction = useAppStore((s) => s.logout);

  const handleLogout = async () => {
    await logoutAction();
    navigate({ to: "/auth/login" });
  };

  return (
    <Sidebar className="hidden border-r md:flex" collapsible="none">
      <SidebarContent>
        <SidebarGroup>
          <SidebarGroupContent>
            <SidebarMenu>
              <SidebarMenuItem>
                <SidebarMenuButton
                  isActive={currentRoute.name === "Profile"}
                  onClick={() => onRouteChange(data.hidden[0])}
                  className="h-fit py-2 border cursor-pointer group data-[active=true]:bg-primary data-[active=true]:text-primary-foreground"
                >
                  <Avatar>
                    <AvatarFallback>
                      {user.profile.display_name.charAt(0)}
                    </AvatarFallback>
                    <AvatarImage src={user.profile.avatar_url || undefined} />
                  </Avatar>
                  <div className="flex flex-col">
                    <span>{user.profile.display_name}</span>
                    <span className="text-muted-foreground group-data-[active=true]:text-white text-xs group-hover:text-white">
                      Edit profile
                    </span>
                  </div>
                </SidebarMenuButton>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>

        {Object.entries(data)
          .filter(([category]) => category !== "hidden")
          .map(([category, items]) => (
            <SidebarGroup key={category} className="pt-0">
              <SidebarGroupLabel className="text-xs uppercase text-muted-foreground font-semibold">
                {category}
              </SidebarGroupLabel>
              <SidebarGroupContent>
                <SidebarMenu>
                  {items.map((item) => (
                    <SidebarMenuItem key={item.name}>
                      <SidebarMenuButton
                        className="cursor-pointer data-[active=true]:bg-primary data-[active=true]:text-primary-foreground"
                        isActive={item.name === currentRoute.name}
                        onClick={() => onRouteChange(item)}
                      >
                        <item.icon />
                        {item.name}
                      </SidebarMenuButton>
                    </SidebarMenuItem>
                  ))}
                </SidebarMenu>
              </SidebarGroupContent>
            </SidebarGroup>
          ))}

        <SidebarGroup>
          <SidebarGroupContent>
            <SidebarMenu>
              <SidebarMenuItem>
                <SidebarMenuButton
                  onClick={handleLogout}
                  className="text-destructive hover:text-destructive/80 cursor-pointer"
                >
                  <LogOut /> Log Out
                </SidebarMenuButton>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>
    </Sidebar>
  );
}
