import { invoke } from "@tauri-apps/api";
import Button from "../ui/Button";
import Container from "../ui/Container";
import { type DayViewNav } from "../views/DayView";
import { useMutation } from "../api/backend";
import { useState } from "react";

type CreateActivityProps = {
  day: Day;
  dayViewNav: DayViewNav;
};

function CreateActivity({ dayViewNav, day }: CreateActivityProps) {
  const newDay = useMutation<Day>("update_day");
  const [description, setDescription] = useState("");

  function handleSaveButtonClick() {
    const newDayData: Day = {
        id: day.id,
        date: day.date,
        activities: [...day.activities, { id: 0, dayId: day.id, description: description, project: {
            id: 0,
            name: "",
            tasks: []
        }, task: {
            id: 0,
            name: ""
        }}]
    }
    newDay.mutate(newDayData);
  }

  return (
    <Container>
      <div className="flex h-full flex-col items-center justify-around">
        <div>
          <label className="font-bold">
            Description:
            <input value={description} onChange={(e) => setDescription(e.target.value)} />
          </label>
        </div>
        <div>
          <Button
            title={"Save"}
            onClick={() => handleSaveButtonClick()}
            solid
          />
          <Button title={"Cancel"} onClick={() => dayViewNav.toActivities()} />
          {newDay.isError ? <div>Error: {newDay.error}</div> : <div />}
          {newDay.isSuccess ? <div>Success!</div> : <div />}
        </div>
      </div>
    </Container>
  );
}

export default CreateActivity;
