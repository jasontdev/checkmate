import { InvokeArgs, invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";

function useQuery<T>(
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

// TODO: needs cleanup function
function useMutation<T>(mutateCommand: string) {
  const [error, setError] = useState<string>();
  const [data, setData] = useState<T>();
  const [isSuccess, setIsSuccess] = useState(false);
  const [isError, setIsError] = useState(false);

  const mutate = (mutationData: InvokeArgs) => {
    invoke(mutateCommand, mutationData).then((result) => {
      setData(result as T);
      setIsSuccess(true);
    }).catch((error) => { setError(error); setIsError(true)});
  }

  return { data, error, isSuccess, isError, mutate} ;
}

export { useMutation, useQuery };
