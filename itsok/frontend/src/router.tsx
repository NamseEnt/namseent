import { RootRoute, Route, Router } from "@tanstack/react-router";
import { Root } from "./Root";
import { InProgressPage } from "./pages/inProgress/InProgressPage";
import { WaitForResponsePage } from "./pages/WaitForResponsePage";
import { ResponsePage } from "./pages/ResponsePage";
import { NewGoalPage } from "./pages/NewGoalPage";
import { HistoryPage } from "./pages/history/HistoryPage";

const rootRoute = new RootRoute({
  component: Root,
});

const routeTree = rootRoute.addChildren([
  new Route({
    getParentRoute: () => rootRoute,
    path: "/new-goal",
    component: NewGoalPage,
  }),
  new Route({
    getParentRoute: () => rootRoute,
    path: "/in-progress",
    component: InProgressPage,
  }),
  new Route({
    getParentRoute: () => rootRoute,
    path: "/wait-for-response",
    component: WaitForResponsePage,
  }),
  new Route({
    getParentRoute: () => rootRoute,
    path: "/response",
    component: ResponsePage,
  }),
  new Route({
    getParentRoute: () => rootRoute,
    path: "/history",
    component: HistoryPage,
  }),
]);

export const router = new Router({ routeTree });

declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}
