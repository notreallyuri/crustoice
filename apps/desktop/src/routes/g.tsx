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
      <div>
        <SidebarProvider>
          <Sidebar setSettingsDialogOpen={setSettingsDialogOpen} />
          <SidebarInset>
            <Outlet />
          </SidebarInset>
        </SidebarProvider>
      </div>
    </>
  );
}
