import mermaid from 'mermaid';
import svgPanZoom from 'svg-pan-zoom';

mermaid.initialize({
  startOnLoad: false,
  securityLevel: 'strict',
  theme: 'default',
  flowchart: {
    useMaxWidth: true,
    htmlLabels: true,
    curve: 'basis',
  },
});

export async function renderMermaid(target: HTMLElement, id: string, code: string) {
  const { svg } = await mermaid.render(id, code);
  target.innerHTML = svg;
  const svgEl = target.querySelector('svg') as SVGSVGElement;
  if (svgEl) {
    // Remove Mermaid's hardcoded max-width to allow full container fill
    svgEl.style.maxWidth = '';
    svgEl.style.width = '100%';
    svgEl.style.height = '460px';
    svgPanZoom(svgEl, {
      zoomEnabled: true,
      controlIconsEnabled: true,
      fit: true,
      center: true,
      minZoom: 0.1,
      maxZoom: 10,
    });
  }
}
