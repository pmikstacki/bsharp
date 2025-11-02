import { generateSimplePalette, applyPaletteToNavbar, updateNavbarFromSection } from './palette-utils.js';

function applyFirstPalette() {
  const heroSection = document.querySelector('section[data-mc-seed="#00C9B9"]') as HTMLElement | null;
  if (heroSection) {
    const vars = generateSimplePalette('#00C9B9');
    Object.entries(vars).forEach(([k, v]) => heroSection.style.setProperty(k, v as string));
    applyPaletteToNavbar(vars);
  }
}

function updateFabPalette() {
  const fab = document.querySelector('.fab') as HTMLElement | null;
  if (!fab) return;

  const hero = document.querySelector('section[data-mc-seed="#00C9B9"]') as HTMLElement | null;
  if (hero) {
    const heroRect = hero.getBoundingClientRect();
    if (heroRect && heroRect.bottom > window.innerHeight * 0.8) {
      fab.style.display = 'none';
      return;
    }
  }
  fab.style.display = '';

  const sections = Array.from(document.querySelectorAll<HTMLElement>('section[data-mc-seed]'));
  let closest: HTMLElement | null = null;
  let minDist = Infinity;
  sections.forEach(sec => {
    const rect = sec.getBoundingClientRect();
    const dist = Math.abs(rect.top + rect.height / 2 - window.innerHeight / 2);
    if (dist < minDist) {
      minDist = dist;
      closest = sec;
    }
  });
  if (closest) {
    updateNavbarFromSection(closest);
  }
}

function boot() {
  applyFirstPalette();
  updateFabPalette();
  window.addEventListener('scroll', updateFabPalette, { passive: true });
  window.addEventListener('resize', updateFabPalette, { passive: true });
  document.addEventListener('material-colors-ready', updateFabPalette);
  const mo = new MutationObserver(updateFabPalette);
  mo.observe(document.documentElement, { attributes: true, attributeFilter: ['class'] });
}

if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', boot);
} else {
  boot();
}
