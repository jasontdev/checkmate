import TaskList from "../components/TaskList.tsx";
import DayNavBar from "../components/DayNavBar.tsx";
import NoTasks from "../components/NoTasks.tsx";
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
  toTasks: () => void;
  toCreateTasks: () => void;
};

function DayView({day, setDate}: DayViewProps) {
  function onNextDateClick() {
    setDate(addDaysToDateString(day.date, 1));
  }

  function onPrevDateClick() {
    setDate(addDaysToDateString(day.date, -1));
  }

  enum MainComponent {
    Tasks,
    CreateTask,
  }

  const dayViewNav: DayViewNav = {
    toTasks: () => {
      setMainComponent(MainComponent.Tasks);
    },
    toCreateTasks: () => {
      setMainComponent(MainComponent.CreateTask);
    },
  };

  // TODO: make this a component
  function renderMainComponent(mainComponent: MainComponent, day: Day) {
    switch (mainComponent) {
      case MainComponent.Tasks: {
        return day.tasks.length > 0 ? (
          <TaskList tasks={day.tasks}/>
        ) : (
          <NoTasks dayViewNav={dayViewNav}/>
        );
      }

      case MainComponent.CreateTask: {
        return <CreateTask dayViewNav={dayViewNav} day={day}/>;
      }
    }
  }

  const [mainComponent, setMainComponent] = useState(MainComponent.Tasks);

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
