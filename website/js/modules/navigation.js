/**
 * Navigation Module
 * Handles mobile menu, smooth scrolling, and navigation state
 */

class NavigationManager {
  constructor() {
    this.mobileMenuButton = null;
    this.mobileMenu = null;
    this.isMenuOpen = false;
    
    this.init();
  }

  /**
   * Initialize navigation manager
   */
  init() {
    this.bindElements();
    this.bindEvents();
    this.updateActiveLinks();
  }

  /**
   * Bind DOM elements
   */
  bindElements() {
    this.mobileMenuButton = document.getElementById('mobile-menu-button');
    this.mobileMenu = document.getElementById('mobile-menu');
  }

  /**
   * Bind event listeners
   */
  bindEvents() {
    // Mobile menu toggle
    if (this.mobileMenuButton) {
      this.mobileMenuButton.addEventListener('click', () => {
        this.toggleMobileMenu();
      });
    }

    // Close mobile menu when clicking outside
    document.addEventListener('click', (e) => {
      if (this.isMenuOpen && 
          !this.mobileMenu?.contains(e.target) && 
          !this.mobileMenuButton?.contains(e.target)) {
        this.closeMobileMenu();
      }
    });

    // Close mobile menu on escape key
    document.addEventListener('keydown', (e) => {
      if (e.key === 'Escape' && this.isMenuOpen) {
        this.closeMobileMenu();
      }
    });

    // Close mobile menu when clicking on links
    if (this.mobileMenu) {
      this.mobileMenu.addEventListener('click', (e) => {
        if (e.target.tagName === 'A') {
          this.closeMobileMenu();
        }
      });
    }

    // Handle smooth scrolling for anchor links
    document.addEventListener('click', (e) => {
      const link = e.target.closest('a[href^="#"]');
      if (link) {
        e.preventDefault();
        this.smoothScrollTo(link.getAttribute('href'));
      }
    });

    // Update active links on scroll
    window.addEventListener('scroll', () => {
      this.updateActiveLinks();
    });
  }

  /**
   * Toggle mobile menu
   */
  toggleMobileMenu() {
    if (this.isMenuOpen) {
      this.closeMobileMenu();
    } else {
      this.openMobileMenu();
    }
  }

  /**
   * Open mobile menu
   */
  openMobileMenu() {
    if (!this.mobileMenu) return;

    this.mobileMenu.classList.remove('hidden');
    this.mobileMenuButton?.setAttribute('aria-expanded', 'true');
    this.isMenuOpen = true;

    // Prevent body scroll
    document.body.style.overflow = 'hidden';

    // Focus first menu item for accessibility
    const firstLink = this.mobileMenu.querySelector('a');
    if (firstLink) {
      firstLink.focus();
    }
  }

  /**
   * Close mobile menu
   */
  closeMobileMenu() {
    if (!this.mobileMenu) return;

    this.mobileMenu.classList.add('hidden');
    this.mobileMenuButton?.setAttribute('aria-expanded', 'false');
    this.isMenuOpen = false;

    // Restore body scroll
    document.body.style.overflow = '';
  }

  /**
   * Smooth scroll to element
   */
  smoothScrollTo(target) {
    const element = document.querySelector(target);
    if (element) {
      const headerOffset = 80; // Account for fixed header
      const elementPosition = element.getBoundingClientRect().top;
      const offsetPosition = elementPosition + window.pageYOffset - headerOffset;

      window.scrollTo({
        top: offsetPosition,
        behavior: 'smooth'
      });
    }
  }

  /**
   * Update active navigation links based on current page/section
   */
  updateActiveLinks() {
    const currentPath = window.location.pathname;
    const currentHash = window.location.hash;
    
    // Update page-based active states
    const navLinks = document.querySelectorAll('.nav-link, .mobile-nav-link');
    navLinks.forEach(link => {
      const href = link.getAttribute('href');
      let isActive = false;

      if (href) {
        if (href.startsWith('#')) {
          // Hash-based navigation
          isActive = currentHash === href;
        } else {
          // Page-based navigation
          isActive = currentPath === href || 
                    currentPath.endsWith(href) ||
                    (href === '/' && currentPath === '/index.html');
        }
      }

      link.classList.toggle('active', isActive);
    });

    // Update section-based active states for single-page navigation
    this.updateSectionActiveStates();
  }

  /**
   * Update active states based on visible sections
   */
  updateSectionActiveStates() {
    const sections = document.querySelectorAll('section[id]');
    const navLinks = document.querySelectorAll('a[href^="#"]');
    
    if (sections.length === 0) return;

    let currentSection = '';
    const scrollPosition = window.scrollY + 100; // Offset for header

    sections.forEach(section => {
      const sectionTop = section.offsetTop;
      const sectionHeight = section.offsetHeight;
      
      if (scrollPosition >= sectionTop && scrollPosition < sectionTop + sectionHeight) {
        currentSection = '#' + section.id;
      }
    });

    navLinks.forEach(link => {
      const href = link.getAttribute('href');
      const isActive = href === currentSection;
      link.classList.toggle('active', isActive);
    });
  }

  /**
   * Get current menu state
   */
  isMenuOpen() {
    return this.isMenuOpen;
  }
}

// Export for use in other modules
export default NavigationManager;
