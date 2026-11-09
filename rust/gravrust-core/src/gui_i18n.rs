//! Compatibility re-export: use the canonical `crate::i18n` module.
//! The `client/js/gui/i18n.js` helper functions (`tr`, `tr_option`, `on_off`)
//! map to the canonical `crate::i18n` module which has the same interface
//! (different internal impl, but same public API).
//!
//! This file exists only as a compatibility shim for downstream code that
//! imports `crate::gui::i18n`. All logic lives in `crate::i18n`.

pub use crate::i18n::{tr, tr_args as tr_option, set_translations as set_translator};

/// JS: `onOff(v)` — `tr(v ? 'options.on' : 'options.off')`.
pub fn on_off(v: bool) -> String {
    crate::i18n::tr(if v { "options.on" } else { "options.off" })
}