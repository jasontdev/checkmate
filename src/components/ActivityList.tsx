import ActivityListItem from "./ActivityListItem";

type ActivityListProps = {
  activities: Activity[];
};

function ActivityList({ activities}: ActivityListProps) {
  return (
    <div className="flex flex-col gap-1 p-2">
      {activities.map((activity) => (
        <ActivityListItem
          key={activity.id}
          activity={activity}
        />
      ))}
    </div>
  );
}

export default ActivityList;
