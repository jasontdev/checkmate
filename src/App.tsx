import { useState } from "react";
import { DayView } from "./views/DayView";
import { useQuery } from "./api/backend";

enum Views {
  DayView,
}

type AppNav = {
  toDayView: () => void;
};

function App() {
  const [date, setDate] = useState<string>(new Date(Date.now()).toDateString());
  const [currentView] = useState<Views>(Views.DayView);

  const mutationEvent = `day_${date.replace(/ /g, "_")}_updated`;

  const dayQuery = useQuery<Day>("get_day", { date }, mutationEvent);

  if (currentView === Views.DayView) {
    return (
      <div className="h-screen w-screen">
        {dayQuery.data ? (
          <DayView day={dayQuery.data} setDate={setDate} />
        ) : (
          <div />
        )}
      </div>
    );
  }
}

export { App, type AppNav };
