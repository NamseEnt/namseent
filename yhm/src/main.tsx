import { createRoot } from "react-dom/client";
import "./index.css";
import { createBrowserRouter } from "react-router";
import { RouterProvider } from "react-router/dom";
import Home from "./routes/home";
import SettingsPage from "./routes/settings";
import PlayPageRoute from "./routes/play";

const router = createBrowserRouter([
  {
    path: "/",
    element: <Home />,
  },
  {
    path: "/settings",
    element: <SettingsPage />,
  },
  {
    path: "/play",
    element: <PlayPageRoute />,
  },
]);

createRoot(document.getElementById("root")!).render(
  <RouterProvider router={router} />
);
