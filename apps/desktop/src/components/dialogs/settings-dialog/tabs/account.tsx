import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";
import { useState } from "react";
import { KeyRound, Mail, ShieldAlert, UserSquare2 } from "lucide-react";
import { useAppStore } from "@/store/app-store";

export function AccountSettings() {
  const user = useAppStore((s) => s.currentUser);
  const [revealEmail, setRevealEmail] = useState(false);

  if (!user) return null;

  return (
    <div className="w-full max-w-3xl space-y-10 pb-10">
      <div className="space-y-6">
        <div>
          <h2 className="text-sm font-bold uppercase tracking-wider text-muted-foreground mb-4">
            Identity & Login
          </h2>
          <div className="border border-white/10 bg-black/20 overflow-hidden">
            <div className="flex items-center justify-between p-4 hover:bg-white/5 transition-colors">
              <div className="flex items-center gap-4">
                <div className="p-2 bg-white/5 border rounded border-white/10 text-muted-foreground">
                  <UserSquare2 className="size-4" />
                </div>
                <div>
                  <p className="text-sm font-medium text-foreground">
                    Username
                  </p>
                  <p className="text-xs text-muted-foreground mt-0.5">
                    {user.profile.username}
                  </p>
                </div>
              </div>
              <Button variant="outline" size="sm">
                Change
              </Button>
            </div>

            <Separator className="bg-white/10" />

            <div className="flex items-center justify-between p-4 hover:bg-white/5 transition-colors">
              <div className="flex items-center gap-4">
                <div className="p-2 bg-white/5 border border-white/10 rounded text-muted-foreground">
                  <Mail className="size-4" />
                </div>
                <div>
                  <p className="text-sm font-medium text-foreground">
                    Email Address
                  </p>
                  <p className="text-xs text-muted-foreground mt-0.5 font-mono">
                    {revealEmail
                      ? user.account.email
                      : user.account.email.replace(
                          /(.{2})(.*)(?=@)/,
                          "$1••••••••"
                        )}
                  </p>
                </div>
              </div>
              <div className="flex items-center gap-2">
                <Button
                  variant="ghost"
                  size="sm"
                  onClick={() => setRevealEmail((prev) => !prev)}
                >
                  {revealEmail ? "Hide" : "Reveal"}
                </Button>
                <Button variant="outline" size="sm">
                  Update
                </Button>
              </div>
            </div>

            <Separator className="bg-white/10" />

            <div className="flex items-center justify-between p-4 hover:bg-white/5 transition-colors">
              <div className="flex items-center gap-4">
                <div className="p-2 bg-white/5 border border-white/10 rounded text-muted-foreground">
                  <KeyRound className="size-4" />
                </div>
                <div>
                  <p className="text-sm font-medium text-foreground">
                    Password
                  </p>
                  <p className="text-xs text-muted-foreground mt-0.5">
                    Last changed 3 months ago
                  </p>
                </div>
              </div>
              <Button variant="outline" size="sm">
                Update Password
              </Button>
            </div>
          </div>
        </div>
      </div>

      <div className="space-y-6">
        <div>
          <h2 className="text-sm font-bold uppercase tracking-wider text-destructive mb-4 flex items-center gap-2">
            <ShieldAlert className="size-4" />
            Danger Zone
          </h2>

          <div className="border border-destructive/20 bg-destructive/5 overflow-hidden">
            <div className="flex items-center justify-between p-4">
              <div>
                <p className="text-sm font-medium text-foreground">
                  Disable Account
                </p>
                <p className="text-xs text-muted-foreground mt-0.5">
                  Temporarily suspend your account. You can recover it at any
                  time by logging back in.
                </p>
              </div>
              <Button variant="outline">Disable</Button>
            </div>

            <Separator className="bg-destructive/20" />

            <div className="flex items-center justify-between p-4">
              <div>
                <p className="text-sm font-medium text-foreground">
                  Delete Account
                </p>
                <p className="text-xs text-muted-foreground mt-0.5">
                  Permanently remove your account and all associated data. This
                  action cannot be undone.
                </p>
              </div>
              <Button variant="destructive">Delete Account</Button>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
