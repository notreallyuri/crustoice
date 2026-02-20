import { RootSidebar } from "@/components/root-sidebar";
import { useAppStore } from "@/store/app-store";
import { createFileRoute, Outlet } from "@tanstack/react-router";

export const Route = createFileRoute("/_auth")({
  component: AuthLayout
});

function AuthLayout() {
  const { activeChannelId, activeGuildId, guilds, currentUser, selectChannel } =
    useAppStore();

  const activeGuild = guilds.find((g) => g.id === activeGuildId);

  return (
    <div className="flex h-screen">
      <RootSidebar
        activeChannelId={activeChannelId}
        activeGuildId={activeGuildId}
        guilds={guilds}
        activeGuild={activeGuild}
        currentUser={currentUser}
        channelAction={selectChannel}
      />

      <main className="flex flex-1 flex-col min-w-0">
        <Outlet />
      </main>
    </div>
  );
}
