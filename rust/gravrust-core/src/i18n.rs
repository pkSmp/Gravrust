//! Port of `client/js/gui/i18n.js`: i18n helpers.
//!
//! Translation helper. Vanilla keys come from the game's .lang files
//! (and resource packs); strings that only exist in MinecraftJS's own
//! screens live in `MINECRAFTJS_EN` below and are always available
//! in English, so any language without them falls back to English
//! (just like vanilla falls back to en_US for missing keys).

use std::collections::HashMap;
use std::sync::{Arc, LazyLock, RwLock};

/// Global translator function. JS: `let translateFn = (key) => key;`
static TRANSLATOR: LazyLock<RwLock<Arc<dyn Fn(&str, &[&str]) -> String + Send + Sync>>> =
    LazyLock::new(|| RwLock::new(Arc::new(|key, _args| key.to_string())));

/// JS: `setTranslator(fn)` — replace the translator function.
pub fn set_translator(f: impl Fn(&str, &[&str]) -> String + Send + Sync + 'static) {
    *TRANSLATOR.write().unwrap() = Arc::new(f);
}

/// JS: `tr(key, ...args)` — translate a key with optional args.
pub fn tr(key: &str, args: &[&str]) -> String {
    TRANSLATOR.read().unwrap()(key, args)
}

/// JS: `tr(key)` — convenience for single-arg calls.
pub fn tr_simple(key: &str) -> String {
    tr(key, &[])
}

/// JS: `tr(key, ...args)` — alias used by the gui_i18n shim.
pub fn tr_args(key: &str, args: &[String]) -> String {
    tr(key, &args.iter().map(|s| s.as_str()).collect::<Vec<_>>())
}

/// JS: `trOption(key, value)` — "Label: VALUE" like vanilla's GameSettings.getKeyBinding.
pub fn tr_option(key: &str, value: &str) -> String {
    format!("{}: {}", tr_simple(key), value)
}

/// JS: `onOff(v)` — `tr(v ? 'options.on' : 'options.off')`.
pub fn on_off(v: bool) -> String {
    tr_simple(if v { "options.on" } else { "options.off" })
}

/// Load translations from a .lang file (JSON or key=value format).
/// Returns number of keys loaded.
pub fn load_translations(lang_json: &str) -> usize {
    let mut map = HashMap::new();
    // Try JSON first
    if let Ok(v) = serde_json::from_str::<HashMap<String, String>>(lang_json) {
        map.extend(v);
    } else {
        // Fallback: simple key=value lines
        for line in lang_json.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some((k, v)) = line.split_once('=') {
                map.insert(k.trim().to_string(), v.trim().to_string());
            }
        }
    }
    let count = map.len();
    let old_translator = TRANSLATOR.read().unwrap().clone();
    let new_map = map;
    *TRANSLATOR.write().unwrap() = Arc::new(move |key, args| {
        if let Some(tmpl) = new_map.get(key) {
            format_args(tmpl, args)
        } else {
            old_translator(key, args)
        }
    });
    count
}

/// Format a template string with positional args: `%s` -> arg[0], etc.
fn format_args(tmpl: &str, args: &[&str]) -> String {
    let mut out = String::with_capacity(tmpl.len() + args.iter().map(|s| s.len()).sum::<usize>());
    let mut chars = tmpl.chars().peekable();
    let mut arg_idx = 0;
    while let Some(ch) = chars.next() {
        if ch == '%' && matches!(chars.peek(), Some('s' | 'd')) {
            chars.next(); // consume 's' or 'd'
            if arg_idx < args.len() {
                out.push_str(args[arg_idx]);
            } else {
                out.push('%');
            }
            arg_idx += 1;
        } else {
            out.push(ch);
        }
    }
    out
}

/// MinecraftJS-specific English strings (always available as fallback).
/// Ported from `MINECRAFTJS_EN` in `client/js/gui/i18n.js`.
pub const MINECRAFTJS_EN: &[(&str, &str)] = &[
    ("minecraftjs.extras.title", "MinecraftJS Extras"),
    ("minecraftjs.extras.preloadTextures", "Preload World Textures"),
    ("minecraftjs.extras.functionKeys", "F + Number Function Keys"),
    ("minecraftjs.extras.zoom", "Zoom"),
    ("minecraftjs.extras.dynamicLights", "Held Item Lights"),
    ("minecraftjs.extras.otherLights", "Other Holder Lights"),
    ("minecraftjs.extras.fullBright", "Fullbright"),
    ("minecraftjs.skinProxy.title", "Skin Proxy"),
    ("minecraftjs.skinProxy.url", "Skin/profile URL template"),
    ("minecraftjs.skinProxy.automatic", "Blank: use the automatic MinecraftJS proxy."),
    ("minecraftjs.skinProxy.encoded", "%R inserts the URL with percent encoding."),
    ("minecraftjs.skinProxy.raw", "%u: player UUID. %r: original URL without encoding."),
    ("minecraftjs.skinProxy.example", "Example: https://proxy.example/?url=%R"),
    ("minecraftjs.skinProxy.invalid", "Use an HTTP(S) URL containing %u, %r or %R."),
    ("minecraftjs.version", "MinecraftJS 1.8.9"),
    ("minecraftjs.notAffiliated", "Not affiliated with Mojang AB"),
    ("minecraftjs.accounts.title", "Accounts"),
    ("minecraftjs.accounts.use", "Use Account"),
    ("minecraftjs.accounts.addMicrosoft", "Add Microsoft Account"),
    ("minecraftjs.accounts.addOffline", "Add Offline"),
    ("minecraftjs.accounts.remove", "Remove"),
    ("minecraftjs.accounts.inUse", "(in use)"),
    ("minecraftjs.accounts.microsoft", "Microsoft account"),
    ("minecraftjs.accounts.microsoftRefresh", "Microsoft account (session refreshes on join)"),
    ("minecraftjs.accounts.offline", "Offline (offline-mode servers only)"),
    ("minecraftjs.login.title", "Sign in with Microsoft"),
    ("minecraftjs.login.contacting", "Contacting Microsoft..."),
    ("minecraftjs.login.step1", "1. Open %s on any device"),
    ("minecraftjs.login.step2", "2. Enter this code:"),
    ("minecraftjs.login.waiting", "Waiting for you to sign in"),
    ("minecraftjs.login.copied", "Code copied to clipboard"),
    ("minecraftjs.login.openLink", "Open Link"),
    ("minecraftjs.login.copyCode", "Copy Code"),
    ("minecraftjs.login.retry", "Try Again"),
    ("minecraftjs.login.signedIn", "Signed in as %s"),
    ("minecraftjs.login.expired", "The code expired. Please try again."),
    ("minecraftjs.login.privacy", "Your login is used only to join servers; tokens are stored on this device."),
    ("minecraftjs.offline.title", "Add Offline Account"),
    ("minecraftjs.offline.username", "Username"),
    ("minecraftjs.offline.add", "Add"),
    ("minecraftjs.offline.info", "Offline accounts can only join servers with online-mode=false."),
    ("minecraftjs.packs.add", "Add Resource Pack..."),
    ("minecraftjs.packs.hint", "Click a pack icon to move it. Shift-click an available pack to delete it."),
    ("minecraftjs.packs.added", "Added %s"),
    ("minecraftjs.packs.deleted", "Deleted %s"),
    ("minecraftjs.packs.failed", "Failed to load resources: %s"),
    ("minecraftjs.packs.reloading", "Reloading resources..."),
    ("minecraftjs.packs.defaultName", "Default"),
    ("minecraftjs.packs.defaultDescription", "The default look of MinecraftJS"),
    ("minecraftjs.loading", "Loading..."),
    ("minecraftjs.key.zoom", "Zoom"),
    ("minecraftjs.key.hideGui", "Hide GUI"),
    ("minecraftjs.deathScreen.spectate", "Spectate world"),
    ("options.advancedTooltips", "Advanced Tooltips"),
    ("minecraftjs.options.graphicsApi", "Graphics API"),
    ("minecraftjs.options.graphicsApi.software", "Software (CPU)"),
    ("minecraftjs.options.superPotato", "Super Potato Mode"),
    ("minecraftjs.options.restart", "(restart)"),
    ("minecraftjs.options.graphicsApiChanged", "Graphics API changes apply after a restart."),
    ("minecraftjs.realms.unavailable", "Minecraft Realms"),
    ("minecraftjs.disconnect.online", "This server is in online mode and needs a Microsoft account.\nSign in with the account button on the title screen."),
    ("minecraftjs.multiplayer.pinging", "Pinging..."),
    ("minecraftjs.multiplayer.cantConnect", "Can't connect to server"),
    ("minecraftjs.multiplayer.eagler", "Eaglercraft server:"),
    ("minecraftjs.extras.lodFastLeaves", "Fast Distant Leaves"),
    ("minecraftjs.extras.reduceDistantAnimations", "Reduce Distant Mob Animations"),
    ("minecraftjs.extras.animateDistantTextures", "Animate Distant Textures"),
    ("minecraftjs.options.lodDistance", "LOD Distance"),
    ("minecraftjs.proxies.title", "Proxy Servers"),
    ("minecraftjs.proxies.none", "No proxies yet: add one to join vanilla servers"),
    ("minecraftjs.proxies.noneShort", "No Proxy"),
    ("minecraftjs.proxies.use", "Use Proxy"),
    ("minecraftjs.proxies.add", "Add Proxy"),
    ("minecraftjs.proxies.edit", "Edit Proxy"),
    ("minecraftjs.proxies.test", "Test"),
    ("minecraftjs.proxies.testAll", "Test All"),
    ("minecraftjs.proxies.defaultName", "MinecraftJS Proxy"),
    ("minecraftjs.proxies.enterName", "Proxy Name"),
    ("minecraftjs.proxies.enterUrl", "Proxy Address (wss://host:port/path)"),
    ("minecraftjs.proxies.deleteQuestion", "Are you sure you want to remove this proxy?"),
    ("minecraftjs.proxies.unreachable", "Can't reach proxy"),
    ("minecraftjs.proxies.reachable", "Proxy reachable"),
    ("minecraftjs.proxies.type", "Type: %s"),
    ("minecraftjs.proxies.type.minecraftjs", "MinecraftJS Proxy"),
    ("minecraftjs.proxies.type.relay", "Java Relay"),
    ("minecraftjs.proxies.type.wisp", "Wisp"),
    ("minecraftjs.proxies.typeInfo.minecraftjs", "Logs in for the client; also signs in accounts"),
    ("minecraftjs.proxies.typeInfo.relay", "Eaglercraft 26.2 relay: raw TCP, the client logs in"),
    ("minecraftjs.proxies.typeInfo.wisp", "Wisp server: raw TCP, the client logs in"),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tr_fallback() {
        assert_eq!(tr_simple("unknown.key"), "unknown.key");
    }

    #[test]
    fn tr_with_args() {
        assert_eq!(tr("test.key", &["a".into(), "b".into()]), "test.key");
    }

    #[test]
    fn on_off_true() {
        assert_eq!(on_off(true), "options.on");
    }

    #[test]
    fn on_off_false() {
        assert_eq!(on_off(false), "options.off");
    }

    #[test]
    fn tr_option_basic() {
        assert_eq!(tr_option("fov", "70"), "fov: 70");
    }

    #[test]
    fn load_translations_json() {
        let json = r#"{"test.key": "Hello %s"}"#;
        assert_eq!(load_translations(json), 1);
        assert_eq!(tr("test.key", &["World".into()]), "Hello World");
    }

    #[test]
    fn load_translations_kv() {
        let kv = "test.key=Hello %s\nother=value";
        assert_eq!(load_translations(kv), 2);
        assert_eq!(tr("test.key", &["World".into()]), "Hello World");
        assert_eq!(tr_simple("other"), "value");
    }

    #[test]
    fn format_args_percent_s() {
        assert_eq!(format_args("Hello %s", &["World".into()]), "Hello World");
    }

    #[test]
    fn format_args_multiple() {
        assert_eq!(format_args("%s %d", &["a".into(), "1".into()]), "a 1");
    }

    #[test]
    fn format_args_no_args() {
        assert_eq!(format_args("Hello %s", &[]), "Hello %");
    }
}