import React from "react";
import ReactDOM from "react-dom/client";
import { routeTree } from "./routeTree.gen";
import { createRouter, RouterProvider } from "@tanstack/react-router";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { SplashScreen } from "./components/splash-screen";
import { Toaster } from "@/components/ui/sonner";
import { ThemeProvider } from "./components/theme-provider";

import "@fontsource/figtree/400.css";
import "@fontsource/figtree/500.css";
import "@fontsource/figtree/600.css";
import "@fontsource/figtree/700.css";

import "./globals.css";

const router = createRouter({ routeTree });
const currentWindow = getCurrentWindow();

function App() {
  if (currentWindow.label === "splashscreen") return <SplashScreen />;

  return (
    <ThemeProvider>
      <Toaster position="bottom-left" />
      <RouterProvider router={router} />
    </ThemeProvider>
  );
}

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
