import { argbFromHex, hexFromArgb, themeFromSourceColor, Hct, Blend } from '@material/material-color-utilities';

interface McSection extends HTMLElement {
  dataset: {
    mcSeed?: string;
    mcAccent?: string;
  };
}

function hexToArgb(hex: string): number {
  const clean = hex.replace(/^#/, '');
  if (clean.length !== 6) throw new Error('Invalid hex color');
  const r = parseInt(clean.slice(0, 2), 16);
  const g = parseInt(clean.slice(2, 4), 16);
  const b = parseInt(clean.slice(4, 6), 16);
  return ((255 << 24) | (r << 16) | (g << 8) | b) >>> 0;
}

function argbToHex(argb: number): string {
  const r = (argb >> 16) & 0xff;
  const g = (argb >> 8) & 0xff;
  const b = argb & 0xff;
  return `#${((1 << 24) | (r << 16) | (g << 8) | b).toString(16).slice(1)}`;
}

// Fruit salad algorithm: blend complementary and analogous colors for richer palettes
function generateFruitSaladPalette(seedHex: string, accentHex?: string) {
  const seedArgb = hexToArgb(seedHex);
  const seedHct = Hct.fromInt(seedArgb);
  
  // Generate complementary color (180 degrees opposite)
  const complementaryHue = (seedHct.hue + 180) % 360;
  const complementary = Hct.from(complementaryHue, seedHct.chroma * 0.8, seedHct.tone);
  
  // Generate analogous colors (30 degrees offset)
  const analogous1 = Hct.from((seedHct.hue + 30) % 360, seedHct.chroma * 0.9, seedHct.tone);
  const analogous2 = Hct.from((seedHct.hue - 30 + 360) % 360, seedHct.chroma * 0.9, seedHct.tone);
  
  // Generate triadic colors (120 degrees offset)
  const triadic1 = Hct.from((seedHct.hue + 120) % 360, seedHct.chroma * 0.7, seedHct.tone);
  const triadic2 = Hct.from((seedHct.hue + 240) % 360, seedHct.chroma * 0.7, seedHct.tone);
  
  // If accent color provided, blend it with the seed for tertiary
  let tertiaryHct = triadic1;
  if (accentHex) {
    const accentArgb = hexToArgb(accentHex);
    const accentHct = Hct.fromInt(accentArgb);
    const blended = Blend.harmonize(seedArgb, accentArgb);
    tertiaryHct = Hct.fromInt(blended);
  }
  
  // Create enhanced theme with fruit salad colors
  const theme = themeFromSourceColor(seedArgb);
  
  return {
    theme,
    fruitSaladColors: {
      complementary: argbToHex(complementary.toInt()),
      analogous1: argbToHex(analogous1.toInt()),
      analogous2: argbToHex(analogous2.toInt()),
      triadic1: argbToHex(triadic1.toInt()),
      triadic2: argbToHex(triadic2.toInt()),
      customTertiary: argbToHex(tertiaryHct.toInt()),
    }
  };
}

function applyPaletteToSection(section: McSection, seedHex: string, isDark: boolean) {
  const accent = section.dataset.mcAccent;
  const { theme, fruitSaladColors } = generateFruitSaladPalette(seedHex, accent);
  const scheme = isDark ? theme.schemes.dark : theme.schemes.light;

  const vars: Record<string, string> = {
    '--mc-primary': argbToHex(scheme.primary),
    '--mc-on-primary': argbToHex(scheme.onPrimary),
    '--mc-primary-container': argbToHex(scheme.primaryContainer),
    '--mc-on-primary-container': argbToHex(scheme.onPrimaryContainer),
    '--mc-secondary': accent || fruitSaladColors.analogous1,
    '--mc-on-secondary': argbToHex(scheme.onSecondary),
    '--mc-secondary-container': argbToHex(scheme.secondaryContainer),
    '--mc-on-secondary-container': argbToHex(scheme.onSecondaryContainer),
    '--mc-tertiary': fruitSaladColors.customTertiary,
    '--mc-on-tertiary': argbToHex(scheme.onTertiary),
    '--mc-tertiary-container': argbToHex(scheme.tertiaryContainer),
    '--mc-on-tertiary-container': argbToHex(scheme.onTertiaryContainer),
    '--mc-surface': argbToHex(scheme.surface),
    '--mc-on-surface': argbToHex(scheme.onSurface),
    '--mc-surface-variant': argbToHex(scheme.surfaceVariant),
    '--mc-on-surface-variant': argbToHex(scheme.onSurfaceVariant),
    '--mc-outline': argbToHex(scheme.outline),
    '--mc-outline-variant': argbToHex(scheme.outlineVariant),
    '--mc-scrim': argbToHex(scheme.scrim),
    '--mc-inverse-surface': argbToHex(scheme.inverseSurface),
    '--mc-inverse-on-surface': argbToHex(scheme.inverseOnSurface),
    '--mc-inverse-primary': argbToHex(scheme.inversePrimary),
    // Additional fruit salad colors for gradients and accents
    '--mc-complementary': fruitSaladColors.complementary,
    '--mc-analogous1': fruitSaladColors.analogous1,
    '--mc-analogous2': fruitSaladColors.analogous2,
    '--mc-triadic1': fruitSaladColors.triadic1,
    '--mc-triadic2': fruitSaladColors.triadic2,
  };
  Object.entries(vars).forEach(([k, v]) => section.style.setProperty(k, v));
}

function isDarkMode(): boolean {
  return document.documentElement.classList.contains('dark');
}

function updateAllSections() {
  const sections = document.querySelectorAll<McSection>('[data-mc-seed]');
  const isDark = isDarkMode();
  sections.forEach(section => {
    const seed = section.dataset.mcSeed;
    if (seed) applyPaletteToSection(section, seed, isDark);
  });
  
  // Dispatch event to notify that Material Colors are ready
  document.dispatchEvent(new CustomEvent('material-colors-ready'));
}

// Run on DOM ready and observe dark mode changes
if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', updateAllSections);
} else {
  updateAllSections();
}

// Observe dark mode toggles (class changes on html)
const observer = new MutationObserver(() => updateAllSections());
observer.observe(document.documentElement, {
  attributes: true,
  attributeFilter: ['class'],
  subtree: false,
});

export {};
