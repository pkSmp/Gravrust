// Minimal Gravrust boot - only JS glue, all game logic in Rust/WASM

import { setPlatform, platform } from './platform/platform.js';
import { BrowserPlatform } from './platform/browser.js';
import { buildCrashReport, showCrashScreen } from './crash.js';

// Import the WASM module
import * as wasmCore from './wasm/pkg/gravrust.js';
import { wasmBase64 } from './wasm/pkg/wasm-bytes.js';

async function boot() {
  // 1. Initialize Rust/WASM Core
  try {
    if (typeof wasmBase64 === 'string' && wasmBase64.length > 0 && typeof wasmCore.initSync === 'function') {
      const binStr = atob(wasmBase64);
      const wasmBytes = new Uint8Array(binStr.length);
      for (let i = 0; i < binStr.length; i++) wasmBytes[i] = binStr.charCodeAt(i);
      wasmCore.initSync({ module: wasmBytes });
      globalThis.gravrust = wasmCore;
      console.log('[Gravrust] Rust/WASM Core v' + wasmCore.core_version() + ' initialized (' + (wasmBytes.length / 1024).toFixed(1) + ' KB)');
    }
  } catch (err) {
    console.warn('[Gravrust] WASM core init warning:', err);
  }

  // 2. Set up platform
  setPlatform(new BrowserPlatform());
  const P = platform();
  
  // 3. Load configuration
  let config = {};
  try { config = P.loadConfig ? await P.loadConfig() : {}; } catch (e) { console.warn('Config unavailable', e); }

  // 4. Create settings (Rust handles the actual settings logic)
  const settings = {
    graphicsApi: config.graphicsApi || 'webgl2',
    superPotato: false,
    language: 'en_us',
    resourcePacks: [],
    proxy: 0,
    proxies: config.proxies || []
  };

  // 5. Create graphics device
  let device;
  try {
    device = await P.createGraphicsDevice(settings.graphicsApi);
    console.log('[Gravrust] Graphics device:', device.name);
  } catch (e) {
    console.error('Failed to create graphics device:', e);
    P.setLoadingError('Failed to create graphics device: ' + e.message);
    showCrashScreen(buildCrashReport(e, 'Creating graphics device'));
    return;
  }

  // 6. Apply super potato mode if needed
  if (settings.superPotato || device.name === 'Software') {
    if (P.setPotato) P.setPotato(true);
  }

  // 7. Initialize assets
  const assets = {
    device: device,
    language: 'en_us',
    data: { recipes: null }
  };

  // 8. Show startup splash
  const splash = new Splash(device);
  const splashBytes = await assets.readBytes ? await assets.readBytes('textures/gui/title/mojang.png') : null;
  await splash.load(splashBytes);
  splash.start();
  P.hideLoading();

  // 9. Load assets
  try {
    // In a full implementation, this would load all game assets
    // For now, we just load the minimal required data
    console.log('[Gravrust] Loading assets...');
    
    // Load recipes if available
    if (assets.readBytes) {
      const recipesData = await assets.readBytes('bundles/recipes.json');
      if (recipesData) {
        assets.data.recipes = new TextDecoder().decode(recipesData);
        console.log('[Gravrust] Loaded recipes');
      }
    }
  } catch (e) {
    console.warn('[Gravrust] Asset loading warning:', e);
  }

  splash.stop();

  // 10. Call Rust boot function
  try {
    // The Rust boot_alg handles the full boot sequence
    if (typeof wasmCore.boot === 'function') {
      await wasmCore.boot(settings, device, assets);
      console.log('[Gravrust] Rust boot completed');
    }
  } catch (e) {
    console.error('[Gravrust] Rust boot failed:', e);
    P.setLoadingError('Boot failed: ' + e.message);
    showCrashScreen(buildCrashReport(e, 'Rust boot', globalThis.game || null));
    return;
  }

  // 11. Start game loop
  if (typeof wasmCore.start_game_loop === 'function') {
    wasmCore.start_game_loop();
  }
}

// Simple splash class (JS-side for boot splash)
class Splash {
  constructor(device) {
    this.device = device;
    this.texture = false;
    this.running = false;
  }

  async load(bytes) {
    if (bytes && bytes.length > 0) {
      this.texture = true;
    }
  }

  start() {
    this.running = true;
  }

  stop() {
    this.running = false;
    this.texture = false;
  }

  setProgress(pct) {
    const bar = document.getElementById('loading-bar');
    const text = document.getElementById('loading-text');
    if (bar) bar.style.width = Math.round(pct * 100) + '%';
    if (text) text.textContent = Math.round(pct * 100) + '%';
  }
}

// Boot the game
boot().catch(e => {
  console.error('[Gravrust] Fatal boot error:', e);
  const P = platform();
  if (P.setLoadingError) P.setLoadingError('Fatal: ' + e.message);
  showCrashScreen(buildCrashReport(e, 'Fatal boot'));
});