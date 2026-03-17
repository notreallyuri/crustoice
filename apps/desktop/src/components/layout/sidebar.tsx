import { Link, useRouterState, useParams } from "@tanstack/react-router";
import { useAppStore } from "@/store/app-store";
import {
  Sidebar as ShadSidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem
} from "@/components/ui/sidebar";
import { SidebarFooter } from "./sidebar-footer";
import { SidebarHeader, Tab } from "./sidebar-header";
import { ChevronDown, Hash, MessageSquareDashed, Users } from "lucide-react";
import { useState } from "react";
import { useCurrentUser } from "@/hooks/use-current-user";
import { Channel, ChannelCategory, Guild } from "@/types";
import { cn } from "@/lib/utils";
import { Avatar, AvatarBadge, AvatarFallback } from "../ui/avatar";
import { AvatarGif } from "../avatar-gif";
import { UserProfileCard } from "./profile-card";

type Props = {
  setSettingsDialogOpen: (open: boolean) => void;
};

type CategoryGroup = ChannelCategory & { channels: Channel[] };

function ChannelItem({
  channel,
  guildId,
  isActive
}: {
  channel: Channel;
  guildId: string;
  isActive: boolean;
}) {
  return (
    <SidebarMenuItem>
      <SidebarMenuButton
        isActive={isActive}
        className={cn(
          "group/channel text-muted-foreground hover:bg-white/5 hover:text-white transition-colors",
          isActive && "bg-white/10 text-white"
        )}
        render={
          <Link
            to="/g/$guildId/$channelId"
            params={{ guildId, channelId: channel.id }}
            className="w-full"
          >
            <Hash
              className={cn(
                "size-4 shrink-0 transition-colors",
                isActive
                  ? "text-white opacity-80"
                  : "opacity-40 group-hover/channel:opacity-60"
              )}
            />
            <span className="truncate font-medium">{channel.name}</span>
          </Link>
        }
      />
    </SidebarMenuItem>
  );
}

function CategoryGroup({
  category,
  guildId,
  activeChannelId
}: {
  category: CategoryGroup;
  guildId: string;
  activeChannelId?: string;
}) {
  const [collapsed, setCollapsed] = useState(false);

  return (
    <div className="mt-4 first:mt-0">
      <button
        type="button"
        onClick={() => setCollapsed((c) => !c)}
        className="group/cat flex w-full items-center gap-1 px-2 py-0.5 text-left"
      >
        <ChevronDown
          className={cn(
            "size-3 shrink-0 text-muted-foreground/60 transition-transform duration-150",
            collapsed && "-rotate-90"
          )}
        />
        <span className="truncate text-[11px] font-bold uppercase tracking-wider text-muted-foreground/60 group-hover/cat:text-muted-foreground transition-colors">
          {category.name}
        </span>
      </button>

      {!collapsed && (
        <SidebarMenu className="mt-0.5">
          {category.channels.length === 0 ? (
            <p className="px-4 py-1 text-xs text-muted-foreground/40 italic">
              No channels
            </p>
          ) : (
            category.channels.map((channel) => (
              <ChannelItem
                key={channel.id}
                channel={channel}
                guildId={guildId}
                isActive={channel.id === activeChannelId}
              />
            ))
          )}
        </SidebarMenu>
      )}
    </div>
  );
}

export function Sidebar({ setSettingsDialogOpen }: Props) {
  const [activeTab, setActiveTab] = useState<Tab>("channels");

  const currentUser = useCurrentUser();
  const guilds = useAppStore((s) => s.guilds);
  const routerState = useRouterState();

  const pathname = routerState.location.pathname;
  const isHome = pathname === "/g/@me";

  const params = useParams({ strict: false });
  const activeGuild = guilds.find((g) => g.id === params.guildId);

  const categorized: CategoryGroup[] = activeGuild
    ? activeGuild.categories
        .sort((a, b) => a.position - b.position)
        .map((cat) => ({
          ...cat,
          channels: activeGuild.channels
            .filter((ch) => ch.category_id === cat.id)
            .sort((a, b) => a.position - b.position)
        }))
    : [];

  const uncategorized: Channel[] = activeGuild
    ? activeGuild.channels
        .filter((ch) => ch.category_id === null)
        .sort((a, b) => a.position - b.position)
    : [];

  return (
    <ShadSidebar className="pt-0" variant="inset">
      <SidebarHeader
        activeGuild={activeGuild}
        guilds={guilds}
        isHome={isHome}
        activeTab={activeTab}
        setActiveTab={setActiveTab}
      />

      <SidebarContent>
        <SidebarGroup>
          <SidebarGroupContent>
            {isHome ? (
              <SidebarMenu>
                <SidebarMenuItem>
                  <SidebarMenuButton className="hover:bg-white/5 hover:text-white">
                    <Users className="size-4 text-muted-foreground" />
                    <span className="font-medium">Friends</span>
                  </SidebarMenuButton>
                </SidebarMenuItem>
              </SidebarMenu>
            ) : activeGuild ? (
              activeTab === "members" ? (
                <MemberList members={activeGuild.members} />
              ) : (
                <ChannelList
                  uncategorized={uncategorized}
                  categorized={categorized}
                  activeGuild={activeGuild}
                  activeChannelId={params.channelId}
                />
              )
            ) : null}
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>

      <SidebarFooter
        setSettingsDialogOpen={setSettingsDialogOpen}
        currentUser={currentUser}
      />
    </ShadSidebar>
  );
}

function ChannelList({
  uncategorized,
  categorized,
  activeGuild,
  activeChannelId
}: {
  uncategorized: Channel[];
  categorized: CategoryGroup[];
  activeGuild: Guild;
  activeChannelId?: string;
}) {
  if (uncategorized.length === 0 && categorized.length === 0) {
    return (
      <div className="flex flex-col items-center justify-center gap-2 py-10 text-center">
        <MessageSquareDashed className="size-8 text-muted-foreground/30" />
        <p className="text-xs text-muted-foreground/50">No channels yet</p>
      </div>
    );
  }

  return (
    <div className="space-y-0.5">
      {uncategorized.length > 0 && (
        <SidebarMenu>
          {uncategorized.map((channel) => (
            <ChannelItem
              key={channel.id}
              channel={channel}
              guildId={activeGuild.id}
              isActive={channel.id === activeChannelId}
            />
          ))}
        </SidebarMenu>
      )}
      {categorized.map((category) => (
        <CategoryGroup
          key={category.id}
          category={category}
          guildId={activeGuild.id}
          activeChannelId={activeChannelId}
        />
      ))}
    </div>
  );
}

function MemberList({ members }: { members: Guild["members"] }) {
  const online = members.filter(
    (m) =>
      m.data.presence.status !== "Offline" &&
      m.data.presence.status !== "Invisible"
  );
  const offline = members.filter(
    (m) =>
      m.data.presence.status === "Offline" ||
      m.data.presence.status === "Invisible"
  );

  return (
    <div className="space-y-3">
      <MemberGroup label={`Online — ${online.length}`} members={online} />
      {offline.length > 0 && (
        <MemberGroup label={`Offline — ${offline.length}`} members={offline} />
      )}
    </div>
  );
}

function MemberGroup({
  label,
  members
}: {
  label: string;
  members: Guild["members"];
}) {
  return (
    <div>
      <p className="text-sm mb-1.5 font-bold uppercase tracking-wider text-muted-foreground/50">
        {label}
      </p>
      {members.map((member) => (
        <UserProfileCard user={member.data} side="right" align="start">
          <button
            key={member.user_id}
            className="flex w-full items-center gap-2 cursor-pointer px-2 py-1 rounded-md hover:bg-white/5 transition-colors"
            type="button"
          >
            <div className="relative shrink-0">
              <Avatar className="size-6">
                {member.data.avatar_url && (
                  <AvatarGif src={member.data.avatar_url} alt="" />
                )}
                <AvatarFallback className="text-[10px]">
                  {member.data.display_name.charAt(0)}
                </AvatarFallback>
                <AvatarBadge status={member.data.presence.status} />
              </Avatar>
            </div>
            <span
              className={cn(
                "truncate text-sm font-medium",
                member.data.presence.status === "Offline"
                  ? "text-muted-foreground/50"
                  : "text-muted-foreground"
              )}
            >
              {member.data.display_name}
            </span>
          </button>
        </UserProfileCard>
      ))}
    </div>
  );
}
