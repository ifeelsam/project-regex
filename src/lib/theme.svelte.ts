export type Theme = 'dark' | 'light';

function readStoredTheme(): Theme {
  if (typeof localStorage === 'undefined') return 'dark';
  const stored = localStorage.getItem('regex-theme');
  return stored === 'light' ? 'light' : 'dark';
}

function applyTheme(theme: Theme) {
  if (typeof document === 'undefined') return;
  document.documentElement.classList.toggle('dark', theme === 'dark');
}

export const theme = $state<{ value: Theme }>({ value: readStoredTheme() });

applyTheme(theme.value);

export function setTheme(next: Theme) {
  theme.value = next;
  applyTheme(next);
  localStorage.setItem('regex-theme', next);
}

export function toggleTheme() {
  setTheme(theme.value === 'dark' ? 'light' : 'dark');
}
