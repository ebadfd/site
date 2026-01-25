// Turnstile explicit rendering
function renderTurnstile() {
  const containers = document.querySelectorAll('.cf-turnstile');
  containers.forEach((container) => {
    // Only render if not already rendered
    if (!container.hasChildNodes() && typeof turnstile !== 'undefined') {
      turnstile.render(container, {
        sitekey: container.getAttribute('data-sitekey'),
      });
    }
  });
}

// Called when Turnstile script loads
function onTurnstileLoad() {
  renderTurnstile();
}

htmx.on("htmx:afterSwap", (e) => {
  if (e.detail.target.id == "termx") {
    e.detail.target.style.display = '';

    const button = document.querySelector('.termx-open')
    button.style.display = 'none'

    terminal_resize();
  }

  // Re-render Turnstile after HTMX swap
  renderTurnstile();
})


function terminal_resize() {
  const termx = document.getElementById('termx');
  const resizeHandle = document.getElementById('resizeHandle');

  let startY = null;
  let initialHeight = null;

  resizeHandle.addEventListener('mousedown', (event) => {
    startY = event.clientY;
    initialHeight = termx.offsetHeight;
  });

  document.addEventListener('mousemove', (event) => {
    if (startY !== null) {
      const deltaY = startY - event.clientY;
      const newHeight = Math.max(0, initialHeight + deltaY); // Prevent negative heights
      termx.style.height = `${newHeight}px`;
    }
  });

  document.addEventListener('mouseup', () => {
    startY = null;
  });
}

