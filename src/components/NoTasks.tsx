import Button from "../ui/Button.tsx";
import { type DayViewNav } from "../views/DayView.tsx";

type NoTasksProps = {
  dayViewNav: DayViewNav
};

function NoTasks({ dayViewNav }: NoTasksProps) {
  return (
    <div className="flex h-full flex-col items-center justify-around">
      <div className="text-4xl font-semibold">No tasks... yet</div>
      <Button
        title={"New task"}
        onClick={() => dayViewNav.toCreateTasks()}
        solid
      />
    </div>
  );
}

export default NoTasks;
