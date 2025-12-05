# Installation
1. Go to https://www.github.com/codespaces
2. Make a new codespace and use this repository
3. Go to the terminal and enter the following commands
```bash
chmod +x init.sh
./init.sh
chmod +x build.sh
./build.sh
```
4. Now wait for everything to finish, and you'll find your .wasm and .js files in the /pkg directory!

# Usage
```javascript
import init, { binary_to_xml, xml_to_binary } from "./your_directory/index.js";
await init();
const bytes = new Uint8Array(...);
const xml = binary_to_xml(bytes); // returns Uint8Array
const binary = xml_to_binary(xml) // returns Uint8Array
```
