// Gravrust WASM bytes - base64 encoded WASM module
// This is populated at build time by the builder script

// The actual WASM bytes are embedded in the compact HTML as a base64 string
// in the <script id="minecraftjs-wasm"> tag. This module extracts them.

let wasmBase64 = '';

export function setWasmBase64(b64) {
    wasmBase64 = b64;
}

export function getWasmBase64() {
    return wasmBase64;
}

// The builder script will replace this placeholder with the actual base64 string
// when building the compact HTML.
export const wasmBase64 = '__WASM_BASE64_PLACEHOLDER__';

export default { wasmBase64, setWasmBase64, getWasmBase64 };