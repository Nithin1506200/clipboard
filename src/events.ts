import { listen } from "@tauri-apps/api/event";

type PoolClipboardUpdatedPayload = boolean;

async function registerPoolClipboardListener(
  onChange: (payload: PoolClipboardUpdatedPayload) => void
) {
  const unlisten = await listen<PoolClipboardUpdatedPayload>(
    "POOL_CLIPBOARD_UPDATED",
    (event) => {
      onChange(event.payload);
    }
  );

  return unlisten;
}

export { registerPoolClipboardListener };
