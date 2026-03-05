import { useAppStore } from "@/store/app-store";
import { invoke } from "@tauri-apps/api/core";
import { useEffect } from "react";
import { useNavigate } from "@tanstack/react-router";

export function SplashScreen() {
  const initSession = useAppStore((s) => s.initSession);
  const navigate = useNavigate();

  useEffect(() => {
    async function checkAuth() {
      try {
        await invoke("check_auth");
      } catch (e) {
        console.log("No saved session found:", e);
      } finally {
        await invoke("close_splashscreen");
      }
    }

    setTimeout(checkAuth, 500);
  }, [initSession, navigate]);

  return (
    <div className="flex h-screen w-screen flex-col items-center justify-center bg-background dark text-white">
      <div className="size-16 animate-bounce rounded-2xl bg-primary shadow-lg shadow-primary/50" />
      <h1 className="mt-6 text-xl font-bold tracking-tight">Starting up...</h1>
    </div>
  );
}
