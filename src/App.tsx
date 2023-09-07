import {useState} from "react";
import {useQuery} from "@tanstack/react-query";
import {invoke} from "@tauri-apps/api/tauri";
import {DayView} from "./views/DayView.tsx";

enum Views {
  DayView,
}

type AppNav = {
  toDayView: () => void;
};

function App() {
  const [date, setDate] = useState<string>(new Date(Date.now()).toDateString());
  const [currentView] = useState<Views>(Views.DayView);

  const dayQuery = useQuery<Day, string>({
    queryKey: [`day_${date.replace(/ /g, " ")}`], queryFn: () => {
      return invoke('get_day', {date: date})
    }
  })

  if (currentView === Views.DayView) {
    return (
      <div className="h-screen w-screen">
        {dayQuery.isSuccess ? <DayView day={dayQuery.data} setDate={setDate}/> : null}
      </div>
    );
  }
}

export {App, type AppNav};
