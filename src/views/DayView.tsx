import ActivityList from "../components/ActivityList";
import DayNavBar from "../ui/DayNavBar";

type DayViewProps = {
  day: Day;
  setDate: React.Dispatch<React.SetStateAction<string>>
};

function addDaysToDateString(dateString: string, daysToAdd: number) {
  const date = new Date(dateString);
  date.setDate(date.getDate() + daysToAdd)

  return date.toDateString();
}


function DayView({ day, setDate }: DayViewProps) {
  function onNextDateClick() {
    setDate(addDaysToDateString(day.date, 1));
  }

  function onPrevDateClick() {
    setDate(addDaysToDateString(day.date, -1));
  }

  return (
    <div>
      <DayNavBar date={day.date}  onNextDateClick={onNextDateClick} onPrevDateClick={onPrevDateClick}/>
      {day.activities ? (
        <ActivityList activities={day.activities} />
      ) : null}
    </div>
  );
}

export default DayView;
