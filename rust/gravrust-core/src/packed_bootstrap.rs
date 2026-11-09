//! Port of `client/js/packed-bootstrap.js`: packed module loader (stub).
//!
//! The actual implementation loads a gzip-compressed, base85-encoded
//! JavaScript module from a `<script>` element and evaluates it.
//! This is browser-bound and stays as a stub in the Rust port.

use wasm_bindgen::prelude::*;

/// Stub for loading packed modules (browser-bound in JS).
#[wasm_bindgen]
pub fn load_packed_module(_element_id: &str) -> Result<(), JsValue> {
    // Browser-bound: requires DOM, Blob, import()
    Ok(())
}

/// Auto-load the `minecraftjs-code` script element.
#[wasm_bindgen]
pub fn load_packed_module_auto() -> Result<(), JsValue> {
    // Browser-bound stub
    Ok(())
}