type Day = {
  id: number;
  date: string;
  tasks: Task[];
};

type Task = {
  id: number;
  dayId: number;
  description: string;
  project: Project;
  category: Category;
};

type Project = {
  id: number;
  name: string;
};

type Category = {
  id: number;
  name: string;
};
