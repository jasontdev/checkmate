import { useEffect, useState } from "react";
import { DayView } from "./views/DayView";
import { invoke } from "@tauri-apps/api/tauri";
import { useDay } from "./hooks/useDay";

enum Views {
  DayView,
}

type AppNav = {
  toDayView: () => void;
};

function App() {
  const [date, setDate] = useState<string>(new Date(Date.now()).toDateString());
  const [currentView] = useState<Views>(Views.DayView);
  
  const day = useDay(date);

  if (currentView === Views.DayView) {
    return (
      <div className="h-screen w-screen">
        <DayView day={day} setDate={setDate} />
      </div>
    );
  }
}

export { App, type AppNav };
