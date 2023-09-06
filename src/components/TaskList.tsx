import TaskListItem from "./TaskListItem.tsx";

type TaskListProps = {
  activities: Task[];
};

function TaskList({ activities}: TaskListProps) {
  return (
    <div className="flex flex-col gap-1 p-2">
      {activities.map((activity) => (
        <TaskListItem
          key={activity.id}
          activity={activity}
        />
      ))}
    </div>
  );
}

export default TaskList;
