type Day = {
  date: string;
  activites: Activity[];
};

type Activity = {
  id: number;
  project: string;
  task: string;
  log: ActivityEvent[];
};

enum ActivityState {
  Closed = 0,
  Open = 1,
}

type ActivityEvent = {
  state: ActivityState;
  time: Date;
};
