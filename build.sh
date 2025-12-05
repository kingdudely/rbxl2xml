#!/bin/bash
cd "$(dirname "${BASH_SOURCE[0]}")"
source "$HOME/.cargo/env"
wasm-pack build --target web --release