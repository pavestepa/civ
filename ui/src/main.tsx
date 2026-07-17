import React from "react";
import ReactDOM from "react-dom/client";
import { App } from "@/app/App";
import { startInputBridge } from "@/shared/lib";
import "@/app/index.css";

startInputBridge();

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
