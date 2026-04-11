import { type Event } from "@tauri-apps/api/event";
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
    oldSimulateFakeMouseMoved(element, screenX, screenY);
  }
}

export function simulateFakeMousePressed(event: {
  payload: [number, number, string];
}) {
  let button = 0;
  let buttons = 0;

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
  }

  const [x, y] = event.payload;
  const element = document.elementFromPoint(x, y);
  if (!element || element instanceof HTMLButtonElement) return;

  console.log(`Sending click to element: ${element}`);

  ["mousedown", "mouseup", "click"].forEach((type) => {
    const evt = new MouseEvent(type, {
      clientX: x,
      clientY: y,
      screenX: x,
      screenY: y,
      bubbles: true,
      cancelable: true,
      button,
      buttons,
    });
    element.dispatchEvent(evt);
  });
}

export function oldSimulateFakeMouseMoved(element: Element, x: number, y: number) {
  ["mousemove", "mouseover"].forEach((type) => {
    const event = new MouseEvent(type, {
      clientX: x,
      clientY: y,
      bubbles: true,
      cancelable: true,
    });
    (element ?? document).dispatchEvent(event);
  });
}

export function simulateFakeMouseMoved(x: number, y: number) {
  const element = document.elementFromPoint(x, y);
  if (!element) return;

  ["mousemove", "mouseover"].forEach((type) => {
    const event = new MouseEvent(type, {
      clientX: x,
      clientY: y,
      bubbles: true,
      cancelable: true,
    });
    (element ?? document).dispatchEvent(event);
  });
}
