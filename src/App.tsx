import { useEffect, useState } from "react";
import DayView from "./views/DayView";
import { invoke } from "@tauri-apps/api/tauri";

function App() {
  const [day, setDay] = useState<Day>({
    date: "",
    activities: [],
  });

  useEffect(() => {
    invoke("get_day", {day: "2023-07-06"}).then((day) => {
      setDay(day as Day);
    });
  }, []);

  if (day.activities) {
    return <DayView day={day}/>;
  }
}

export default App;
