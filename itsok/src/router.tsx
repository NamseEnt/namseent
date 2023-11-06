import { RootRoute, Route, Router } from "@tanstack/react-router";
import { Root } from "./Root";
import { InProgressPage } from "./pages/inProgress/InProgressPage";

const rootRoute = new RootRoute({
  component: Root,
});

const routeTree = rootRoute.addChildren([
  new Route({
    getParentRoute: () => rootRoute,
    path: "/",
    component: InProgressPage,
  }),
]);

export const router = new Router({ routeTree });

declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}
