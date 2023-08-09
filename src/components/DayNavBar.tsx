import Button from "../ui/Button.tsx";

type DayNavBarProps = {
  date: string;
  onPrevDateClick: () => void;
  onNextDateClick: () => void;
};

function DayNavBar({ date, onNextDateClick, onPrevDateClick }: DayNavBarProps) {
  return (
    <div className="flex items-center justify-between p-1">
      <div className="flex w-full items-center gap-1">
        <Button onClick={() => onPrevDateClick()} title="Previous" />
        <div className="text-xl font-bold">{date}</div>
        <Button onClick={() => onNextDateClick()} title="Next" />
      </div>
    </div>
  );
}

export default DayNavBar;
