import { SettingsDialog } from "@/components/dialogs/settings-dialog";
import { Sidebar } from "@/components/layout/sidebar";
import { SidebarInset, SidebarProvider } from "@/components/ui/sidebar";
import { createFileRoute, Outlet } from "@tanstack/react-router";
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
      <div className="[--header-height:--spacing(6)]">
        <SidebarProvider>
          <Sidebar setSettingsDialogOpen={setSettingsDialogOpen} />
          <SidebarInset className="bg-sidebar">
            <div className="group" data-tauri-drag-region>
              <div className="absolute w-full h-4" />
              <div className="flex overflow-hidden relative justify-end items-center h-0 gap-2 transition-all duration-200 group-hover:h-6 px-3">
                <button className="opacity-0 transition-opacity group-hover:opacity-100 size-3 rounded-full bg-yellow-400 hover:bg-yellow-300" />
                <button className="opacity-0 transition-opacity group-hover:opacity-100 size-3 rounded-full bg-green-500 hover:bg-green-400" />
                <button className="opacity-0 transition-opacity group-hover:opacity-100 size-3 rounded-full bg-red-500 hover:bg-red-400" />
              </div>
            </div>
            <div className="bg-background h-full rounded">
              <Outlet />
            </div>
          </SidebarInset>
        </SidebarProvider>
      </div>
    </>
  );
}
