/**
 * Main JavaScript for Commi Website
 * Modern modular architecture with proper error handling
 */

// For browsers that don't support modules, we'll use a fallback approach
class CommiWebsite {
  constructor() {
    this.themeManager = null;
    this.navigationManager = null;
    this.isInitialized = false;

    this.init();
  }

  /**
   * Initialize the website
   */
  async init() {
    try {
      // Wait for DOM to be ready
      if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', () => this.initializeModules());
      } else {
        this.initializeModules();
      }
    } catch (error) {
      console.error('Failed to initialize website:', error);
    }
  }

  /**
   * Initialize all modules
   */
  initializeModules() {
    try {
      // Initialize features
      this.initThemeToggle();
      this.initMobileMenu();
      this.initCopyButtons();
      this.initOSDetection();
      this.initAnimations();
      this.initErrorHandling();
      this.initServiceWorker();
      this.initPerformanceOptimizations();

      this.isInitialized = true;
      console.log('Commi website initialized successfully');

      // Dispatch custom event
      document.dispatchEvent(new CustomEvent('websiteInitialized'));

    } catch (error) {
      console.error('Failed to initialize modules:', error);
    }
  }

  /**
   * Initialize theme toggle functionality
   */
  initThemeToggle() {
    const themeToggle = document.getElementById('theme-toggle');
    const darkIcon = document.getElementById('theme-toggle-dark-icon');
    const lightIcon = document.getElementById('theme-toggle-light-icon');

    if (!themeToggle) return;

    // Get initial theme
    const getInitialTheme = () => {
      const savedTheme = localStorage.getItem('theme');
      if (savedTheme) return savedTheme;
      return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
    };

    // Apply theme
    const applyTheme = (theme) => {
      const isDark = theme === 'dark';
      document.documentElement.classList.toggle('dark', isDark);

      if (darkIcon && lightIcon) {
        darkIcon.classList.toggle('hidden', isDark);
        lightIcon.classList.toggle('hidden', !isDark);
      }

      localStorage.setItem('theme', theme);
    };

    // Initialize theme
    applyTheme(getInitialTheme());

    // Toggle theme on click
    themeToggle.addEventListener('click', () => {
      const currentTheme = document.documentElement.classList.contains('dark') ? 'dark' : 'light';
      const newTheme = currentTheme === 'dark' ? 'light' : 'dark';
      applyTheme(newTheme);
    });

    // Watch system theme changes
    window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
      if (!localStorage.getItem('theme')) {
        applyTheme(e.matches ? 'dark' : 'light');
      }
    });
  }

  /**
   * Initialize mobile menu
   */
  initMobileMenu() {
    const mobileMenuButton = document.getElementById('mobile-menu-button');
    const mobileMenu = document.getElementById('mobile-menu');

    if (!mobileMenuButton || !mobileMenu) return;

    let isMenuOpen = false;

    const toggleMenu = () => {
      isMenuOpen = !isMenuOpen;
      mobileMenu.classList.toggle('hidden', !isMenuOpen);
      mobileMenuButton.setAttribute('aria-expanded', isMenuOpen.toString());
      document.body.style.overflow = isMenuOpen ? 'hidden' : '';
    };

    const closeMenu = () => {
      if (isMenuOpen) {
        isMenuOpen = false;
        mobileMenu.classList.add('hidden');
        mobileMenuButton.setAttribute('aria-expanded', 'false');
        document.body.style.overflow = '';
      }
    };

    // Toggle on button click
    mobileMenuButton.addEventListener('click', toggleMenu);

    // Close on outside click
    document.addEventListener('click', (e) => {
      if (isMenuOpen && !mobileMenu.contains(e.target) && !mobileMenuButton.contains(e.target)) {
        closeMenu();
      }
    });

    // Close on escape key
    document.addEventListener('keydown', (e) => {
      if (e.key === 'Escape' && isMenuOpen) {
        closeMenu();
      }
    });

    // Close on link click
    mobileMenu.addEventListener('click', (e) => {
      if (e.target.tagName === 'A') {
        closeMenu();
      }
    });
  }

  /**
   * Initialize copy buttons for code blocks
   */
  initCopyButtons() {
    // Handle existing copy buttons with data-copy attribute
    const copyButtons = document.querySelectorAll('.copy-btn[data-copy]');

    copyButtons.forEach(button => {
      button.addEventListener('click', async () => {
        const textToCopy = button.getAttribute('data-copy');
        const success = await this.copyToClipboard(textToCopy);

        if (success) {
          const originalText = button.textContent;
          button.textContent = 'Copied!';
          button.classList.add('bg-green-500');
          this.showToast('Command copied to clipboard!', 'success');

          setTimeout(() => {
            button.textContent = originalText;
            button.classList.remove('bg-green-500');
          }, 2000);
        } else {
          this.showToast('Failed to copy command', 'error');
        }
      });
    });

    // Handle legacy code blocks without copy buttons
    const codeBlocks = document.querySelectorAll('pre code, .code-block code');

    codeBlocks.forEach(codeBlock => {
      const container = codeBlock.closest('pre') || codeBlock.closest('.code-block');

      // Skip if container already has a copy button
      if (container && !container.querySelector('.copy-btn')) {
        const button = this.createCopyButton();
        container.style.position = 'relative';
        container.appendChild(button);

        button.addEventListener('click', async () => {
          const success = await this.copyToClipboard(codeBlock.textContent);

          if (success) {
            button.textContent = 'Copied!';
            button.classList.add('bg-green-500');
            this.showToast('Code copied to clipboard!', 'success');

            setTimeout(() => {
              button.textContent = 'Copy';
              button.classList.remove('bg-green-500');
            }, 2000);
          } else {
            this.showToast('Failed to copy code', 'error');
          }
        });
      }
    });
  }

  /**
   * Create copy button element
   */
  createCopyButton() {
    const button = document.createElement('button');
    button.textContent = 'Copy';
    button.className = 'absolute top-2 right-2 px-3 py-1 bg-gray-700 hover:bg-gray-600 text-white text-sm rounded transition-colors';
    button.setAttribute('aria-label', 'Copy code to clipboard');
    return button;
  }

  /**
   * Copy text to clipboard
   */
  async copyToClipboard(text) {
    try {
      if (navigator.clipboard && window.isSecureContext) {
        await navigator.clipboard.writeText(text);
        return true;
      } else {
        // Fallback for older browsers
        const textArea = document.createElement('textarea');
        textArea.value = text;
        textArea.style.position = 'fixed';
        textArea.style.left = '-999999px';
        document.body.appendChild(textArea);
        textArea.focus();
        textArea.select();

        const success = document.execCommand('copy');
        document.body.removeChild(textArea);
        return success;
      }
    } catch (error) {
      console.error('Failed to copy text:', error);
      return false;
    }
  }

  /**
   * Initialize OS detection for installation instructions
   */
  initOSDetection() {
    const osElements = document.querySelectorAll('[data-os]');
    if (osElements.length === 0) return;

    const detectedOS = this.detectOS();

    osElements.forEach(element => {
      const targetOS = element.getAttribute('data-os');
      if (targetOS === detectedOS.toLowerCase()) {
        element.classList.remove('hidden');
        element.classList.add('block');
      }
    });
  }

  /**
   * Detect user's operating system
   */
  detectOS() {
    const userAgent = window.navigator.userAgent;
    const platform = window.navigator.platform;
    const macosPlatforms = ['Macintosh', 'MacIntel', 'MacPPC', 'Mac68K'];
    const windowsPlatforms = ['Win32', 'Win64', 'Windows', 'WinCE'];

    if (macosPlatforms.indexOf(platform) !== -1) {
      return 'macOS';
    } else if (windowsPlatforms.indexOf(platform) !== -1) {
      return 'Windows';
    } else if (/Linux/.test(platform)) {
      return 'Linux';
    }

    return 'Linux'; // Default to Linux for unknown systems
  }

  /**
   * Initialize scroll animations
   */
  initAnimations() {
    const animatedElements = document.querySelectorAll('.animate-on-scroll');

    if (animatedElements.length === 0) return;

    const observer = new IntersectionObserver((entries) => {
      entries.forEach(entry => {
        if (entry.isIntersecting) {
          entry.target.classList.add('animate-fade-in-up');
          observer.unobserve(entry.target);
        }
      });
    }, {
      threshold: 0.1,
      rootMargin: '0px 0px -50px 0px'
    });

    animatedElements.forEach(element => {
      observer.observe(element);
    });
  }

  /**
   * Initialize global error handling
   */
  initErrorHandling() {
    window.addEventListener('error', (event) => {
      console.error('Global error:', event.error);
    });

    window.addEventListener('unhandledrejection', (event) => {
      console.error('Unhandled promise rejection:', event.reason);
    });
  }

  /**
   * Show toast notification
   */
  showToast(message, type = 'info', duration = 3000) {
    const toast = document.createElement('div');
    toast.className = `fixed top-4 right-4 px-6 py-3 rounded-lg shadow-lg z-50 transition-all duration-300 transform translate-x-full`;

    const typeStyles = {
      success: 'bg-green-500 text-white',
      error: 'bg-red-500 text-white',
      warning: 'bg-yellow-500 text-black',
      info: 'bg-blue-500 text-white'
    };

    toast.className += ` ${typeStyles[type] || typeStyles.info}`;
    toast.textContent = message;

    document.body.appendChild(toast);

    setTimeout(() => toast.classList.remove('translate-x-full'), 10);

    setTimeout(() => {
      toast.classList.add('translate-x-full');
      setTimeout(() => toast.remove(), 300);
    }, duration);
  }

  /**
   * Initialize service worker for offline functionality
   */
  async initServiceWorker() {
    if ('serviceWorker' in navigator) {
      try {
        const registration = await navigator.serviceWorker.register('/sw.js');
        console.log('Service Worker registered successfully:', registration);

        // Listen for updates
        registration.addEventListener('updatefound', () => {
          const newWorker = registration.installing;
          newWorker.addEventListener('statechange', () => {
            if (newWorker.state === 'installed' && navigator.serviceWorker.controller) {
              this.showToast('New version available! Refresh to update.', 'info', 5000);
            }
          });
        });

      } catch (error) {
        console.warn('Service Worker registration failed:', error);
      }
    }
  }

  /**
   * Initialize performance optimizations
   */
  initPerformanceOptimizations() {
    // Preload critical resources
    this.preloadCriticalResources();

    // Lazy load images
    this.initLazyLoading();

    // Optimize font loading
    this.optimizeFontLoading();

    // Monitor Core Web Vitals
    this.monitorWebVitals();
  }

  /**
   * Preload critical resources
   */
  preloadCriticalResources() {
    const criticalResources = [
      { href: '/css/styles.css', as: 'style' },
      { href: '/js/main.js', as: 'script' },
      { href: 'https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700&display=swap', as: 'style' }
    ];

    criticalResources.forEach(resource => {
      const link = document.createElement('link');
      link.rel = 'preload';
      link.href = resource.href;
      link.as = resource.as;
      if (resource.as === 'style') {
        link.onload = () => { link.rel = 'stylesheet'; };
      }
      document.head.appendChild(link);
    });
  }

  /**
   * Initialize lazy loading for images
   */
  initLazyLoading() {
    if ('IntersectionObserver' in window) {
      const imageObserver = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
          if (entry.isIntersecting) {
            const img = entry.target;
            img.src = img.dataset.src;
            img.classList.remove('lazy');
            imageObserver.unobserve(img);
          }
        });
      });

      document.querySelectorAll('img[data-src]').forEach(img => {
        imageObserver.observe(img);
      });
    }
  }

  /**
   * Optimize font loading
   */
  optimizeFontLoading() {
    // Use font-display: swap for better performance
    const fontLink = document.querySelector('link[href*="fonts.googleapis.com"]');
    if (fontLink && !fontLink.href.includes('display=swap')) {
      fontLink.href += '&display=swap';
    }
  }

  /**
   * Monitor Core Web Vitals
   */
  monitorWebVitals() {
    // Monitor Largest Contentful Paint (LCP)
    if ('PerformanceObserver' in window) {
      try {
        const lcpObserver = new PerformanceObserver((list) => {
          const entries = list.getEntries();
          const lastEntry = entries[entries.length - 1];
          console.log('LCP:', lastEntry.startTime);
        });
        lcpObserver.observe({ entryTypes: ['largest-contentful-paint'] });

        // Monitor First Input Delay (FID)
        const fidObserver = new PerformanceObserver((list) => {
          const entries = list.getEntries();
          entries.forEach(entry => {
            console.log('FID:', entry.processingStart - entry.startTime);
          });
        });
        fidObserver.observe({ entryTypes: ['first-input'] });

        // Monitor Cumulative Layout Shift (CLS)
        const clsObserver = new PerformanceObserver((list) => {
          let clsValue = 0;
          const entries = list.getEntries();
          entries.forEach(entry => {
            if (!entry.hadRecentInput) {
              clsValue += entry.value;
            }
          });
          console.log('CLS:', clsValue);
        });
        clsObserver.observe({ entryTypes: ['layout-shift'] });

      } catch (error) {
        console.warn('Performance monitoring failed:', error);
      }
    }
  }
}

// Initialize the website
new CommiWebsite();
