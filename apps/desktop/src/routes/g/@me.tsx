import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/g/@me')({
  component: RouteComponent,
})

function RouteComponent() {
  return <div>Hello "/g/@me"!</div>
}
