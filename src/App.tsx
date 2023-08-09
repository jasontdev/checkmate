import {useEffect, useState} from "react";
import DayView from "./views/DayView";
import {invoke} from "@tauri-apps/api/tauri";

enum Views {
  DayView
}

function App() {
  const [day, setDay] = useState<Day>({
    date: "",
    activities: [],
  });
  const [date, setDate] = useState<string>(new Date(Date.now()).toDateString());
  const [currentView] = useState<Views>(Views.DayView);

  useEffect(() => {
    invoke("get_day", {date}).then((day) => {
      setDay(day as Day);
    }).catch(error => console.log(error));
  }, [date]);

  if (currentView === Views.DayView) {
    return (
      <div className="h-screen w-screen">
        <DayView day={day} setDate={setDate}/>
      </div>)
  }
}

export default App;
