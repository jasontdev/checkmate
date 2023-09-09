import React from "react";
import ReactDOM from "react-dom/client";
import {App} from "./App.tsx";
import {invoke} from "@tauri-apps/api/tauri";
import {QueryClient, QueryClientProvider} from "@tanstack/react-query";

await invoke("create_tables");
const queryClient = new QueryClient();

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <QueryClientProvider client={queryClient}>
      <App/>
    </QueryClientProvider>
  </React.StrictMode>
);
