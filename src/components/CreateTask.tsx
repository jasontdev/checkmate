import Button from "../ui/Button";
import Container from "../ui/Container";
import {type DayViewNav} from "../views/DayView";
import {useState} from "react";
import {useMutation, useQueryClient} from "@tanstack/react-query";
import {invoke} from "@tauri-apps/api/tauri";

type CreateTaskProps = {
    day: Day;
    dayViewNav: DayViewNav;
};

function CreateTask({dayViewNav, day}: CreateTaskProps) {
    const queryClient = useQueryClient();
    const [description, setDescription] = useState("");
    const task = useMutation({
        mutationFn: (newTask: Task) => {
            return invoke<Task>('save_task', {task: newTask});
        },
        onSuccess: () => {
            queryClient.invalidateQueries([`day_${day.date.replace(/ /g, " ")}`])
            dayViewNav.toTasks();
        }
    });

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

        task.mutate(newTask);
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
                    <Button title={"Cancel"} onClick={() => dayViewNav.toTasks()}/>
                </div>
                {task.isSuccess ? <div>Success!</div> : <div>Error...</div>}
            </div>
        </Container>
    );
}

export default CreateTask;
