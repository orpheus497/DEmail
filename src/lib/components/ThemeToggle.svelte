<script lang="ts">
  import { onMount } from 'svelte';
  import { Sun, Moon } from 'lucide-svelte';
  import Button from '$lib/components/ui/button/index.svelte';
  import { saveSetting, getSetting } from '$lib/services/api';

  let theme: 'light' | 'dark' = 'light';
  let loading = true;

  onMount(() => {
    // Load theme from settings asynchronously
    (async () => {
      try {
        const savedTheme = await getSetting('theme');
        if (savedTheme === 'dark' || savedTheme === 'light') {
          theme = savedTheme;
        } else {
          // Check system preference
          const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
          theme = prefersDark ? 'dark' : 'light';
        }
        applyTheme(theme);
      } catch (error) {
        console.error('Failed to load theme:', error);
      } finally {
        loading = false;
      }
    })();

    // Listen for system theme changes
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
    const handleChange = (e: MediaQueryListEvent) => {
      if (!localStorage.getItem('theme')) {
        theme = e.matches ? 'dark' : 'light';
        applyTheme(theme);
      }
    };
    mediaQuery.addEventListener('change', handleChange);

    return () => {
      mediaQuery.removeEventListener('change', handleChange);
    };
  });

  function applyTheme(newTheme: 'light' | 'dark') {
    if (newTheme === 'dark') {
      document.documentElement.classList.add('dark');
    } else {
      document.documentElement.classList.remove('dark');
    }
  }

  async function toggleTheme() {
    theme = theme === 'light' ? 'dark' : 'light';
    applyTheme(theme);

    try {
      await saveSetting('theme', theme);
    } catch (error) {
      console.error('Failed to save theme:', error);
    }
  }
</script>

<Button
  variant="ghost"
  size="sm"
  on:click={toggleTheme}
  disabled={loading}
  aria-label={theme === 'light' ? 'Switch to dark mode' : 'Switch to light mode'}
>
  {#if theme === 'light'}
    <Sun class="h-5 w-5" />
  {:else}
    <Moon class="h-5 w-5" />
  {/if}
</Button>
