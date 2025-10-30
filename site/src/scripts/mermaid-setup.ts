import mermaid from 'mermaid';

mermaid.initialize({
  startOnLoad: false,
  securityLevel: 'strict',
  theme: 'default',
});

export async function renderMermaid(target: HTMLElement, id: string, code: string) {
  const { svg } = await mermaid.render(id, code);
  target.innerHTML = svg;
}
