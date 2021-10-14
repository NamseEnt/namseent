import { SnackbarProvider } from "notistack";
import React from "react";
import ReactDOM from "react-dom";
import App from "./components/App";
import Provider from "./store/Provider";

ReactDOM.render(
  <SnackbarProvider maxSnack={3}>
    <Provider>
      <App />
    </Provider>
  </SnackbarProvider>,
  document.getElementById("root"),
);
