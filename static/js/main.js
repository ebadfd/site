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
