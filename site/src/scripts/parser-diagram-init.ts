import { renderMermaid } from './mermaid-setup.ts';

function baseUrl() {
  const raw = import.meta.env.BASE_URL || '/';
  return raw.endsWith('/') ? raw : `${raw}/`;
}

function wasmUrl() {
  const b = baseUrl();
  return new URL(`${b}wasm/bsharp_wasm.js`, window.location.origin).toString();
}

const $ = (s: string) => document.querySelector(s) as HTMLElement | null;
const btn = $('#btn');
const target = $('#m-target');

function getEditor(){
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  return (window as any).__monacoEditors?.['parser-monaco'];
}

async function ensureEditor(){
  const start = Date.now();
  while (!getEditor() && Date.now() - start < 2000) await new Promise(r => setTimeout(r, 50));
  if (!getEditor()) console.warn('Monaco editor instance not found after timeout');
  return getEditor();
}

btn?.addEventListener('click', async () => {
  try {
    const mod: any = await import(wasmUrl());
    if (mod && mod.default) await mod.default();
    const ed: any = await ensureEditor();
    const diag: string = await mod.ast_to_mermaid(ed.getValue());
    const id = 'ast-' + Math.random().toString(36).slice(2);
    if (target) await renderMermaid(target, id, diag);
  } catch (e: any) {
    if (target) target.textContent = 'Mermaid render error: ' + (e?.message || String(e));
  }
});

window.addEventListener('load', () => btn?.click());
