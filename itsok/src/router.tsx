import { RootRoute, Route, Router } from "@tanstack/react-router";
import { Root } from "./Root";
import { InProgressPage } from "./pages/inProgress/InProgressPage";
import { WaitForResponsePage } from "./pages/WaitForResponsePage";

const rootRoute = new RootRoute({
  component: Root,
});

const routeTree = rootRoute.addChildren([
  new Route({
    getParentRoute: () => rootRoute,
    path: "/",
    component: InProgressPage,
  }),
  new Route({
    getParentRoute: () => rootRoute,
    path: "/wait-for-response",
    component: WaitForResponsePage,
  }),
]);

export const router = new Router({ routeTree });

declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}
