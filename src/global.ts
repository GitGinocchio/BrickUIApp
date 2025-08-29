window.addEventListener('contextmenu', (e: MouseEvent) => {
  const target = e.target as HTMLElement;

  // Blocca solo su elementi “generici” come div o span
  const allowedTags = ['IMG', 'A', 'VIDEO', 'CANVAS', 'TEXTAREA', 'INPUT'];
  if (!allowedTags.includes(target.tagName)) {
    e.preventDefault();
  }
});