import Button from "../ui/Button";
import Container from "../ui/Container";
import { type DayViewNav } from "../views/DayView";

type CreateActivityProps = {
  dayId: number;
  dayViewNav: DayViewNav;
  updateDay: () => void;
};

function CreateActivity({ dayViewNav, updateDay }: CreateActivityProps) {
  function handleSaveButtonClick() {
    updateDay();
    dayViewNav.toActivities();
  }

  return (
    <Container>
      <div className="flex h-full flex-col items-center justify-around">
        <div>
          <label className="font-bold">
            Description:
            <input />
          </label>
        </div>
        <div>
          <Button
            title={"Save"}
            onClick={() => handleSaveButtonClick()}
            solid
          />
          <Button title={"Cancel"} onClick={() => dayViewNav.toActivities()} />
        </div>
      </div>
    </Container>
  );
}

export default CreateActivity;
