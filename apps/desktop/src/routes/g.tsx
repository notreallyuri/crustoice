import { SettingsDialog } from "@/components/dialogs/settings-dialog";
import { Sidebar } from "@/components/layout/sidebar";
import { SidebarInset, SidebarProvider } from "@/components/ui/sidebar";
import { createFileRoute, Outlet } from "@tanstack/react-router";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useState } from "react";

export const Route = createFileRoute("/g")({
  component: RouteComponent
});

function RouteComponent() {
  const [settingsDialogOpen, setSettingsDialogOpen] = useState(false);

  return (
    <>
      <SettingsDialog
        open={settingsDialogOpen}
        onOpenChange={setSettingsDialogOpen}
      />
      <div className="[--header-height:--spacing(8)]">
        <SidebarProvider>
          <Sidebar setSettingsDialogOpen={setSettingsDialogOpen} />
          <SidebarInset>
            <header
              data-tauri-drag-region
              className="relative flex bg-sidebar h-8 w-full shrink-0 items-center  px-3 select-none"
            >
              <div className="flex items-center gap-1.5 ml-auto">
                <button className="size-3 rounded-full bg-yellow-400 hover:bg-yellow-300 transition-colors" />
                <button className="size-3 rounded-full bg-green-500 hover:bg-green-400 transition-colors" />
                <button className="size-3 rounded-full bg-red-500 hover:bg-red-400 transition-colors" />
              </div>
            </header>
            <Outlet />
          </SidebarInset>
        </SidebarProvider>
      </div>
    </>
  );
}
