import { useEffect, useState } from "react";
import DayView from "./views/DayView";
import { invoke } from "@tauri-apps/api/tauri";

function App() {
  const [day, setDay] = useState<Day>({
    date: "",
    activities: [],
  });

  const [date, setDate] = useState<string>(new Date(Date.now()).toDateString());

  useEffect(() => {
    invoke("get_day", {date}).then((day) => {
      setDay(day as Day);
    }).catch(error => console.log(error));
  }, [date]);

  if (day.activities) {
    return <DayView day={day} setDate={setDate}/>;
  }
}

export default App;
