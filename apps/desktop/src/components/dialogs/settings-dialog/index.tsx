import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogTitle
} from "@/components/ui/dialog";
import { SidebarProvider } from "@/components/ui/sidebar";
import { useSidebarData } from "./nav-hook";
import { SettingsSidebar } from "./sidebar";
import { useCurrentUser } from "@/hooks/use-current-user";
import { SettingsRender } from "./settings-render";

type Props = {
  open: boolean;
  onOpenChange: (open: boolean) => void;
};

export function SettingsDialog({ open, onOpenChange }: Props) {
  const currentUser = useCurrentUser();

  const { sidebarData, setActiveRoute, activeRoute } = useSidebarData();

  return (
    <Dialog onOpenChange={onOpenChange} open={open}>
      <DialogContent className="w-[calc(100vw-16rem)] h-[calc(100vh-10rem)] max-w-none! p-0 rounded-none border-0 overflow-hidden flex">
        <DialogTitle className="sr-only">Settings</DialogTitle>
        <DialogDescription className="sr-only">
          Configure your preferences and settings for the application
        </DialogDescription>
        <SidebarProvider>
          <SettingsSidebar
            currentRoute={activeRoute}
            data={sidebarData}
            onRouteChange={setActiveRoute}
            user={currentUser}
          />
          <main className="relative flex flex-1 flex-col overflow-hidden bg-background">
            <div className="sticky w-full px-8 py-2 ">
              <h1 className="font-bold text-2xl">{activeRoute.name}</h1>
              <p className="text-muted-foreground text-sm">
                {activeRoute.description}
              </p>
            </div>
            <div className="overflow-y-auto px-8 py-6">
              <SettingsRender currentRoute={activeRoute.name} />
            </div>
          </main>
        </SidebarProvider>
      </DialogContent>
    </Dialog>
  );
}
