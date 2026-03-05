import { useAppStore } from "@/store/app-store";
import { redirect } from "@tanstack/react-router";

export function useCurrentUser() {
  const user = useAppStore((s) => s.currentUser);

  if (!user) {
    throw redirect({ to: "/auth/login" });
  }

  return user;
}
