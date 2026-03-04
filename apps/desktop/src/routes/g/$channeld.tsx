import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/g/$channeld')({
  component: RouteComponent,
})

function RouteComponent() {
  return <div>Hello "/g/$channeld"!</div>
}
