import { Dispatch, SetStateAction } from "react";
import ActivityListItem from "./ActivityListItem";

type ActivityListProps = {
  activities: Activity[];
  setActivity: Dispatch<SetStateAction<number | null>>;
};

function ActivityList({ activities, setActivity }: ActivityListProps) {
  return (
    <div className="flex flex-col gap-1 p-2">
      {activities.map((activity) => (
        <ActivityListItem
          key={activity.id}
          activity={activity}
          setActivity={setActivity}
        />
      ))}
    </div>
  );
}

export default ActivityList;
