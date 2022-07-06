# Testing WASM in browser
A repo I'm using to play with web assembly. Different ways to link into a webpage (wasm-pack and manually) and different ways to write the wasm (from rust with wasm-bindgen, from wat and from raw hex).

## Building commands
Using `wasm-pack build --target web` to compile Rust to wasm in `/pkg`.

Using `python -m http.server` to serve locally for testing.

Using `sed 's/;.*//' fact.wasm.txt | xxd -r -p > fact.wasm` to assembled the handwritten wasm hex into a binary.

Using `wasm2wat handwritten.wat -o handwritten.wasm` to assemble handwritten WAT to wasm binary.
