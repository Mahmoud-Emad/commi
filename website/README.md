# Commi Website

This is the official website for Commi, an AI-powered Git commit message generator.

## 🌐 Live Site

Visit the website at: [https://commi.sh](https://commi.sh) (when deployed)

## 🚀 Features

- **Responsive Design**: Works perfectly on desktop, tablet, and mobile devices
- **Dark/Light Mode**: Automatic theme detection with manual toggle
- **Modern UI**: Clean, professional design inspired by the Rust website
- **SEO Optimized**: Proper meta tags, Open Graph, and structured data
- **Fast Loading**: Optimized assets and efficient code
- **Accessibility**: WCAG compliant with proper ARIA labels and keyboard navigation

## 📁 Structure

```
website/
├── index.html              # Homepage with installation instructions
├── css/
│   └── styles.css          # Custom CSS styles and animations
├── js/
│   └── main.js            # Interactive functionality and theme toggle
├── pages/
│   ├── learn.html         # Documentation and tutorials
│   ├── extensions.html    # VS Code extension and other integrations
│   └── tools.html         # Integrations and developer tools
├── assets/
│   └── images/           # Images and icons (to be added)
├── robots.txt            # Search engine crawling instructions
├── sitemap.xml          # Site structure for search engines
└── README.md           # This file
```

## 🎨 Design System

### Colors

- **Primary (Cyan)**: `#06b6d4` - Main brand color
- **Success (Green)**: `#10b981` - Success states and positive actions
- **Warning (Yellow)**: `#f59e0b` - Warnings and "coming soon" badges
- **Error (Red)**: `#ef4444` - Error states and destructive actions
- **Dark**: `#0f172a` - Dark mode background
- **Light**: `#f8fafc` - Light mode background

### Typography

- **Font Family**: Inter (Google Fonts)
- **Headings**: Bold weights (600-700)
- **Body**: Regular weight (400)
- **Code**: Monospace font for code blocks

### Components

- **Cards**: Rounded corners with subtle shadows
- **Buttons**: Primary (filled) and secondary (outlined) variants
- **Code blocks**: Dark background with syntax highlighting colors
- **Navigation**: Fixed header with backdrop blur

## 🛠️ Technologies Used

- **HTML5**: Semantic markup
- **TailwindCSS**: Utility-first CSS framework (via CDN)
- **Vanilla JavaScript**: No frameworks, pure JS for interactions
- **Google Fonts**: Inter font family
- **CSS Grid & Flexbox**: Modern layout techniques

## 🚀 Development

### Local Development

1. **Simple HTTP Server** (recommended):

   ```bash
   # Using Python 3
   cd website
   python -m http.server 8000
   
   # Using Node.js
   npx serve .
   
   # Using PHP
   php -S localhost:8000
   ```

2. **Open in browser**: Navigate to `http://localhost:8000`

### File Serving

The website is designed to work with any static file server. All paths are relative and the site can be served from any directory.

## 📱 Responsive Breakpoints

- **Mobile**: < 640px
- **Tablet**: 640px - 1024px  
- **Desktop**: > 1024px

## ♿ Accessibility Features

- **Keyboard Navigation**: All interactive elements are keyboard accessible
- **Screen Reader Support**: Proper ARIA labels and semantic HTML
- **Color Contrast**: WCAG AA compliant color combinations
- **Focus Indicators**: Clear focus states for all interactive elements
- **Reduced Motion**: Respects user's motion preferences

## 🔍 SEO Features

- **Meta Tags**: Comprehensive meta descriptions and keywords
- **Open Graph**: Social media sharing optimization
- **Structured Data**: JSON-LD for better search engine understanding
- **Sitemap**: XML sitemap for search engine crawling
- **Robots.txt**: Proper crawling instructions

## 🌙 Dark Mode

The website automatically detects the user's system preference and applies the appropriate theme. Users can manually toggle between light and dark modes using the theme toggle button in the navigation.

## 📦 Deployment

The website is a static site and can be deployed to any static hosting service:

- **GitHub Pages**: Push to `gh-pages` branch
- **Netlify**: Connect repository and deploy
- **Vercel**: Import project and deploy
- **AWS S3**: Upload files to S3 bucket with static hosting
- **Any CDN**: Upload files to your preferred CDN

### Build Process

No build process is required. The website uses:

- TailwindCSS via CDN (no compilation needed)
- Vanilla JavaScript (no bundling needed)
- Standard HTML/CSS (no preprocessing needed)

## 🔧 Customization

### Changing Colors

Update the Tailwind config in each HTML file:

```javascript
tailwind.config = {
    theme: {
        extend: {
            colors: {
                'commi-cyan': '#your-color',
                // ... other colors
            }
        }
    }
}
```

### Adding New Pages

1. Create new HTML file in `pages/` directory
2. Copy the navigation structure from existing pages
3. Update navigation links in all pages
4. Add new page to `sitemap.xml`

### Modifying Styles

Custom styles are in `css/styles.css`. The file includes:

- Custom animations
- Component styles
- Utility classes
- Responsive overrides

## 📄 License

This website is part of the Commi project and is licensed under the MIT License.

## 📞 Support

For website-related issues, open an issue on the [Commi GitHub repository](https://github.com/Mahmoud-Emad/commi).

---

Built with ❤️ for the Commi community
