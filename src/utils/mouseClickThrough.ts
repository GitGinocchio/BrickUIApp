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
    simulateFakeMouseEvent(screenX, screenY);
  }
}

function simulateFakeMouseEvent(x: number, y: number) {
  const event = new MouseEvent('mousemove', {
    clientX: x,
    clientY: y,
    bubbles: true,
    cancelable: true
  });
  document.dispatchEvent(event);

  // In futuro da fare quando avviene un click globale
  /*
  const clickEvent = new MouseEvent('click', {
    clientX: x,
    clientY: y,
    bubbles: true,
    cancelable: true
  });
  document.dispatchEvent(clickEvent);
  */
}