/**
 * Component Loader System
 * Loads and injects HTML components into pages
 */

class ComponentLoader {
  constructor() {
    this.cache = new Map();
    this.basePath = this.getBasePath();
  }

  /**
   * Get the base path for component loading based on current location
   */
  getBasePath() {
    const path = window.location.pathname;
    if (path.includes('/pages/')) {
      return '../components/';
    }
    return './components/';
  }

  /**
   * Load a component from file
   */
  async loadComponent(name) {
    if (this.cache.has(name)) {
      return this.cache.get(name);
    }

    try {
      const response = await fetch(`${this.basePath}${name}.html`);
      if (!response.ok) {
        throw new Error(`Failed to load component: ${name}`);
      }
      
      const html = await response.text();
      this.cache.set(name, html);
      return html;
    } catch (error) {
      console.error(`Error loading component ${name}:`, error);
      return '';
    }
  }

  /**
   * Inject component into DOM element
   */
  async injectComponent(elementId, componentName) {
    const element = document.getElementById(elementId);
    if (!element) {
      console.error(`Element with id '${elementId}' not found`);
      return;
    }

    const html = await this.loadComponent(componentName);
    element.innerHTML = html;
    
    // Trigger custom event for component loaded
    element.dispatchEvent(new CustomEvent('componentLoaded', {
      detail: { componentName }
    }));
  }

  /**
   * Load all components marked with data-component attribute
   */
  async loadAllComponents() {
    const components = document.querySelectorAll('[data-component]');
    const promises = Array.from(components).map(async (element) => {
      const componentName = element.getAttribute('data-component');
      const html = await this.loadComponent(componentName);
      element.innerHTML = html;
      
      element.dispatchEvent(new CustomEvent('componentLoaded', {
        detail: { componentName }
      }));
    });

    await Promise.all(promises);
  }

  /**
   * Update navigation active state based on current page
   */
  updateNavigationState() {
    const currentPath = window.location.pathname;
    const navLinks = document.querySelectorAll('.nav-link, .mobile-nav-link');
    
    navLinks.forEach(link => {
      const href = link.getAttribute('href');
      if (href && (currentPath === href || currentPath.endsWith(href))) {
        link.classList.add('active');
      } else {
        link.classList.remove('active');
      }
    });
  }
}

// Initialize component loader
const componentLoader = new ComponentLoader();

// Auto-load components when DOM is ready
document.addEventListener('DOMContentLoaded', async () => {
  await componentLoader.loadAllComponents();
  componentLoader.updateNavigationState();
});

// Export for use in other modules
window.ComponentLoader = ComponentLoader;
window.componentLoader = componentLoader;
