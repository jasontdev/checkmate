import Button from "../ui/Button.tsx";
import { type DayViewNav } from "../views/DayView.tsx";

type NoActivitiesProps = {
  dayViewNav: DayViewNav
};

function NoActivities({ dayViewNav }: NoActivitiesProps) {
  return (
    <div className="flex h-full flex-col items-center justify-around">
      <div className="text-4xl font-semibold">No activities... yet</div>
      <Button
        title={"New activity"}
        onClick={() => dayViewNav.toCreateActivity()}
        solid
      />
    </div>
  );
}

export default NoActivities;
