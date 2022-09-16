# evtx-ffi

## Usage

```bash
# Install cbindgen && bindgen utilities
cargo install --force cbindgen bindgen

# Build the library
cargo build -r

# Generate hpp from library
cbindgen --config cbindgen.toml --crate evtx-ffi -o evtxffi.hpp

# The resulting static library is in target/release/
```
