import Button from "../ui/Button";
import Container from "../ui/Container";
import {type DayViewNav} from "../views/DayView";
import {useMutation} from "../api/backend";
import {useState} from "react";

type CreateTaskProps = {
    day: Day;
    dayViewNav: DayViewNav;
};

function CreateTask({dayViewNav, day}: CreateTaskProps) {
    const [description, setDescription] = useState("");

    function handleSaveButtonClick() {
        const newTask: Task = {
            dayId: day.id, description: description,
            category: {
                id: 0,
                name: ""
            },
            project: {
                id: 0,
                name: ""
            },
            id: day.id
        }
    }

    console.log("updating...");

    return (
        <Container>
            <div className="flex h-full flex-col items-center justify-around">
                <div>
                    <label className="font-bold">
                        Description:
                        <input value={description} onChange={(e) => setDescription(e.target.value)}/>
                    </label>
                </div>
                <div>
                    <Button
                        title={"Save"}
                        onClick={() => handleSaveButtonClick()}
                        solid
                    />
                    <Button title={"Cancel"} onClick={() => dayViewNav.toActivities()}/>
                </div>
            </div>
        </Container>
    );
}

export default CreateTask;
