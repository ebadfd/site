// @ts-check

/**
 * Cloudflare Turnstile, rendered explicitly.
 *
 * The widget script is loaded with `?render=explicit&onload=onTurnstileLoad`, so
 * it looks for a *global* callback by that name. ES modules don't create globals,
 * hence the explicit assignment to `window` below.
 *
 * Because the Turnstile script is conditionally loaded (only on contact pages in
 * the initial <head>), we also dynamically inject it when a `.cf-turnstile` element
 * appears in an htmx swap on a page that didn't start with the script.
 */

let turnstileScriptLoading = false;

/** Dynamically load the Turnstile API script if it's not already present. */
function ensureTurnstileScript() {
  if (/** @type {any} */ (window).turnstile) return;
  if (turnstileScriptLoading) return;
  if (document.querySelector('script[src*="challenges.cloudflare.com/turnstile"]')) return;

  turnstileScriptLoading = true;
  const s = document.createElement('script');
  s.src = 'https://challenges.cloudflare.com/turnstile/v0/api.js?render=explicit&onload=onTurnstileLoad';
  s.async = true;
  s.defer = true;
  document.head.appendChild(s);
}

/** Render every widget on the page that hasn't been rendered yet. */
export function renderTurnstile() {
  const turnstile = /** @type {any} */ (window).turnstile;
  if (!turnstile) {
    // If there are widgets on the page but the script isn't loaded yet, load it.
    if (document.querySelector('.cf-turnstile')) ensureTurnstileScript();
    return;
  }

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
