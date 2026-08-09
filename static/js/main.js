// @ts-check

/**
 * Entry point.
 *
 * The `?bustCache=` query on this module's own URL is propagated to the imports:
 * a static `import './terminal.js'` would resolve to an unbusted URL and could
 * pair a fresh entry point with a stale sibling after a deploy.
 */
const bust = new URL(import.meta.url).search;

const [{ initTurnstile }, { initTerminal }] = await Promise.all([
  import(`./turnstile.js${bust}`),
  import(`./terminal.js${bust}`),
]);

initTurnstile();
initTerminal();

// ── htmx afterSwap: focus, scroll, and live-region management ──────────────
// After a boosted navigation swaps .snowframe, move focus to the main heading,
// reset scroll, and announce the new page title to screen readers.
htmx.on('htmx:afterSwap', (/** @type {any} */ e) => {
  // Only handle swaps into .snowframe (page content), not #termx (terminal).
  const target = e.detail?.target;
  if (!target || target.id === 'termx') return;
  if (!target.classList?.contains('snowframe')) return;

  // Reset scroll so navigating from deep in a long post lands at the top.
  window.scrollTo(0, 0);

  // Move focus to the first heading inside <main> so keyboard users aren't
  // left focused in the footer while content above changes silently.
  const main = document.getElementById('content');
  if (main) {
    const heading = main.querySelector('h1, h2');
    if (heading) {
      // Make the heading focusable without adding it to the permanent tab order.
      heading.setAttribute('tabindex', '-1');
      /** @type {any} */ (heading).focus({ preventScroll: true });
    } else {
      main.setAttribute('tabindex', '-1');
      /** @type {any} */ (main).focus({ preventScroll: true });
    }
  }

  // Announce the new page title via the polite live region.
  const announcer = document.getElementById('page-announcer');
  if (announcer) {
    announcer.textContent = document.title || '';
  }

  syncNavCurrent();

  // Flickity and FontAwesome are only in <head> on pages that need them, so a
  // boosted navigation into such a page has to bring its own.
  if (target.querySelector('.main-carousel')) ensureFlickity();
  if (target.querySelector('.fa-solid, .fa-brands, .fa-regular')) ensureFontAwesome();
});

/**
 * Re-mark the current nav item after a boosted navigation.
 *
 * The header and footer sit outside `.snowframe`, so a swap never re-renders
 * them and the server-rendered `aria-current` from the *first* page load would
 * otherwise stay put for the rest of the session — landing on /blog and then
 * clicking Home left "Blog" marked as the current page.
 *
 * Mirrors `tmpl::nav_state`: an exact path match is `page`, a path inside the
 * section is `true`.
 */
function syncNavCurrent() {
  const path = window.location.pathname;

  for (const a of document.querySelectorAll('.primary-nav a, footer nav a')) {
    const href = a.getAttribute('href');
    if (!href || !href.startsWith('/')) continue;

    let state = null;
    if (path === href) state = 'page';
    else if (href !== '/' && path.startsWith(`${href}/`)) state = 'true';

    if (state) a.setAttribute('aria-current', state);
    else a.removeAttribute('aria-current');
  }
}

/**
 * Initialise every carousel that isn't already running.
 *
 * Options come from each element's own `data-flickity` attribute — the same
 * source Flickity's auto-init reads on a full page load — so a carousel behaves
 * identically whether the page was loaded directly or swapped in by htmx.
 */
function initCarousels() {
  const Flickity = /** @type {any} */ (window).Flickity;
  if (!Flickity) return;

  for (const el of document.querySelectorAll('.main-carousel')) {
    if (el.classList.contains('flickity-enabled')) continue;
    let options = {};
    try {
      options = JSON.parse(el.getAttribute('data-flickity') || '{}');
    } catch {
      // Malformed attribute: fall back to Flickity's defaults rather than
      // leaving the images stacked with no carousel at all.
    }
    new Flickity(el, options);
  }
}

let flickityLoading = false;

function ensureFlickity() {
  if (/** @type {any} */ (window).Flickity) return initCarousels();
  if (flickityLoading) return;
  flickityLoading = true;

  const link = document.createElement('link');
  link.rel = 'stylesheet';
  link.href = 'https://unpkg.com/flickity@2/dist/flickity.min.css';
  document.head.appendChild(link);

  const script = document.createElement('script');
  script.src = 'https://unpkg.com/flickity@2/dist/flickity.pkgd.min.js';
  script.async = true;
  script.onload = initCarousels;
  document.head.appendChild(script);
}

function ensureFontAwesome() {
  if (document.querySelector('script[src*="kit.fontawesome.com"]')) return;
  const script = document.createElement('script');
  script.src = 'https://kit.fontawesome.com/eabf947950.js';
  script.crossOrigin = 'anonymous';
  document.head.appendChild(script);
}
