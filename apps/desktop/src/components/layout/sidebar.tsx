import { Link, useRouterState } from "@tanstack/react-router";
import { useAppStore } from "@/store/app-store";
import {
  Sidebar as ShadSidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarSeparator
} from "@/components/ui/sidebar";
import { SidebarFooter } from "./sidebar-footer";
import { SidebarHeader } from "./sidebar-header";
import { Hash, Users } from "lucide-react";
import { useState } from "react";
import { DialogCreateGuild } from "../dialog-create-guild";

export function Sidebar() {
  const [createDialogOpen, setCreateDialogOpen] = useState(false);

  const currentUser = useAppStore((s) => s.currentUser);
  const guilds = useAppStore((s) => s.guilds);

  const routerState = useRouterState();
  const pathname = routerState.location.pathname;

  const isHome = pathname === "/g/@me";

  const currentGuildId = pathname.split("/")[2];
  const activeGuild = guilds.find((g) => g.id === currentGuildId);

  console.log("Current user", currentUser);

  return (
    <ShadSidebar>
      <DialogCreateGuild
        isOpen={createDialogOpen}
        setIsOpen={setCreateDialogOpen}
      />
      <SidebarHeader
        activeGuild={activeGuild}
        guilds={guilds}
        isHome={isHome}
        setCreateDialogOpen={setCreateDialogOpen}
      />
      <SidebarContent>
        <SidebarGroup>
          <SidebarGroupContent>
            <SidebarMenu>
              {isHome ? (
                <SidebarMenuItem>
                  <SidebarMenuButton className="hover:bg-white/5 hover:text-white">
                    <Users className="text-muted-foreground" />
                    <span className="font-medium">Friends</span>
                  </SidebarMenuButton>
                  <SidebarSeparator className="my-1 border-white/10" />
                </SidebarMenuItem>
              ) : (
                activeGuild?.channels?.map((channel) => (
                  <SidebarMenuItem key={channel.id}>
                    <SidebarMenuButton
                      asChild
                      className="text-muted-foreground hover:bg-white/5 hover:text-white"
                    >
                      <Link
                        to={`/g/${activeGuild.id}/${channel.id}`}
                        className="w-full"
                      >
                        <Hash className="size-4 opacity-60" />
                        <span className="truncate font-medium">
                          {channel.name}
                        </span>
                      </Link>
                    </SidebarMenuButton>
                  </SidebarMenuItem>
                ))
              )}
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>
      <SidebarFooter currentUser={currentUser} />
    </ShadSidebar>
  );
}
