import Button from "../ui/Button";

type TaskListItemProps = {
  activity: Task;
};

function TaskListItem({activity}: TaskListItemProps) {
  const {description, project, category} = activity;
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
