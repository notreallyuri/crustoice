import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/g/$guildId/$channeld')({
  component: RouteComponent,
})

function RouteComponent() {
  return <div>Hello "/g/$channeld"!</div>
}
