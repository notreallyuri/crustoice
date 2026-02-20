import { createFileRoute, redirect } from "@tanstack/react-router";
import { useEffect, useState } from "react";
import { useAppStore } from "../store/app-store";
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";

export const Route = createFileRoute("/login")({
  beforeLoad: () => {
    const user = useAppStore.getState().currentUser;
    if (user) {
      throw redirect({ to: "/" });
    }
  },
  component: LoginScreen
});

function LoginScreen() {
  const login = useAppStore((state) => state.login);
  const currentUser = useAppStore((state) => state.currentUser);
  const navigate = Route.useNavigate();

  const [username, setUsername] = useState<string>("");
  const [isLoading, setIsLoading] = useState<boolean>(false);

  useEffect(() => {
    if (currentUser) {
      navigate({ to: "/" });
    }
  }, [currentUser, navigate]);

  async function handleLogin() {
    if (!username.trim()) return;
    setIsLoading(true);
    try {
      await login(username);
    } catch (error) {
      console.error("Login failed:", error);
    } finally {
      setIsLoading(false);
    }
  }

  return (
    <div className="flex h-screen items-center justify-center bg-[#1e1f22]">
      <div className="w-full max-w-100 rounded-lg bg-[#2b2d31] p-8 shadow-2xl">
        <div className="space-y-2 text-center">
          <h2 className="text-2xl font-bold text-white">Welcome back!</h2>
          <p className="text-sm text-zinc-400">
            We're so excited to see you again!
          </p>
        </div>

        <div className="mt-6 space-y-4">
          <div className="space-y-2">
            <Label
              htmlFor="username"
              className="text-xs font-bold uppercase text-zinc-400"
            >
              Username
            </Label>
            <Input
              id="username"
              value={username}
              onChange={(e) => setUsername(e.target.value)}
              onKeyDown={(e) => e.key === "Enter" && handleLogin()}
              className="border-none bg-[#1e1f22] text-white focus-visible:ring-1 focus-visible:ring-[#5865f2]"
              autoFocus
              placeholder="Your username"
            />
          </div>

          <Button
            onClick={handleLogin}
            disabled={isLoading || !username.trim()}
            className="w-full bg-[#5865f2] font-bold text-white hover:bg-[#4752c4]"
          >
            {isLoading ? "Logging in..." : "Log In"}
          </Button>
        </div>
      </div>
    </div>
  );
}
