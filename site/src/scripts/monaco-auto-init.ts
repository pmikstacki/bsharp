import { createCSharpEditor } from './monaco-setup';

declare global {
  interface Window { __monacoEditors?: Record<string, any>; }
}

function initOne(el: HTMLElement) {
  const id = el.getAttribute('id') || `monaco-${Math.random().toString(36).slice(2)}`;
  el.id = id;
  const initial = el.getAttribute('data-initial') || '';
  const editor = createCSharpEditor(el, initial);
  window.__monacoEditors = window.__monacoEditors || {};
  window.__monacoEditors[id] = editor;
}

function initAll() {
  document.querySelectorAll<HTMLElement>('[data-monaco="true"]').forEach((el) => {
    if ((el as any).__monacoInit) return;
    (el as any).__monacoInit = true;
    initOne(el);
  });
}

if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', initAll);
} else {
  initAll();
}
