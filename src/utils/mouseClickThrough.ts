import { Event } from "@tauri-apps/api/event";
import { Window } from "@tauri-apps/api/window";

let isClickThroughEnabled = true;
const elementsToSkip = ["HTML", "BODY"];

export async function handleClickThrough(
  event: Event<[number, number]>,
  currentWindow: Window
) {
  const [screenX, screenY] = event.payload;

  let element = document.elementFromPoint(screenX, screenY);
  let elementTagName = element?.tagName;

  // logica per decidere se abilitare o disabilitare click-through
  const shouldEnable =
    !elementTagName || elementsToSkip.includes(elementTagName);
  if (shouldEnable !== isClickThroughEnabled) {
    isClickThroughEnabled = shouldEnable;
    await currentWindow.setIgnoreCursorEvents(shouldEnable);
    //console.log(`${shouldEnable ? "Enabled" : "Disabled"} click through`);
  }

  // se click-through abilitato, genera eventi finti
  if (isClickThroughEnabled && element) {
    simulateFakeMouseMoved(element, screenX, screenY);
  }
}

export function simulateFakeMousePressed(event: {
  payload: [number, number, string];
}) {
  let button = null;
  let buttons = null;
  switch (event.payload[2]) {
    case "Left":
      button = 0;
      buttons = 1;
      break;
    case "Middle":
      button = 1;
      buttons = 4;
      break;
    case "Right":
      button = 2;
      buttons = 2;
      break;
    default:
      button = 0;
      buttons = 0;
  }

  const element = document.elementFromPoint(event.payload[0], event.payload[1]);
  if (!element) return;

  ["mousedown", "mouseup", "click"].forEach((type) => {
    const evt = new MouseEvent(type, {
      clientX: event.payload[0],
      clientY: event.payload[1],
      screenX: event.payload[0],
      screenY: event.payload[1],
      bubbles: true,
      cancelable: true,
      button,
      buttons,
    });
    element.dispatchEvent(evt);
  });
}

export function simulateFakeMouseMoved(element: Element, x: number, y: number) {
  const event = new MouseEvent("mousemove", {
    clientX: x,
    clientY: y,
    bubbles: true,
    cancelable: true,
  });

  (element ?? document).dispatchEvent(event);
}
