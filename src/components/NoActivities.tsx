import Button from "../ui/Button.tsx";

type NoActivitiesProps = {
  handleNewActivityClick: () => void;
}
function NoActivities({handleNewActivityClick}: NoActivitiesProps) {
  return (
    <div className="flex flex-col items-center justify-around h-full">
      <div className="text-4xl font-semibold">No activities... yet</div>
      <Button title={"New activity"} onClick={() => handleNewActivityClick()} solid />
    </div>
  )
}

export default NoActivities;