/**
 * Theme Management Module
 * Handles dark/light mode switching and persistence
 */

class ThemeManager {
  constructor() {
    this.themeToggle = null;
    this.darkIcon = null;
    this.lightIcon = null;
    this.currentTheme = this.getInitialTheme();
    
    this.init();
  }

  /**
   * Initialize theme manager
   */
  init() {
    this.bindElements();
    this.applyTheme(this.currentTheme);
    this.bindEvents();
    this.watchSystemTheme();
  }

  /**
   * Bind DOM elements
   */
  bindElements() {
    this.themeToggle = document.getElementById('theme-toggle');
    this.darkIcon = document.getElementById('theme-toggle-dark-icon');
    this.lightIcon = document.getElementById('theme-toggle-light-icon');

    if (!this.themeToggle) {
      console.warn('Theme toggle button not found');
      return;
    }
  }

  /**
   * Get initial theme based on saved preference or system preference
   */
  getInitialTheme() {
    const savedTheme = localStorage.getItem('theme');
    if (savedTheme) {
      return savedTheme;
    }

    const systemPrefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    return systemPrefersDark ? 'dark' : 'light';
  }

  /**
   * Apply theme to document
   */
  applyTheme(theme) {
    const isDark = theme === 'dark';
    
    // Update document class
    document.documentElement.classList.toggle('dark', isDark);
    
    // Update icons
    if (this.darkIcon && this.lightIcon) {
      this.darkIcon.classList.toggle('hidden', isDark);
      this.lightIcon.classList.toggle('hidden', !isDark);
    }

    // Save to localStorage
    localStorage.setItem('theme', theme);
    this.currentTheme = theme;

    // Dispatch custom event
    document.dispatchEvent(new CustomEvent('themeChanged', {
      detail: { theme, isDark }
    }));
  }

  /**
   * Toggle between light and dark themes
   */
  toggleTheme() {
    const newTheme = this.currentTheme === 'dark' ? 'light' : 'dark';
    this.applyTheme(newTheme);
  }

  /**
   * Bind event listeners
   */
  bindEvents() {
    if (this.themeToggle) {
      this.themeToggle.addEventListener('click', () => {
        this.toggleTheme();
      });

      // Add keyboard support
      this.themeToggle.addEventListener('keydown', (e) => {
        if (e.key === 'Enter' || e.key === ' ') {
          e.preventDefault();
          this.toggleTheme();
        }
      });
    }
  }

  /**
   * Watch for system theme changes
   */
  watchSystemTheme() {
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
    
    mediaQuery.addEventListener('change', (e) => {
      // Only auto-switch if user hasn't manually set a preference
      if (!localStorage.getItem('theme')) {
        this.applyTheme(e.matches ? 'dark' : 'light');
      }
    });
  }

  /**
   * Get current theme
   */
  getCurrentTheme() {
    return this.currentTheme;
  }

  /**
   * Set theme programmatically
   */
  setTheme(theme) {
    if (theme === 'dark' || theme === 'light') {
      this.applyTheme(theme);
    } else {
      console.warn('Invalid theme:', theme);
    }
  }
}

// Export for use in other modules
export default ThemeManager;
