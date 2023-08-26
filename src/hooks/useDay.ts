import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";

export function useDay(date: string) {
  const [dayInnerState, setDayInnerState] = useState<Day>({
    id: -1,
    date: "",
    activities: []
  });

  // TODO: need to add cleanup
  useEffect(() => {
    invoke("get_day", { date })
      .then((day) => {
        listen(`day_${date.replace(/ /g, "_")}_updated`, (event) => {
          setDayInnerState(event.payload as Day)
        })
        setDayInnerState(day as Day);
      })
      .catch((error) => console.log(error));
  }, [date]);

  return dayInnerState;
}
