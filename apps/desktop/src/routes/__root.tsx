import { useAppStore } from "@/store/app-store";
import { Outlet, createRootRoute, redirect } from "@tanstack/react-router";
import { TanStackRouterDevtools } from "@tanstack/router-devtools";
import { invoke } from "@tauri-apps/api/core";

export const Route = createRootRoute({
  beforeLoad: async ({ location }) => {
    if (location.pathname === "/") {
      try {
        const userId = await invoke<string>("check_auth");
        useAppStore.getState().fetchUser(userId);
        throw redirect({ to: "/g/me" });
      } catch {
        throw redirect({ to: "/auth/login" });
      }
    }
  },
  component: RootComponent
});

function RootComponent() {
  return (
    <main className="dark w-full ">
      <Outlet />
      {process.env.NODE_ENV === "development" && (
        <TanStackRouterDevtools position="bottom-right" />
      )}
    </main>
  );
}
