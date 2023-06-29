import Button from "./Button";

type DayNavBarProps = {
  date: string;
};

function DayNavBar({ date }: DayNavBarProps) {
  return (
    <div className="flex items-center justify-between p-1">
      <div className="flex w-full items-center gap-1">
        <Button title="Previous" />
        <div className="text-xl font-bold">{date}</div>
        <Button title="Next" />
      </div>
      <div>
        <Button title="New" solid />
      </div>
    </div>
  );
}

export default DayNavBar;
