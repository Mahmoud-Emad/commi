/**
 * Service Worker for Commi Website
 * Provides offline functionality and performance improvements
 */

const CACHE_NAME = 'commi-website-v1';
const STATIC_CACHE = 'commi-static-v1';
const DYNAMIC_CACHE = 'commi-dynamic-v1';

// Files to cache immediately
const STATIC_ASSETS = [
  '/',
  '/index.html',
  '/pages/learn.html',
  '/pages/extensions.html',
  '/pages/tools.html',
  '/css/styles.css',
  '/js/main.js',
  '/js/components.js',
  '/components/head.html',
  '/components/navigation.html',
  '/components/footer.html',
  '/robots.txt',
  '/sitemap.xml'
];

// External resources to cache
const EXTERNAL_RESOURCES = [
  'https://cdn.tailwindcss.com',
  'https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700&display=swap',
  'https://fonts.gstatic.com'
];

/**
 * Install event - cache static assets
 */
self.addEventListener('install', (event) => {
  console.log('Service Worker: Installing...');
  
  event.waitUntil(
    Promise.all([
      // Cache static assets
      caches.open(STATIC_CACHE).then((cache) => {
        console.log('Service Worker: Caching static assets');
        return cache.addAll(STATIC_ASSETS);
      }),
      
      // Cache external resources
      caches.open(DYNAMIC_CACHE).then((cache) => {
        console.log('Service Worker: Caching external resources');
        return Promise.allSettled(
          EXTERNAL_RESOURCES.map(url => 
            cache.add(url).catch(err => console.warn(`Failed to cache ${url}:`, err))
          )
        );
      })
    ]).then(() => {
      console.log('Service Worker: Installation complete');
      return self.skipWaiting();
    })
  );
});

/**
 * Activate event - clean up old caches
 */
self.addEventListener('activate', (event) => {
  console.log('Service Worker: Activating...');
  
  event.waitUntil(
    caches.keys().then((cacheNames) => {
      return Promise.all(
        cacheNames.map((cacheName) => {
          if (cacheName !== STATIC_CACHE && cacheName !== DYNAMIC_CACHE) {
            console.log('Service Worker: Deleting old cache:', cacheName);
            return caches.delete(cacheName);
          }
        })
      );
    }).then(() => {
      console.log('Service Worker: Activation complete');
      return self.clients.claim();
    })
  );
});

/**
 * Fetch event - serve cached content with network fallback
 */
self.addEventListener('fetch', (event) => {
  const { request } = event;
  const url = new URL(request.url);

  // Skip non-GET requests
  if (request.method !== 'GET') {
    return;
  }

  // Skip chrome-extension and other non-http(s) requests
  if (!url.protocol.startsWith('http')) {
    return;
  }

  event.respondWith(
    caches.match(request).then((cachedResponse) => {
      // Return cached version if available
      if (cachedResponse) {
        // For HTML files, check for updates in background
        if (request.headers.get('accept')?.includes('text/html')) {
          updateCache(request);
        }
        return cachedResponse;
      }

      // Network request with caching
      return fetch(request).then((response) => {
        // Don't cache non-successful responses
        if (!response || response.status !== 200 || response.type !== 'basic') {
          return response;
        }

        // Clone response for caching
        const responseToCache = response.clone();
        
        // Determine cache to use
        const cacheName = isStaticAsset(request.url) ? STATIC_CACHE : DYNAMIC_CACHE;
        
        caches.open(cacheName).then((cache) => {
          cache.put(request, responseToCache);
        });

        return response;
      }).catch((error) => {
        console.error('Service Worker: Fetch failed:', error);
        
        // Return offline page for HTML requests
        if (request.headers.get('accept')?.includes('text/html')) {
          return caches.match('/offline.html') || createOfflineResponse();
        }
        
        throw error;
      });
    })
  );
});

/**
 * Background sync for cache updates
 */
self.addEventListener('sync', (event) => {
  if (event.tag === 'background-sync') {
    event.waitUntil(updateAllCaches());
  }
});

/**
 * Push notification handling
 */
self.addEventListener('push', (event) => {
  if (event.data) {
    const data = event.data.json();
    const options = {
      body: data.body,
      icon: '/assets/images/icon-192.png',
      badge: '/assets/images/badge-72.png',
      vibrate: [100, 50, 100],
      data: {
        dateOfArrival: Date.now(),
        primaryKey: data.primaryKey
      },
      actions: [
        {
          action: 'explore',
          title: 'Explore',
          icon: '/assets/images/checkmark.png'
        },
        {
          action: 'close',
          title: 'Close',
          icon: '/assets/images/xmark.png'
        }
      ]
    };

    event.waitUntil(
      self.registration.showNotification(data.title, options)
    );
  }
});

/**
 * Notification click handling
 */
self.addEventListener('notificationclick', (event) => {
  event.notification.close();

  if (event.action === 'explore') {
    event.waitUntil(
      clients.openWindow('/')
    );
  }
});

/**
 * Helper Functions
 */

/**
 * Check if URL is a static asset
 */
function isStaticAsset(url) {
  return STATIC_ASSETS.some(asset => url.endsWith(asset)) ||
         url.includes('/css/') ||
         url.includes('/js/') ||
         url.includes('/assets/');
}

/**
 * Update cache in background
 */
function updateCache(request) {
  fetch(request).then((response) => {
    if (response && response.status === 200) {
      caches.open(STATIC_CACHE).then((cache) => {
        cache.put(request, response);
      });
    }
  }).catch((error) => {
    console.warn('Service Worker: Background update failed:', error);
  });
}

/**
 * Update all caches
 */
function updateAllCaches() {
  return Promise.all([
    updateStaticCache(),
    updateDynamicCache()
  ]);
}

/**
 * Update static cache
 */
function updateStaticCache() {
  return caches.open(STATIC_CACHE).then((cache) => {
    return Promise.allSettled(
      STATIC_ASSETS.map(url => 
        fetch(url).then(response => {
          if (response.status === 200) {
            return cache.put(url, response);
          }
        }).catch(err => console.warn(`Failed to update ${url}:`, err))
      )
    );
  });
}

/**
 * Update dynamic cache
 */
function updateDynamicCache() {
  return caches.open(DYNAMIC_CACHE).then((cache) => {
    return Promise.allSettled(
      EXTERNAL_RESOURCES.map(url => 
        fetch(url).then(response => {
          if (response.status === 200) {
            return cache.put(url, response);
          }
        }).catch(err => console.warn(`Failed to update ${url}:`, err))
      )
    );
  });
}

/**
 * Create offline response
 */
function createOfflineResponse() {
  return new Response(`
    <!DOCTYPE html>
    <html lang="en">
    <head>
      <meta charset="UTF-8">
      <meta name="viewport" content="width=device-width, initial-scale=1.0">
      <title>Offline - Commi</title>
      <style>
        body { font-family: system-ui, sans-serif; text-align: center; padding: 2rem; }
        .offline { max-width: 400px; margin: 0 auto; }
        .icon { font-size: 4rem; margin-bottom: 1rem; }
        h1 { color: #374151; margin-bottom: 0.5rem; }
        p { color: #6b7280; margin-bottom: 2rem; }
        button { background: #06b6d4; color: white; border: none; padding: 0.75rem 1.5rem; border-radius: 0.5rem; cursor: pointer; }
        button:hover { background: #0891b2; }
      </style>
    </head>
    <body>
      <div class="offline">
        <div class="icon">📡</div>
        <h1>You're Offline</h1>
        <p>Please check your internet connection and try again.</p>
        <button onclick="window.location.reload()">Retry</button>
      </div>
    </body>
    </html>
  `, {
    headers: { 'Content-Type': 'text/html' }
  });
}
