import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/_auth/guild/$guildId')({
  component: RouteComponent,
})

function RouteComponent() {
  return <div>Hello "/_auth/guild/$guildId"!</div>
}
