import { Dispatch, SetStateAction } from "react";
import Button from "../ui/Button";

type ActivityListItemProps = {
  activity: Activity;
  setActivity: Dispatch<SetStateAction<number | null>>;
};

function ActivityListItem({ activity, setActivity }: ActivityListItemProps) {
  const { id, project, task } = activity;
  return (
    <div className="flex-between flex w-full items-center justify-between px-2 py-1 even:bg-zinc-100">
      <div>
        {project} - {task}
      </div>
      <div>
        <Button title={"Resume"} onClick={() => setActivity(id)} />
      </div>
    </div>
  );
}

export default ActivityListItem;
