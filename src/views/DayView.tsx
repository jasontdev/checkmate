import DayNavBar from "../components/DayNavBar.tsx";
import Container from "../ui/Container.tsx";
import { useLoaderData, useNavigate, useParams } from "react-router-dom";
import React from "react";
import TaskList from "../components/TaskList.tsx";
import NoTasks from "../components/NoTasks.tsx";

type DayViewNav = {
  toTasks: () => void;
  toCreateTasks: () => void;
};

function DayView() {
  const { date } = useParams();
  const { day } = useLoaderData() as { day: Day };
  const navigate = useNavigate();

  function changeDate(date: string, n: number) {
    const nextDay = new Date(date);
    nextDay.setDate(nextDay.getDate() + n);
    return nextDay.toISOString().split("T")[0];
  }

  if (date) {
    return (
      <Container>
        <DayNavBar
          date={day.date}
          onNextDateClick={() => navigate("/day/" + changeDate(date, 1))}
          onPrevDateClick={() => navigate("/day/" + changeDate(date, -1))}
        />
        {day.tasks.length > 0 ? <TaskList tasks={day.tasks} /> : <NoTasks />}
      </Container>
    );
  }
}

export { DayView };
