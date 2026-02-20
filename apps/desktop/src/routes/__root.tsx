import { Outlet, createRootRoute } from "@tanstack/react-router";
import { TanStackRouterDevtools } from "@tanstack/router-devtools";

export const Route = createRootRoute({
  component: RootComponent
});

function RootComponent() {
  return (
    <main className="dark w-full ">
      <Outlet />
      {process.env.NODE_ENV === "development" && <TanStackRouterDevtools />}
    </main>
  );
}
