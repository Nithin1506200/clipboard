import { useEffect, useState } from "react";
import { registerPoolClipboardListener } from "./events";
import { commands } from "./bindings";

interface PoolClipboard {
  value: boolean;
  changeValue: (newValue: boolean) => void;
}
export function usePoolClipBoardListener(): PoolClipboard {
  const [value, setValue] = useState<boolean>(true);
  useEffect(() => {
    commands.getPoolClipboardState().then((e) => setValue(e));
  }, []);
  useEffect(() => {
    let unlisten: (() => void) | undefined;
    registerPoolClipboardListener((payload) => {
      setValue(payload);
    }).then((_unlisten) => {
      unlisten = _unlisten;
    });

    return () => {
      if (unlisten) unlisten();
    };
  }, []);

  return {
    value,
    changeValue: commands.setPoolClipboardState,
  };
}
