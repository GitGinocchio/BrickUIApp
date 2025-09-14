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


  // logica per decidere se abilitare o disabilitare click-through
  const shouldEnable = !elementTagName || elementsToSkip.includes(elementTagName);
  if (shouldEnable !== isClickThroughEnabled) {
    isClickThroughEnabled = shouldEnable;
    await currentWindow.setIgnoreCursorEvents(shouldEnable);
    console.log(`${shouldEnable ? "Enabled" : "Disabled"} click through`);
  }

  // se click-through abilitato, genera eventi finti
  if (isClickThroughEnabled) {
    simulateFakeMouseMoved(screenX, screenY);
  }
}

export function simulateFakeMousePressed(event: { payload: [number, number, string]}) {
  console.log(event);

  let button = null;
  switch (event.payload[2]) {
    case "Left":
      button = 0;
      break;
    case "Middle":
      button = 1;
      break;
    case "Right":
      button = 2;
      break;
    default: 
      button = null;

  }

  const clickEvent = new MouseEvent('click', {
    clientX: event.payload[0],
    clientY: event.payload[1],
    bubbles: true,
    cancelable: true,
    button: button
  });
  document.dispatchEvent(clickEvent);
}

export function simulateFakeMouseMoved(x: number, y: number) {
  const event = new MouseEvent('mousemove', {
    clientX: x,
    clientY: y,
    bubbles: true,
    cancelable: true
  });
  document.dispatchEvent(event);
}