import ActivityList from "../components/ActivityList";
import DayNavBar from "../ui/DayNavBar";

type DayViewProps = {
  day: Day;
};

function DayView({ day }: DayViewProps) {
  return (
    <div>
      <DayNavBar date={day.date} />
      {day.activities ? (
        <ActivityList activities={day.activities} />
      ) : null}
    </div>
  );
}

export default DayView;
