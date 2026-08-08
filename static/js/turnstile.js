// @ts-check

/**
 * Cloudflare Turnstile, rendered explicitly.
 *
 * The widget script is loaded with `?render=explicit&onload=onTurnstileLoad`, so
 * it looks for a *global* callback by that name. ES modules don't create globals,
 * hence the explicit assignment to `window` below.
 */

/** Render every widget on the page that hasn't been rendered yet. */
export function renderTurnstile() {
  const turnstile = /** @type {any} */ (window).turnstile;
  if (!turnstile) return;

  for (const el of document.querySelectorAll('.cf-turnstile')) {
    if (el.hasChildNodes()) continue;
    turnstile.render(el, { sitekey: el.getAttribute('data-sitekey') });
  }
}

export function initTurnstile() {
  /** @type {any} */ (window).onTurnstileLoad = renderTurnstile;

  // htmx swaps can bring new widgets in with the page body.
  htmx.on('htmx:afterSwap', renderTurnstile);
  renderTurnstile();
}
