// Shared palette generation utilities for layouts
// Extracted from Layout.astro to avoid duplication

// Simple hex to ARGB conversion (same as material-colors.ts)
export function hexToArgb(hex) {
  const clean = hex.replace(/^#/, '');
  if (clean.length !== 6) throw new Error('Invalid hex color');
  const r = parseInt(clean.slice(0, 2), 16);
  const g = parseInt(clean.slice(2, 4), 16);
  const b = parseInt(clean.slice(4, 6), 16);
  return ((255 << 24) | (r << 16) | (g << 8) | b) >>> 0;
}

export function argbToHex(argb) {
  const r = (argb >> 16) & 0xff;
  const g = (argb >> 8) & 0xff;
  const b = argb & 0xff;
  return `#${((1 << 24) | (r << 16) | (g << 8) | b).toString(16).slice(1)}`;
}

// Simple HCT-like color generation (simplified version of material-colors.ts approach)
export function generateSimplePalette(seedHex, accentHex) {
  const seedArgb = hexToArgb(seedHex);
  const r = (seedArgb >> 16) & 0xff;
  const g = (seedArgb >> 8) & 0xff;
  const b = seedArgb & 0xff;
  
  // Generate complementary color (simple inversion)
  const compR = 255 - r;
  const compG = 255 - g; 
  const compB = 255 - b;
  
  // Generate analogous colors (shifted hues)
  const analogous1 = `#${((1 << 24) | (Math.min(255, r + 30) << 16) | (Math.min(255, g + 20) << 8) | Math.max(0, b - 20)).toString(16).slice(1)}`;
  const analogous2 = `#${((1 << 24) | (Math.max(0, r - 20) << 16) | (Math.min(255, g + 30) << 8) | Math.min(255, b + 20)).toString(16).slice(1)}`;
  
  // Generate tertiary
  const tertiary = accentHex || analogous1;
  
  // Generate Material Design-like palette (simplified)
  const isDark = document.documentElement.classList.contains('dark');
  
  if (isDark) {
    return {
      '--mc-primary': argbToHex(seedArgb),
      '--mc-on-primary': '#000000',
      '--mc-primary-container': `#${((1 << 24) | (Math.min(255, r + 20) << 16) | (Math.min(255, g + 20) << 8) | Math.min(255, b + 20)).toString(16).slice(1)}`,
      '--mc-on-primary-container': '#000000',
      '--mc-secondary': analogous1,
      '--mc-on-secondary': '#000000',
      '--mc-secondary-container': `#${((1 << 24) | (Math.min(255, r + 15) << 16) | (Math.min(255, g + 15) << 8) | Math.min(255, b + 15)).toString(16).slice(1)}`,
      '--mc-on-secondary-container': '#000000',
      '--mc-tertiary': tertiary,
      '--mc-on-tertiary': '#000000',
      '--mc-tertiary-container': `#${((1 << 24) | (Math.min(255, r + 25) << 16) | (Math.min(255, g + 25) << 8) | Math.min(255, b + 25)).toString(16).slice(1)}`,
      '--mc-on-tertiary-container': '#000000',
      '--mc-surface': '#0f1414',
      '--mc-on-surface': '#dfe4e3',
      '--mc-surface-variant': '#234341',
      '--mc-on-surface-variant': '#a0c0bd',
      '--mc-outline': '#6a8a88',
      '--mc-outline-variant': '#234341',
      '--mc-scrim': '#000000',
      '--mc-inverse-surface': '#dfe4e3',
      '--mc-inverse-on-surface': '#2a3130',
      '--mc-inverse-primary': `#${((1 << 24) | (Math.max(0, r - 50) << 16) | (Math.max(0, g - 50) << 8) | Math.max(0, b - 50)).toString(16).slice(1)}`,
    };
  } else {
    return {
      '--mc-primary': argbToHex(seedArgb),
      '--mc-on-primary': '#ffffff',
      '--mc-primary-container': `#${((1 << 24) | (Math.max(0, r - 30) << 16) | (Math.max(0, g - 30) << 8) | Math.max(0, b - 30)).toString(16).slice(1)}`,
      '--mc-on-primary-container': '#ffffff',
      '--mc-secondary': analogous1,
      '--mc-on-secondary': '#ffffff',
      '--mc-secondary-container': `#${((1 << 24) | (Math.max(0, r - 20) << 16) | (Math.max(0, g - 20) << 8) | Math.max(0, b - 20)).toString(16).slice(1)}`,
      '--mc-on-secondary-container': '#ffffff',
      '--mc-tertiary': tertiary,
      '--mc-on-tertiary': '#ffffff',
      '--mc-tertiary-container': `#${((1 << 24) | (Math.max(0, r - 25) << 16) | (Math.max(0, g - 25) << 8) | Math.max(0, b - 25)).toString(16).slice(1)}`,
      '--mc-on-tertiary-container': '#ffffff',
      '--mc-surface': '#ffffff',
      '--mc-on-surface': '#1c1c1c',
      '--mc-surface-variant': '#e7e7e7',
      '--mc-on-surface-variant': '#49454f',
      '--mc-outline': '#79747e',
      '--mc-outline-variant': '#cac4d0',
      '--mc-scrim': '#000000',
      '--mc-inverse-surface': '#313033',
      '--mc-inverse-on-surface': '#f4eff4',
      '--mc-inverse-primary': `#${((1 << 24) | (Math.min(255, r + 50) << 16) | (Math.min(255, g + 50) << 8) | Math.min(255, b + 50)).toString(16).slice(1)}`,
    };
  }
}

// Apply palette to navbar and global colors
export function applyPaletteToNavbar(vars) {
  // Update navbar immediately with colors
  const navbar = document.querySelector('header');
  if (navbar) {
    navbar.style.setProperty('--mc-primary', vars['--mc-primary']);
    navbar.style.setProperty('--mc-on-primary', vars['--mc-on-primary']);
    navbar.style.setProperty('--mc-secondary', vars['--mc-secondary']);
    navbar.style.setProperty('--mc-surface', vars['--mc-surface']);
    navbar.style.setProperty('--mc-surface-variant', vars['--mc-surface-variant']);
    navbar.style.setProperty('--mc-outline', vars['--mc-outline']);
    navbar.style.setProperty('--mc-outline-variant', vars['--mc-outline-variant']);
  }
  
  // Update global colors
  const root = document.documentElement;
  root.style.setProperty('--bg-primary', vars['--mc-primary']);
  root.style.setProperty('--bg-secondary', vars['--mc-secondary'] || vars['--mc-primary']);
  root.style.setProperty('--bg-tertiary', vars['--mc-tertiary'] || vars['--mc-primary-container']);
  root.style.setProperty('--blob-primary', vars['--mc-primary']);
  root.style.setProperty('--blob-secondary', vars['--mc-secondary'] || vars['--mc-primary-container']);
  root.style.setProperty('--blob-tertiary', vars['--mc-tertiary'] || vars['--mc-tertiary-container']);
}

// Update navbar colors from a section (used in scroll handlers)
export function updateNavbarFromSection(section) {
  const computed = getComputedStyle(section);
  const navbar = document.querySelector('header');
  
  if (navbar) {
    navbar.style.setProperty('--mc-primary', computed.getPropertyValue('--mc-primary'));
    navbar.style.setProperty('--mc-on-primary', computed.getPropertyValue('--mc-on-primary'));
    navbar.style.setProperty('--mc-secondary', computed.getPropertyValue('--mc-secondary'));
    navbar.style.setProperty('--mc-surface', computed.getPropertyValue('--mc-surface'));
    navbar.style.setProperty('--mc-surface-variant', computed.getPropertyValue('--mc-surface-variant'));
    navbar.style.setProperty('--mc-outline', computed.getPropertyValue('--mc-outline'));
    navbar.style.setProperty('--mc-outline-variant', computed.getPropertyValue('--mc-outline-variant'));
  }
  
  // Update global colors
  const root = document.documentElement;
  root.style.setProperty('--bg-primary', computed.getPropertyValue('--mc-primary'));
  root.style.setProperty('--bg-secondary', computed.getPropertyValue('--mc-secondary') || computed.getPropertyValue('--mc-primary'));
  root.style.setProperty('--bg-tertiary', computed.getPropertyValue('--mc-tertiary') || computed.getPropertyValue('--mc-primary-container'));
  root.style.setProperty('--blob-primary', computed.getPropertyValue('--mc-primary'));
  root.style.setProperty('--blob-secondary', computed.getPropertyValue('--mc-secondary') || computed.getPropertyValue('--mc-primary-container'));
  root.style.setProperty('--blob-tertiary', computed.getPropertyValue('--mc-tertiary') || computed.getPropertyValue('--mc-tertiary-container'));
}
