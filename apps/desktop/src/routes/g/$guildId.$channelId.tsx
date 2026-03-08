import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/g/$guildId/$channelId')({
  component: RouteComponent,
})

function RouteComponent() {
  return <div>Hello "/g/$channeld"!</div>
}
