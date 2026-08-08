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
 */

const panel = () => document.getElementById('termx');
const frame = () =>
  /** @type {HTMLIFrameElement | null} */ (document.getElementById('v86-frame'));
const openButton = () => document.querySelector('.termx-open');

const isOpen = () => {
  const el = panel();
  return !!el && !el.hidden && !!frame();
};

/** @param {'focus' | 'pause' | 'resume' | 'reset'} action */
const tell = (action) =>
  frame()?.contentWindow?.postMessage({ type: `v86:${action}` }, '*');

function open() {
  const el = panel();
  if (!el) return;

  el.hidden = false;
  openButton()?.classList.add('is-hidden');
  tell('resume');
  tell('focus');
  bindResizeHandle();
}

function close() {
  const el = panel();
  if (!el) return;

  el.hidden = true;
  openButton()?.classList.remove('is-hidden');
  tell('pause');
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
 * Drag-to-resize. Bound once, on first open, because the handle arrives with the
 * htmx swap. While dragging we disable pointer events on the iframe, otherwise it
 * swallows mousemove the moment the cursor crosses into the emulator.
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
    const f = frame();
    if (f) f.style.pointerEvents = 'none';
  };

  const move = (/** @type {number} */ y) => {
    if (startY === null) return;
    const max = Math.round(window.innerHeight * 0.85);
    el.style.height = `${Math.min(max, Math.max(MIN_HEIGHT, startHeight + startY - y))}px`;
  };

  const stop = () => {
    if (startY === null) return;
    startY = null;
    document.body.classList.remove('is-resizing-terminal');
    const f = frame();
    if (f) f.style.pointerEvents = '';
  };

  handle.addEventListener('mousedown', (e) => {
    e.preventDefault();
    start(e.clientY);
  });
  handle.addEventListener('touchstart', (e) => start(e.touches[0].clientY), { passive: true });

  document.addEventListener('mousemove', (e) => move(e.clientY));
  document.addEventListener('touchmove', (e) => move(e.touches[0].clientY), { passive: true });
  document.addEventListener('mouseup', stop);
  document.addEventListener('touchend', stop);
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
  });

  window.addEventListener('message', onFrameMessage);
}
