// @ts-check

/**
 * The v86 terminal panel.
 *
 * One iframe for the life of the page. htmx fetches /termx on the first click of
 * the tty button (`hx-trigger="click once"`); after that, opening and closing only
 * toggles visibility and pauses/resumes the VM. Never clear the panel's innerHTML —
 * that would throw away a booted Arch Linux and re-download 15 MB of state.
 *
 * The frame is sandboxed without `allow-same-origin`, so its origin is opaque: we
 * talk to it over postMessage and identify it by `event.source`, never by origin.
 *
 * Accessibility: the panel is a real dialog. Opening moves focus into the panel and
 * marks the background `inert`; closing restores focus to the trigger. The trigger
 * is never hidden — the fixed panel covers it visually, but it stays in the DOM and
 * focusable so focus is never destroyed.
 */

const panel = () => document.getElementById('termx');
const frame = () =>
  /** @type {HTMLIFrameElement | null} */ (document.getElementById('v86-frame'));
const openButton = () => document.querySelector('.termx-open');
const closeButton = () => document.getElementById('termx-close');

const isOpen = () => {
  const el = panel();
  return !!el && !el.hidden && !!frame();
};

/** @type {Element | null} */ let lastFocus = null;

/** @param {'focus' | 'pause' | 'resume' | 'reset'} action */
const tell = (action) =>
  frame()?.contentWindow?.postMessage({ type: `v86:${action}` }, '*');

/**
 * Mark the background inert so Tab cannot reach it beneath the panel.
 *
 * Every body child except the panel itself, not just `.container` — the skip
 * link and the tty trigger are siblings of `.container`, so scoping this to the
 * container alone would leave two focusable controls hidden underneath a
 * fixed panel that covers them.
 */
function setBackgroundInert(inert) {
  const el = panel();
  for (const child of document.body.children) {
    if (child === el) continue;
    if (inert) child.setAttribute('inert', '');
    else child.removeAttribute('inert');
  }
}

function open() {
  const el = panel();
  if (!el) return;

  // Save current focus so close() can restore it.
  lastFocus = document.activeElement;

  el.hidden = false;
  el.setAttribute('aria-modal', 'true');
  setBackgroundInert(true);

  // Do NOT hide the trigger — the fixed panel covers it visually, but hiding it
  // with display:none would destroy focus if it was focused. Instead, just let
  // the panel overlay it.
  tell('resume');
  tell('focus');

  // Move focus into the dialog — to the close button, which is the first control.
  closeButton()?.focus();

  bindResizeHandle();
}

function close() {
  const el = panel();
  if (!el) return;

  el.hidden = true;
  el.removeAttribute('aria-modal');
  setBackgroundInert(false);
  tell('pause');

  // Restore focus to whatever had it before the panel opened.
  if (lastFocus && /** @type {any} */ (lastFocus).focus) {
    /** @type {any} */ (lastFocus).focus();
  }
  lastFocus = null;
}

function toggle() {
  if (isOpen()) close();
  else if (frame()) open();
  else /** @type {HTMLElement | null} */ (openButton())?.click(); // first open: let htmx fetch
}

/** Messages from the guest frame. */
function onFrameMessage(/** @type {MessageEvent} */ event) {
  if (!frame() || event.source !== frame()?.contentWindow) return;

  switch (event.data?.type) {
    case 'v86:ready':
      // Anything we sent before the frame's document loaded was dropped, so
      // re-assert the panel's current state now that it can hear us.
      tell(isOpen() ? 'resume' : 'pause');
      if (isOpen()) tell('focus');
      break;
    case 'v86:escape':
      close();
      break;
    case 'v86:error':
      console.error('v86 terminal error:', event.data.message);
      close();
      break;
  }
}

/**
 * Drag-to-resize using Pointer Events with setPointerCapture.
 * Bound once, on first open, because the handle arrives with the htmx swap.
 * setPointerCapture means the iframe no longer needs pointer-events:none mid-drag.
 */
function bindResizeHandle() {
  const handle = document.getElementById('resizeHandle');
  const el = panel();
  if (!handle || !el || handle.dataset.bound) return;
  handle.dataset.bound = 'true';

  const MIN_HEIGHT = 220;
  let startY = /** @type {number | null} */ (null);
  let startHeight = 0;

  const start = (/** @type {number} */ y) => {
    startY = y;
    startHeight = el.offsetHeight;
    document.body.classList.add('is-resizing-terminal');
  };

  const move = (/** @type {number} */ y) => {
    if (startY === null) return;
    const max = Math.round(window.innerHeight * 0.85);
    const raw = startHeight + startY - y;

    // Rubber-band resistance at the boundaries: instead of a hard clamp, apply
    // diminishing resistance so hitting a bound reads as the end of travel,
    // not a freeze. Past the bound, each pixel of drag moves the panel half as much.
    let height;
    if (raw < MIN_HEIGHT) {
      height = MIN_HEIGHT - Math.round((MIN_HEIGHT - raw) * 0.5);
    } else if (raw > max) {
      height = max + Math.round((raw - max) * 0.5);
    } else {
      height = raw;
    }
    el.style.height = `${height}px`;
  };

  const stop = () => {
    if (startY === null) return;
    startY = null;
    document.body.classList.remove('is-resizing-terminal');
  };

  handle.addEventListener('pointerdown', (e) => {
    e.preventDefault();
    handle.setPointerCapture(e.pointerId);
    start(e.clientY);
  });

  handle.addEventListener('pointermove', (e) => {
    if (startY === null) return;
    move(e.clientY);
  });

  handle.addEventListener('pointerup', (e) => {
    if (startY === null) return;
    handle.releasePointerCapture(e.pointerId);
    stop();
  });

  // If the pointer is cancelled (e.g. the window loses focus), stop dragging.
  handle.addEventListener('pointercancel', stop);
}

export function initTerminal() {
  // htmx swaps the /termx response into #termx on the very first click.
  htmx.on('htmx:afterSwap', (/** @type {any} */ e) => {
    if (e.detail.target.id === 'termx') open();
  });

  document.addEventListener('click', (event) => {
    const target = /** @type {Element} */ (event.target);
    if (target.closest('#termx-close')) close();
    // Reopening an already-fetched panel; htmx won't re-fire ("click once").
    else if (target.closest('.termx-open') && frame()) open();
  });

  document.addEventListener('keydown', (event) => {
    if ((event.ctrlKey || event.metaKey) && event.key === '`') {
      event.preventDefault();
      toggle();
    }
    // Esc closes the panel when focus is in the parent document. Focus inside
    // the iframe never reaches this listener — the frame has its own
    // double-Esc handler, so a single Esc still gets through to vim.
    if (event.key === 'Escape' && isOpen() && document.activeElement !== frame()) {
      event.preventDefault();
      close();
    }
  });

  window.addEventListener('message', onFrameMessage);
}
