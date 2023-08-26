import React from "react";
import ReactDOM from "react-dom/client";
import { App } from "./App.tsx";
import { invoke } from "@tauri-apps/api/tauri";

await invoke("create_tables");

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
