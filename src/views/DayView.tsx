import ActivityList from "../components/ActivityList";
import DayNavBar from "../components/DayNavBar.tsx";
import NoActivities from "../components/NoActivities.tsx";
import React from "react";
import Container from "../ui/Container.tsx";

type DayViewProps = {
  day: Day;
  setDate: React.Dispatch<React.SetStateAction<string>>;
};

function addDaysToDateString(dateString: string, daysToAdd: number) {
  const date = new Date(dateString);
  date.setDate(date.getDate() + daysToAdd)

  return date.toDateString();
}

function handleNewActivityClick() {
}

function DayView({ day, setDate }: DayViewProps) {
  function onNextDateClick() {
    setDate(addDaysToDateString(day.date, 1));
  }

  function onPrevDateClick() {
    setDate(addDaysToDateString(day.date, -1));
  }

  return (
    <Container>
      <DayNavBar date={day.date}  onNextDateClick={onNextDateClick} onPrevDateClick={onPrevDateClick}/>
      {day.activities.length > 0 ? (
        <ActivityList activities={day.activities} />
      ) : <NoActivities handleNewActivityClick={handleNewActivityClick} />}
    </Container>
  );
}

export default DayView;
