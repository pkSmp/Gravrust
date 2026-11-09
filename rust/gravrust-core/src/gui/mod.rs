//! `crate::gui` â€” compatibility tree matching the `client/js/gui/` layout.
//!
//! The ported modules reference `crate::gui::<name>` because the JavaScript they
//! came from lives under `client/js/gui/`. The drawing surface is the
//! [`crate::gui_js::Gui`] struct (ported from `client/js/gui/gui.js`), and the
//! algorithmic pieces are the flat top-level modules. This tree re-exports both
//! so the ported call sites resolve, without duplicating any logic.

pub mod font;

// Re-export the ported Gui struct from gui_js (the 1:1 port of client/js/gui/gui.js)
pub use crate::gui_js::Gui;

// The widget types, under the JS paths the ported screens import.
pub use crate::widgets::{Button, KeyEvent, Slider, TextField, WidgetHost};

// The Screen trait lives in the screens manager (client/js/gui/screens/manager.js).
pub use crate::screens::manager::Screen;

/// JS: `ListOf` from `client/js/gui/slot-list.js` — the scrollable slot list.
/// The ported implementation is `ScrollList` in the screens manager.
pub type ListOf = crate::screens::manager::ScrollList;

/// The GUI screenshot/screen manager. The JS `client/js/gui/screens/manager.js`
/// is ported as the flat `src/screens.rs`.
pub mod screens {
    pub use crate::screens::*;
}

