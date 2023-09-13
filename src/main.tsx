import React from "react";
import ReactDOM from "react-dom/client";
import {invoke} from "@tauri-apps/api/tauri";
import {QueryClient, QueryClientProvider} from "@tanstack/react-query";
import {createBrowserRouter, Navigate, RouterProvider} from "react-router-dom";
import {App} from "./App.tsx";
import {DayView} from "./views/DayView.tsx";

const currentIsoDateOnly = () => {
  const date = new Date(Date.now());
  return date.toISOString().split('T')[0]
}

await invoke("create_tables");
const queryClient = new QueryClient();

const router = createBrowserRouter([
  {
    path: "/",
    element: <App/>,
    children: [
      {
        path: "day/:date",
        element: <DayView/>,
        loader: async ({params}) => {
          console.log(params.date);
          const day = await invoke<Day>('get_day', {date: params.date})
          return {day} as { day: Day }
        }
      },
      {
        path: "day",
        element: <Navigate to={`/day/${currentIsoDateOnly()}`}/>
      }
    ]
  },
]);

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <QueryClientProvider client={queryClient}>
      <RouterProvider router={router}/>
    </QueryClientProvider>
  </React.StrictMode>
);