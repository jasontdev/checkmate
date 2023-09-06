import {useState} from "react";

enum Views {
  DayView,
}

type AppNav = {
  toDayView: () => void;
};

function App() {
  const [date, setDate] = useState<string>(new Date(Date.now()).toDateString());
  const [currentView] = useState<Views>(Views.DayView);

  if (currentView === Views.DayView) {
    return (
      <div className="h-screen w-screen">
      </div>
    );
  }
}

export { App, type AppNav };
