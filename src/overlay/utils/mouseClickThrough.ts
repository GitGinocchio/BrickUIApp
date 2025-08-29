import { Event } from "@tauri-apps/api/event";
import { Window } from "@tauri-apps/api/window";

let isClickThroughEnabled = true;
const elementsToSkip = [
  "HTML",
  "BODY"
]

export async function handleClickThrough(event: Event<[number, number]>, currentWindow: Window) {
  const [screenX, screenY] = event.payload;

  let elementTagName = document.elementFromPoint(screenX, screenY)?.tagName;

  if (elementsToSkip.includes(elementTagName) && !isClickThroughEnabled) {
    isClickThroughEnabled = true;
    await currentWindow.setIgnoreCursorEvents(true);
    console.log("Enabled click through");
    return;
  } else if (elementsToSkip.includes(elementTagName)) return;

  const shouldEnable = !elementTagName;
  if (shouldEnable !== isClickThroughEnabled) {
    isClickThroughEnabled = shouldEnable;
    await currentWindow.setIgnoreCursorEvents(shouldEnable);
    console.log("Disabled click through", elementTagName);
  }
}