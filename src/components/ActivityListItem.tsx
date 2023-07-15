import Button from "../ui/Button";

type ActivityListItemProps = {
  activity: Activity;
};

function ActivityListItem({ activity}: ActivityListItemProps) {
  const { description, project, task } = activity;
  return (
    <div className="flex-between flex w-full items-center justify-between px-2 py-1 even:bg-zinc-100">
      <div>
        {project.name} - {task.name} - {description}
      </div>
      <div>
        <Button title={"Resume"} onClick={() => {console.log("Resume button clicked")}} />
      </div>
    </div>
  );
}

export default ActivityListItem;
