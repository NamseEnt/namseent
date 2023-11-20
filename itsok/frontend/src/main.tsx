import React from "react";
import ReactDOM from "react-dom/client";
import { RouterProvider } from "@tanstack/react-router";
import { router } from "./router.tsx";
import {
  CssBaseline,
  ThemeProvider,
  createTheme,
  getContrastRatio,
} from "@mui/material";

const primaryMain = "#BEADFA";
const theme = createTheme({
  palette: {
    primary: {
      main: primaryMain,
      contrastText:
        getContrastRatio(primaryMain, "#fff") > 4.5 ? "#fff" : "#111",
    },
    secondary: {
      main: "#FFF8C9",
    },
  },
});

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <CssBaseline />
    <ThemeProvider theme={theme}>
      <RouterProvider router={router} />
    </ThemeProvider>
  </React.StrictMode>,
);
