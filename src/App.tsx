import { useEffect, useState } from "react";
import { DayView } from "./views/DayView";
import { invoke } from "@tauri-apps/api/tauri";

enum Views {
  DayView,
}

type AppNav = {
  toDayView: () => void;
};

function App() {
  const [day, setDay] = useState<Day>({
    id: -1,
    date: "",
    activities: [],
  });
  const [date, setDate] = useState<string>(new Date(Date.now()).toDateString());
  const [currentView] = useState<Views>(Views.DayView);

  useEffect(() => {
    invoke("get_day", { date })
      .then((day) => {
        setDay(day as Day);
      })
      .catch((error) => console.log(error));
  }, [date]);

  function updateDay() {
    invoke("get_day", { date })
      .then((day) => {
        setDay(day as Day);
      })
      .catch((error) => console.log(error));
  }

  if (currentView === Views.DayView) {
    return (
      <div className="h-screen w-screen">
        <DayView day={day} setDate={setDate} updateDay={updateDay} />
      </div>
    );
  }
}

export { App, type AppNav };
