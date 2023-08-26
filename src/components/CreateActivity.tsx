import Button from "../ui/Button";
import Container from "../ui/Container";
import {type DayViewNav} from "../views/DayView";

type CreateActivityProps = {
  dayId: number;
  dayViewNav: DayViewNav;
};

function CreateActivity({ dayViewNav }: CreateActivityProps) {
  return (
    <Container>
      <div>
        <Button title={"Save"} onClick={() => dayViewNav.toActivities()} />
      </div>
    </Container>
  );
}

export default CreateActivity;
