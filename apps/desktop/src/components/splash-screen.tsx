import { invoke } from "@tauri-apps/api/core";
import { useEffect } from "react";

export function SplashScreen() {
  useEffect(() => {
    async function checkAuth() {
      await new Promise((res) => setTimeout(res, 1500));

      await invoke("close_splashscreen");
    }

    checkAuth();
  }, []);

  return (
    <div className="flex h-screen w-screen flex-col items-center justify-center bg-background dark text-white">
      <div className="size-16 animate-bounce rounded-2xl bg-primary shadow-lg shadow-primary/50" />
      <h1 className="mt-6 text-xl font-bold tracking-tight">Starting up...</h1>
    </div>
  );
}
