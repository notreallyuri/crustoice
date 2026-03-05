import { useAppStore } from "@/store/app-store";
import { Outlet, createRootRoute, redirect } from "@tanstack/react-router";
import { TanStackRouterDevtools } from "@tanstack/router-devtools";

export const Route = createRootRoute({
  beforeLoad: async ({ location }) => {
    const state = useAppStore.getState();

    const isAuthPath = location.pathname.startsWith("/auth");

    if (!state.currentUser) {
      try {
        await state.initSession();
      } catch (e) {
        console.log("Hydration Error (No Session): ", e);

        if (!isAuthPath) {
          throw redirect({ to: "/auth/login" });
        }
      }
    }

    if (location.pathname === "/") {
      throw redirect({ to: "/g/@me" });
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
