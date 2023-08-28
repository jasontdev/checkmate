import { InvokeArgs, invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";

export default function useBackendQuery<T>(
  queryCommand: string,
  queryParams: InvokeArgs | undefined,
  mutateEvent: string
) {
  const [innerState, setInnerState] = useState<T>();

  // TODO: need to add cleanup
  useEffect(() => {
    invoke(queryCommand, queryParams).then((result) => {
      listen(mutateEvent, (event) => {
        setInnerState(event.payload as T);
      });
      setInnerState(result as T);
    });
  }, [queryCommand, queryParams]);

  return { data: innerState };
}
