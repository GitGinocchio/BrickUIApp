import * as Vue from "./vendor/vue.esm-browser.prod.js"; 
import { initBridge, getFetchResult, getFileContent, convertFileSrc } from "./bridge.js";
import { loadBrickComponent, bricksState, toggleBrick, updateBrick } from "./bricks.js";

const { path: appDataDir, bricks, port, key } = await initBridge({
  "get-element-from-point" : (event) => onElementFromPointRequested(event),
  "toggle-brick": (event) => toggleBrick(event.data.payload.brick),
  "update-brick": (event) => updateBrick(event.data.payload.name, event.data.payload.prop_name, event.data.payload.prop_value)
});

function onElementFromPointRequested(event) {
  const element = document.elementFromPoint(event.data.payload.x, event.data.payload.y);
  const response = {
    key: key,
    type: "element-from-point",
    payload: {
      element: {
        tagName: element?.tagName || null,
        id: element?.id || null,
        classList: element ? Array.from(element.classList) : [],
        rect: element ? element.getBoundingClientRect() : null
      }
    }
  }

  port.postMessage(response);
}

async function init() {
  const app = Vue.createApp({
    render() {
      const nodes = [];
      for (const [name, { component, props }] of bricksState) {
        nodes.push(Vue.h(component, { key: name, ...props }));
      }
      return Vue.h('div', nodes);
    }
  });

  app.mount('#sandbox');

  for (const brick of bricks) {
    if (brick.enabled) await loadBrickComponent(brick);
  }
}

await init();