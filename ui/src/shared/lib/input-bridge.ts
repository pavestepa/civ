import { INPUT_FRAME_OP, postUiEvent, type InputFrame } from "@/shared/api";

const pressed = new Set<string>();
let scrollDelta = 0;
let mouseClick = false;
let mouseX = 0;
let mouseY = 0;
let rafId = 0;

function isEditableTarget(target: EventTarget | null): boolean {
  return (
    target instanceof HTMLInputElement ||
    target instanceof HTMLTextAreaElement ||
    target instanceof HTMLSelectElement
  );
}

function isHudTarget(target: EventTarget | null): boolean {
  if (!(target instanceof Element)) {
    return false;
  }
  return target.closest(".hud-top, .hud-bottom, .hud-log") !== null;
}

function postFrame() {
  const frame: InputFrame = {
    keys: Array.from(pressed),
    scroll_delta: scrollDelta,
    mouse_click: mouseClick,
    mouse_x: mouseX,
    mouse_y: mouseY,
  };
  scrollDelta = 0;
  mouseClick = false;
  postUiEvent(INPUT_FRAME_OP, frame);
}

function onKeyDown(event: KeyboardEvent) {
  if (isEditableTarget(event.target)) {
    return;
  }
  pressed.add(event.code);
}

function onKeyUp(event: KeyboardEvent) {
  pressed.delete(event.code);
}

function onBlur() {
  pressed.clear();
}

function onWheel(event: WheelEvent) {
  if (isEditableTarget(event.target)) {
    return;
  }
  scrollDelta += event.deltaY;
  event.preventDefault();
}

function onMouseDown(event: MouseEvent) {
  if (event.button !== 0 || isHudTarget(event.target)) {
    return;
  }
  mouseClick = true;
  mouseX = event.clientX / window.innerWidth;
  mouseY = event.clientY / window.innerHeight;
}

function tick() {
  postFrame();
  rafId = requestAnimationFrame(tick);
}

export function startInputBridge() {
  window.addEventListener("keydown", onKeyDown);
  window.addEventListener("keyup", onKeyUp);
  window.addEventListener("blur", onBlur);
  window.addEventListener("wheel", onWheel, { passive: false });
  window.addEventListener("mousedown", onMouseDown);

  document.body.tabIndex = -1;
  document.body.focus({ preventScroll: true });

  cancelAnimationFrame(rafId);
  rafId = requestAnimationFrame(tick);

  return () => {
    window.removeEventListener("keydown", onKeyDown);
    window.removeEventListener("keyup", onKeyUp);
    window.removeEventListener("blur", onBlur);
    window.removeEventListener("wheel", onWheel);
    window.removeEventListener("mousedown", onMouseDown);
    cancelAnimationFrame(rafId);
    pressed.clear();
    scrollDelta = 0;
    mouseClick = false;
    postFrame();
  };
}
