import TaskListItem from "./TaskListItem.tsx";

type TaskListProps = {
  tasks: Task[];
};

function TaskList({ tasks}: TaskListProps) {
  return (
    <div className="flex flex-col gap-1 p-2">
      {tasks.map((task) => (
        <TaskListItem
          key={task.id}
          task={task}
        />
      ))}
    </div>
  );
}

export default TaskList;
