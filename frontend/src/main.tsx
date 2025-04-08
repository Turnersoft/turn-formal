import React from "react";
import ReactDOM from "react-dom/client";
import { App } from "./App";
import { setBasePath } from "@shoelace-style/shoelace/dist/utilities/base-path";

// Set the base path for Shoelace assets
setBasePath("/node_modules/@shoelace-style/shoelace/dist");

// Import only Shoelace components needed for interactive elements
import "@shoelace-style/shoelace/dist/components/tooltip/tooltip.js";

// Import global styles (SCSS)
import "./styles/global.scss";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
