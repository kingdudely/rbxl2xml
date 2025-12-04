# Installation
1. Go to https://www.github.com/codespaces
2. Make a new codespace and use this repository
3. Go to the terminal and enter the following commands
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

source $HOME/.cargo/env

rustup target add wasm32-unknown-unknown

cargo install wasm-pack

wasm-pack build --target web --release
```
4. Now wait for everything to finish, and you'll find your .wasm and .js files in the /pkg directory!

# Usage
```javascript
import init, { binary_to_xml } from "./your_directory/index.js";
await init();
const binary = new Uint8Array(...);
const xml = binary_to_xml(binary); // string
```
