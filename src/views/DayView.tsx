import TaskList from "../components/TaskList.tsx";
import DayNavBar from "../components/DayNavBar.tsx";
import NoActivities from "../components/NoActivities.tsx";
import React, {useState} from "react";
import Container from "../ui/Container.tsx";
import CreateTask from "../components/CreateTask.tsx";

type DayViewProps = {
  day: Day;
  setDate: React.Dispatch<React.SetStateAction<string>>;
};

function addDaysToDateString(dateString: string, daysToAdd: number) {
  const date = new Date(dateString);
  date.setDate(date.getDate() + daysToAdd);

  return date.toDateString();
}

type DayViewNav = {
  toActivities: () => void;
  toCreateActivity: () => void;
};

function DayView({day, setDate}: DayViewProps) {
  function onNextDateClick() {
    setDate(addDaysToDateString(day.date, 1));
  }

  function onPrevDateClick() {
    setDate(addDaysToDateString(day.date, -1));
  }

  enum MainComponent {
    Activites,
    CreateActivity,
  }

  const dayViewNav: DayViewNav = {
    toActivities: () => {
      setMainComponent(MainComponent.Activites);
    },
    toCreateActivity: () => {
      setMainComponent(MainComponent.CreateActivity);
    },
  };

  // TODO: make this a component
  function renderMainComponent(mainComponent: MainComponent, day: Day) {
    switch (mainComponent) {
      case MainComponent.Activites: {
        return day.tasks.length > 0 ? (
          <TaskList activities={day.tasks}/>
        ) : (
          <NoActivities dayViewNav={dayViewNav}/>
        );
      }

      case MainComponent.CreateActivity: {
        return <CreateTask dayViewNav={dayViewNav} day={day}/>;
      }
    }
  }

  const [mainComponent, setMainComponent] = useState(MainComponent.Activites);

  return (
    <Container>
      <DayNavBar
        date={day.date}
        onNextDateClick={onNextDateClick}
        onPrevDateClick={onPrevDateClick}
      />
      {renderMainComponent(mainComponent, day)}
    </Container>
  );
}

export {DayView, type DayViewNav};
