#!/usr/bin/env node

/**
 * Build Script for Commi Website
 * Optimizes CSS, JS, HTML, and images for production
 */

const fs = require('fs').promises;
const path = require('path');
const { execSync } = require('child_process');

// Import optimization modules
const CleanCSS = require('clean-css');
const { minify: minifyHTML } = require('html-minifier');
const { minify: minifyJS } = require('terser');

class WebsiteBuild {
  constructor() {
    this.srcDir = path.join(__dirname, '..');
    this.distDir = path.join(__dirname, '..', 'dist');
    this.startTime = Date.now();
  }

  /**
   * Main build process
   */
  async build() {
    console.log('🚀 Starting Commi website build...\n');

    try {
      // Clean and create dist directory
      await this.cleanDist();
      await this.createDist();

      // Copy static assets
      await this.copyAssets();

      // Build and optimize files
      await this.buildCSS();
      await this.buildJS();
      await this.buildHTML();
      await this.optimizeImages();

      // Generate reports
      await this.generateBuildReport();

      const buildTime = ((Date.now() - this.startTime) / 1000).toFixed(2);
      console.log(`\n✅ Build completed successfully in ${buildTime}s`);
      console.log(`📁 Output directory: ${this.distDir}`);

    } catch (error) {
      console.error('❌ Build failed:', error);
      process.exit(1);
    }
  }

  /**
   * Clean dist directory
   */
  async cleanDist() {
    try {
      await fs.rmdir(this.distDir, { recursive: true });
      console.log('🧹 Cleaned dist directory');
    } catch (error) {
      // Directory doesn't exist, that's fine
    }
  }

  /**
   * Create dist directory structure
   */
  async createDist() {
    const dirs = [
      this.distDir,
      path.join(this.distDir, 'css'),
      path.join(this.distDir, 'js'),
      path.join(this.distDir, 'pages'),
      path.join(this.distDir, 'assets'),
      path.join(this.distDir, 'assets', 'images'),
      path.join(this.distDir, 'reports')
    ];

    for (const dir of dirs) {
      await fs.mkdir(dir, { recursive: true });
    }

    console.log('📁 Created dist directory structure');
  }

  /**
   * Copy static assets
   */
  async copyAssets() {
    const staticFiles = [
      'robots.txt',
      'sitemap.xml',
      'README.md'
    ];

    for (const file of staticFiles) {
      const srcPath = path.join(this.srcDir, file);
      const distPath = path.join(this.distDir, file);
      
      try {
        await fs.copyFile(srcPath, distPath);
      } catch (error) {
        console.warn(`⚠️  Could not copy ${file}:`, error.message);
      }
    }

    console.log('📋 Copied static assets');
  }

  /**
   * Build and minify CSS
   */
  async buildCSS() {
    const cssPath = path.join(this.srcDir, 'css', 'styles.css');
    const outputPath = path.join(this.distDir, 'css', 'styles.min.css');

    try {
      const cssContent = await fs.readFile(cssPath, 'utf8');
      const cleanCSS = new CleanCSS({
        level: 2,
        returnPromise: true
      });

      const result = await cleanCSS.minify(cssContent);
      
      if (result.errors.length > 0) {
        console.error('CSS minification errors:', result.errors);
      }

      await fs.writeFile(outputPath, result.styles);
      
      const originalSize = Buffer.byteLength(cssContent, 'utf8');
      const minifiedSize = Buffer.byteLength(result.styles, 'utf8');
      const savings = ((originalSize - minifiedSize) / originalSize * 100).toFixed(1);

      console.log(`🎨 CSS minified: ${originalSize} → ${minifiedSize} bytes (${savings}% smaller)`);

    } catch (error) {
      console.error('❌ CSS build failed:', error);
      throw error;
    }
  }

  /**
   * Build and minify JavaScript
   */
  async buildJS() {
    const jsFiles = [
      'js/main.js',
      'js/components.js'
    ];

    for (const jsFile of jsFiles) {
      const jsPath = path.join(this.srcDir, jsFile);
      const fileName = path.basename(jsFile, '.js');
      const outputPath = path.join(this.distDir, 'js', `${fileName}.min.js`);

      try {
        const jsContent = await fs.readFile(jsPath, 'utf8');
        const result = await minifyJS(jsContent, {
          compress: {
            drop_console: true,
            drop_debugger: true
          },
          mangle: true,
          format: {
            comments: false
          }
        });

        if (result.error) {
          console.error(`JS minification error in ${jsFile}:`, result.error);
          continue;
        }

        await fs.writeFile(outputPath, result.code);

        const originalSize = Buffer.byteLength(jsContent, 'utf8');
        const minifiedSize = Buffer.byteLength(result.code, 'utf8');
        const savings = ((originalSize - minifiedSize) / originalSize * 100).toFixed(1);

        console.log(`⚡ ${jsFile} minified: ${originalSize} → ${minifiedSize} bytes (${savings}% smaller)`);

      } catch (error) {
        console.error(`❌ JS build failed for ${jsFile}:`, error);
      }
    }
  }

  /**
   * Build and minify HTML
   */
  async buildHTML() {
    const htmlFiles = [
      'index.html',
      'pages/learn.html',
      'pages/extensions.html',
      'pages/tools.html'
    ];

    const minifyOptions = {
      collapseWhitespace: true,
      removeComments: true,
      removeRedundantAttributes: true,
      removeScriptTypeAttributes: true,
      removeStyleLinkTypeAttributes: true,
      useShortDoctype: true,
      minifyCSS: true,
      minifyJS: true
    };

    for (const htmlFile of htmlFiles) {
      const htmlPath = path.join(this.srcDir, htmlFile);
      const outputPath = path.join(this.distDir, htmlFile);

      try {
        let htmlContent = await fs.readFile(htmlPath, 'utf8');
        
        // Update asset paths for production
        htmlContent = htmlContent
          .replace(/href="css\/styles\.css"/g, 'href="css/styles.min.css"')
          .replace(/src="js\/main\.js"/g, 'src="js/main.min.js"')
          .replace(/src="js\/components\.js"/g, 'src="js/components.min.js"');

        const minifiedHTML = minifyHTML(htmlContent, minifyOptions);
        await fs.writeFile(outputPath, minifiedHTML);

        const originalSize = Buffer.byteLength(htmlContent, 'utf8');
        const minifiedSize = Buffer.byteLength(minifiedHTML, 'utf8');
        const savings = ((originalSize - minifiedSize) / originalSize * 100).toFixed(1);

        console.log(`📄 ${htmlFile} minified: ${originalSize} → ${minifiedSize} bytes (${savings}% smaller)`);

      } catch (error) {
        console.error(`❌ HTML build failed for ${htmlFile}:`, error);
      }
    }
  }

  /**
   * Optimize images (placeholder - requires imagemin setup)
   */
  async optimizeImages() {
    console.log('🖼️  Image optimization skipped (no images found)');
    // TODO: Implement image optimization when images are added
  }

  /**
   * Generate build report
   */
  async generateBuildReport() {
    const report = {
      buildTime: new Date().toISOString(),
      duration: `${((Date.now() - this.startTime) / 1000).toFixed(2)}s`,
      files: await this.getFileStats()
    };

    const reportPath = path.join(this.distDir, 'reports', 'build-report.json');
    await fs.writeFile(reportPath, JSON.stringify(report, null, 2));

    console.log('📊 Generated build report');
  }

  /**
   * Get file statistics
   */
  async getFileStats() {
    const stats = {};
    const files = await this.getAllFiles(this.distDir);

    for (const file of files) {
      const stat = await fs.stat(file);
      const relativePath = path.relative(this.distDir, file);
      stats[relativePath] = {
        size: stat.size,
        modified: stat.mtime.toISOString()
      };
    }

    return stats;
  }

  /**
   * Get all files recursively
   */
  async getAllFiles(dir) {
    const files = [];
    const items = await fs.readdir(dir);

    for (const item of items) {
      const fullPath = path.join(dir, item);
      const stat = await fs.stat(fullPath);

      if (stat.isDirectory()) {
        files.push(...await this.getAllFiles(fullPath));
      } else {
        files.push(fullPath);
      }
    }

    return files;
  }
}

// Run build if called directly
if (require.main === module) {
  const build = new WebsiteBuild();
  build.build().catch(console.error);
}

module.exports = WebsiteBuild;
