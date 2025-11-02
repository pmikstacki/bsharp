import { generateSimplePalette, applyPaletteToNavbar } from './palette-utils.js';

(function initDocsPalette(){
  const vars = generateSimplePalette('#00C9B9');
  applyPaletteToNavbar(vars);
})();
