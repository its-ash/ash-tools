/**
 * Arctic Forsythia Premium — Theme Toggle & Interactive Components
 * Vanilla ES6, no dependencies, single defer script
 */

(function () {
  'use strict';

  // =========================================================================
  // Theme Management
  // =========================================================================
  const THEME_KEY = 'theme';
  const html = document.documentElement;
  const themeToggle = document.getElementById('theme-toggle');
  const themeIcon = themeToggle?.querySelector('.header__theme-icon');
  const announcer = document.getElementById('theme-announcer');

  function getEffectiveTheme() {
    if (html.classList.contains('dark')) return 'dark';
    if (html.classList.contains('light')) return 'light';
    return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
  }

  function applyTheme(theme) {
    html.classList.remove('light', 'dark');
    if (theme === 'dark') {
      html.classList.add('dark');
    } else if (theme === 'light') {
      html.classList.add('light');
    }
    // If neither, let media query decide (no class added)
  }

  function updateToggleUI(theme) {
    const isDark = theme === 'dark';
    if (themeToggle) {
      themeToggle.setAttribute('aria-pressed', isDark.toString());
      themeToggle.setAttribute('aria-label', isDark ? 'Switch to light theme' : 'Switch to dark theme');
    }
    if (themeIcon) {
      themeIcon.textContent = isDark ? 'light_mode' : 'dark_mode';
    }
  }

  function announceThemeChange(theme) {
    if (announcer) {
      announcer.textContent = `Switched to ${theme} theme`;
    }
  }

  function initTheme() {
    const stored = localStorage.getItem(THEME_KEY);
    if (stored === 'dark' || stored === 'light') {
      applyTheme(stored);
      updateToggleUI(stored);
    } else {
      // No stored preference — let media query decide, but sync UI
      const effective = getEffectiveTheme();
      updateToggleUI(effective);
    }
  }

  function toggleTheme() {
    const current = getEffectiveTheme();
    const next = current === 'dark' ? 'light' : 'dark';
    applyTheme(next);
    localStorage.setItem(THEME_KEY, next);
    updateToggleUI(next);
    announceThemeChange(next);
  }

  // Initialize on load
  initTheme();

  // Toggle click handler
  if (themeToggle) {
    themeToggle.addEventListener('click', toggleTheme);
  }

  // Listen for system preference changes (only if no manual override)
  window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
    if (!localStorage.getItem(THEME_KEY)) {
      const next = e.matches ? 'dark' : 'light';
      applyTheme(next);
      updateToggleUI(next);
    }
  });

  // =========================================================================
  // Mobile Navigation
  // =========================================================================
  const hamburger = document.getElementById('hamburger');
  const nav = document.getElementById('main-nav');
  const navLinks = nav?.querySelectorAll('.header__nav-link');

  function openMobileNav() {
    hamburger?.setAttribute('aria-expanded', 'true');
    hamburger?.setAttribute('aria-label', 'Close navigation menu');
    nav?.classList.add('is-open');
    if (hamburger) {
      const icon = hamburger.querySelector('.header__hamburger-icon');
      if (icon) icon.textContent = 'close';
    }
    // Trap focus (simple version)
    document.body.style.overflow = 'hidden';
  }

  function closeMobileNav() {
    hamburger?.setAttribute('aria-expanded', 'false');
    hamburger?.setAttribute('aria-label', 'Open navigation menu');
    nav?.classList.remove('is-open');
    if (hamburger) {
      const icon = hamburger.querySelector('.header__hamburger-icon');
      if (icon) icon.textContent = 'menu';
    }
    document.body.style.overflow = '';
    hamburger?.focus();
  }

  function toggleMobileNav() {
    const isExpanded = hamburger?.getAttribute('aria-expanded') === 'true';
    if (isExpanded) {
      closeMobileNav();
    } else {
      openMobileNav();
    }
  }

  if (hamburger && nav) {
    hamburger.addEventListener('click', toggleMobileNav);

    // Close on link click
    navLinks?.forEach(link => {
      link.addEventListener('click', closeMobileNav);
    });

    // Close on Escape
    document.addEventListener('keydown', (e) => {
      if (e.key === 'Escape' && nav.classList.contains('is-open')) {
        closeMobileNav();
      }
    });

    // Close on outside click
    document.addEventListener('click', (e) => {
      if (nav.classList.contains('is-open') && !nav.contains(e.target) && !hamburger.contains(e.target)) {
        closeMobileNav();
      }
    });
  }

  // =========================================================================
  // FAQ Accordion
  // =========================================================================
  const faqQuestions = document.querySelectorAll('.faq__question');

  faqQuestions.forEach(button => {
    button.addEventListener('click', () => {
      const isExpanded = button.getAttribute('aria-expanded') === 'true';
      const panel = document.getElementById(button.getAttribute('aria-controls'));

      button.setAttribute('aria-expanded', (!isExpanded).toString());

      if (panel) {
        if (isExpanded) {
          panel.hidden = true;
        } else {
          panel.hidden = false;
        }
      }
    });

    // Keyboard support for Enter/Space (native button handles this, but ensure)
    button.addEventListener('keydown', (e) => {
      if (e.key === 'Enter' || e.key === ' ') {
        e.preventDefault();
        button.click();
      }
    });
  });

  // =========================================================================
  // Smooth Scroll Offset for Sticky Header
  // =========================================================================
  const headerHeight = 72; // matches header height in CSS
  document.querySelectorAll('a[href^="#"]').forEach(anchor => {
    anchor.addEventListener('click', function (e) {
      const targetId = this.getAttribute('href');
      if (targetId === '#') return;

      const target = document.querySelector(targetId);
      if (target) {
        e.preventDefault();
        const targetPosition = target.getBoundingClientRect().top + window.pageYOffset - headerHeight;
        window.scrollTo({
          top: targetPosition,
          behavior: 'smooth'
        });
      }
    });
  });

  // =========================================================================
  // Current Year in Footer
  // =========================================================================
  const yearEl = document.getElementById('current-year');
  if (yearEl) {
    yearEl.textContent = new Date().getFullYear().toString();
  }

  // =========================================================================
  // Scroll Reveal (IntersectionObserver) — Optional Enhancement
  // =========================================================================
  const prefersReducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;

  if (!prefersReducedMotion && 'IntersectionObserver' in window) {
    const revealElements = document.querySelectorAll('.reveal');
    if (revealElements.length) {
      const observer = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
          if (entry.isIntersecting) {
            entry.target.classList.add('is-visible');
            observer.unobserve(entry.target);
          }
        });
      }, {
        rootMargin: '0px 0px -10% 0px',
        threshold: 0.1
      });

      revealElements.forEach(el => observer.observe(el));
    }
  }

  // =========================================================================
  // Button Loading State Demo (for forms — not used in static page but available)
  // =========================================================================
  function setButtonLoading(button, isLoading) {
    if (isLoading) {
      button.classList.add('btn--loading');
      button.setAttribute('aria-busy', 'true');
      button.disabled = true;
    } else {
      button.classList.remove('btn--loading');
      button.removeAttribute('aria-busy');
      button.disabled = false;
    }
  }

  // Expose for potential external use
  window.Forsyte = {
    setButtonLoading,
    toggleTheme,
    closeMobileNav
  };

})();