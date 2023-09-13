import Button from "../ui/Button.tsx";

function NoTasks() {
  return (
    <div className="flex h-full flex-col items-center justify-around">
      <div className="text-4xl font-semibold">No tasks... yet</div>
      <Button
        title={"New task"}
        onClick={() => {
          console.log("Clicked")
        }}
        solid
      />
    </div>
  );
}

export default NoTasks;
