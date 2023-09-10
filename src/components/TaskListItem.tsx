import Button from "../ui/Button";

type TaskListItemProps = {
  task: Task;
};

function TaskListItem({task}: TaskListItemProps) {
  const {description, project, category} = task;
  return (
    <div className="flex-between flex w-full items-center justify-between px-2 py-1 even:bg-zinc-100">
      <div>
        {description}
      </div>
      <div>
        <Button title={"Resume"} onClick={() => {
          console.log("Resume button clicked")
        }}/>
      </div>
    </div>
  );
}

export default TaskListItem;
