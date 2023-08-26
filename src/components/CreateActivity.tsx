import { invoke } from "@tauri-apps/api";
import Button from "../ui/Button";
import Container from "../ui/Container";
import { type DayViewNav } from "../views/DayView";

type CreateActivityProps = {
  day: Day;
  dayViewNav: DayViewNav;
};

function CreateActivity({ dayViewNav, day }: CreateActivityProps) {
  function handleSaveButtonClick() {
    invoke("update_day", {
      day: day 
    });
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
