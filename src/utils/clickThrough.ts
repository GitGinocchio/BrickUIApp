import { invoke } from "@tauri-apps/api/core";
import { debounce } from "./misc";

export interface ClickableRect {
  x: number;
  y: number;
  width: number;
  height: number;
  top: number;
  right: number;
  bottom: number;
  left: number;

  borderRadiusTopLeft: number;
  borderRadiusTopRight: number;
  borderRadiusBottomLeft: number;
  borderRadiusBottomRight: number;
}

let clickableRects: ClickableRect[] = [];

let canvas: HTMLCanvasElement | null = null;
let ctx: CanvasRenderingContext2D | null = null;
let mutationObserver: MutationObserver;
let resizeObserver: ResizeObserver;

export async function startClickableTracking(root: HTMLElement, debugCanvas = false) {
  if (debugCanvas && !canvas) {
    canvas = document.createElement("canvas");
    canvas.id = "debug-canvas";
    canvas.style.position = "absolute";
    canvas.style.top = "0";
    canvas.style.left = "0";
    canvas.style.zIndex = "10000000";
    canvas.style.pointerEvents = "none";
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    document.body.appendChild(canvas);
    ctx = canvas.getContext("2d");
  }

  clickableRects = await getClickableRects(root);
  drawRects();

  resizeObserver = new ResizeObserver(async () => {
    clickableRects = await getClickableRects(root);
    drawRects();
  });

  root.querySelectorAll<HTMLElement>('*').forEach(el => resizeObserver.observe(el));

  mutationObserver = new MutationObserver(async (mutations) => {
    for (const m of mutations) {
      m.addedNodes.forEach(node => {
        if (node instanceof HTMLElement) {
          observeNewElements(node);
        }
      });
    }
    clickableRects = await getClickableRects(root);
    console.log(clickableRects);
    drawRects();
  });

  mutationObserver.observe(root, {
    childList: true,
    subtree: true,
    attributes: true,
    attributeFilter: ['style', 'class'],
  });
}

function drawRects() {
  if (!ctx || !canvas) return;
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  clickableRects.forEach(rect => {
    ctx.fillStyle = "rgba(255,0,0,0.5)";
    ctx.beginPath();
    ctx.roundRect(
      rect.x,
      rect.y,
      rect.width,
      rect.height,
      [
        rect.borderRadiusTopLeft,
        rect.borderRadiusTopRight,
        rect.borderRadiusBottomRight,
        rect.borderRadiusBottomLeft
      ]
    );
    ctx.fill();
  });
}

const updateRectsDebounced = debounce(async (rects: ClickableRect[]) => {
  try {
    await invoke("update_clickable_rects", { rects });
  } catch (e) {
    console.error("Failed to update clickable rects:", e);
  }
}, 100); // 100ms di debounce, puoi regolare

export async function getClickableRects(root: HTMLElement): Promise<ClickableRect[]> {
  const allElements = Array.from(root.querySelectorAll<HTMLElement>('*'));
  const rects = allElements
    .filter(isElementClickable)
    .filter(hasValidRect)
    .map(el => {
      const rect = el.getBoundingClientRect();
      const style = getComputedStyle(el);

      return {
        x: rect.x,
        y: rect.y,
        width: rect.width,
        height: rect.height,
        top: rect.top,
        right: rect.right,
        bottom: rect.bottom,
        left: rect.left,

        borderRadiusTopLeft: parseFloat(style.borderTopLeftRadius) || 0,
        borderRadiusTopRight: parseFloat(style.borderTopRightRadius) || 0,
        borderRadiusBottomLeft: parseFloat(style.borderBottomLeftRadius) || 0,
        borderRadiusBottomRight: parseFloat(style.borderBottomRightRadius) || 0
      } as ClickableRect;
    });

  // filtro i rect completamente contenuti in altri
  const filtered =  rects.filter((rect, i) => !rects.some((other, j) => i !== j &&
    rect.left >= other.left && rect.right <= other.right &&
    rect.top >= other.top && rect.bottom <= other.bottom
  ));

  updateRectsDebounced(filtered);

  return filtered;
}

export function observeNewElements(node: HTMLElement) {
  resizeObserver.observe(node);
  node.querySelectorAll<HTMLElement>('*').forEach(child => resizeObserver.observe(child));
}

export function isElementClickable(el: HTMLElement) {
  const style = getComputedStyle(el);
  return (
    style.pointerEvents !== 'none' &&
    style.display !== 'none' &&
    style.visibility !== 'hidden' &&
    el.offsetParent !== null
  );
}

export function hasValidRect(el: HTMLElement) {
  const r = el.getBoundingClientRect();
  return r.width > 0 && r.height > 0;
}

export function isPointClickable(x: number, y: number) {
  return clickableRects.some(r => x >= r.left && x <= r.right && y >= r.top && y <= r.bottom);
}
