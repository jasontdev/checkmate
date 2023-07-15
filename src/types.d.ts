type Day = {
  id?: number;
  date: string;
  activities?: Activity[];
};

type Activity = {
  id: number;
  dayId: number;
  description: string;
  project: Project;
  task: Task;
};

type Project = {
  id: number;
  name: string;
  tasks: Task[];
};

type Task = {
  id: number;
  name: string;
};
