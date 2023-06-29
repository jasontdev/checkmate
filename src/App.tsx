import { useState } from "react";
import DayView from "./views/DayView";
import ActivityView from "./views/ActivityView";

function App() {
  const [day] = useState<Day>({
    date: "19/06/2023",
    activites: [{ id: 0, project: "Amy Swan", task: "Accounts", log: [] }],
  });
  const [currentActivity, setCurrentActivity] = useState<number | null>(null);

  if (currentActivity !== null) {
    let activity = day.activites.find(({ id }) => id === currentActivity);
    if (activity) {
      return <ActivityView activity={activity} />;
    }
  }
  return <DayView setActivity={setCurrentActivity} day={day} />;
}

export default App;
