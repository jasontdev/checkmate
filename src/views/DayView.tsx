import { Dispatch, SetStateAction } from "react";
import ActivityList from "../components/ActivityList";
import DayNavBar from "../ui/DayNavBar";

type DayViewProps = {
  day: Day;
  setActivity: Dispatch<SetStateAction<number | null>>;
};

function DayView({ day, setActivity }: DayViewProps) {
  const hasActivities = day.activites.length > 0;

  return (
    <div>
      <DayNavBar date={day.date} />
      {hasActivities ? (
        <ActivityList activities={day.activites} setActivity={setActivity} />
      ) : null}
    </div>
  );
}

export default DayView;
