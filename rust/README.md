# imgico

High-performance image to ICO/SVG converter built with Rust.

**Features:**
- 🦀 **Pure Rust** - Fast and memory-safe
- 🌐 **WebAssembly Support** - Run in the browser
- 🖼️ **Multiple Formats** - ICO (multi-size) and SVG output
- 🎯 **Simple API** - Easy to use in both Rust and JavaScript
- ⚡ **High Performance** - Optimized for speed and size

## Installation

### As a Rust Library

Add to your `Cargo.toml`:

```toml
[dependencies]
imgico = "0.1"
```

### As a CLI Tool

```bash
cargo install imgico
```

### For WebAssembly

Build from source:

```bash
# Install wasm-pack
cargo install wasm-pack

# Build
wasm-pack build --target web
```

## Usage

### Rust Library

```rust
use imgico::{imgico_core, imgsvg_core};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read input image
    let input = fs::read("input.png")?;

    // Convert to ICO with default sizes [16, 32, 48, 64, 128, 256]
    let ico_data = imgico_core(&input, None)?;
    fs::write("output.ico", ico_data)?;

    // Convert to ICO with custom sizes
    let custom_ico = imgico_core(&input, Some(vec![16, 32, 48]))?;
    fs::write("custom.ico", custom_ico)?;

    // Convert to SVG (default size)
    let svg_data = imgsvg_core(&input, None)?;
    fs::write("output.svg", svg_data)?;

    // Convert to SVG with specific size
    let sized_svg = imgsvg_core(&input, Some(512))?;
    fs::write("output_512.svg", sized_svg)?;

    Ok(())
}
```

### CLI Usage

Convert image to ICO format:

```bash
imgico input.png -f ico
```

Convert image to SVG format:

```bash
imgico input.png -f svg
```

This creates a directory with timestamped name containing multiple sizes (16, 32, 48, 64, 128, 256).

### WebAssembly (Browser)

```javascript
import init, { imgico, imgsvg } from './pkg/imgico.js';

async function convertImage() {
  // Initialize the Wasm module
  await init();

  // Load image as Uint8Array
  const response = await fetch('input.png');
  const arrayBuffer = await response.arrayBuffer();
  const inputBuffer = new Uint8Array(arrayBuffer);

  // Convert to ICO with default sizes
  try {
    const icoData = imgico(inputBuffer);
    downloadFile(icoData, 'icon.ico');
  } catch (e) {
    console.error('ICO conversion failed:', e);
  }

  // Convert to ICO with custom sizes
  try {
    const customIco = imgico(inputBuffer, new Uint32Array([16, 32, 64]));
    downloadFile(customIco, 'custom.ico');
  } catch (e) {
    console.error('Custom ICO conversion failed:', e);
  }

  // Convert to SVG
  try {
    const svgData = imgsvg(inputBuffer, 512);
    const svgString = new TextDecoder().decode(svgData);
    console.log(svgString);
  } catch (e) {
    console.error('SVG conversion failed:', e);
  }
}

function downloadFile(data, filename) {
  const blob = new Blob([data], { type: 'application/octet-stream' });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = filename;
  a.click();
  URL.revokeObjectURL(url);
}

convertImage();
```

## API Reference

### Rust

#### `imgico_core(input: &[u8], sizes: Option<Vec<u32>>) -> Result<Vec<u8>, String>`

Convert an image to ICO format.

**Parameters:**
- `input` - Input image data (PNG, JPEG, WebP, etc.)
- `sizes` - Optional vector of icon sizes. Default: `[16, 32, 48, 64, 128, 256]`

**Returns:** ICO file data as bytes

**Example:**
```rust
let ico = imgico_core(&image_data, Some(vec![16, 32, 64]))?;
```

#### `imgsvg_core(input: &[u8], size: Option<u32>) -> Result<Vec<u8>, String>`

Convert an image to SVG format with embedded PNG.

**Parameters:**
- `input` - Input image data
- `size` - Optional target width/height. If `None`, uses original size

**Returns:** SVG file data as bytes

**Example:**
```rust
let svg = imgsvg_core(&image_data, Some(512))?;
```

### WebAssembly (JavaScript)

#### `imgico(input: Uint8Array, sizes?: Uint32Array) -> Uint8Array`

Convert an image to ICO format.

**Parameters:**
- `input` - Input image data as `Uint8Array`
- `sizes` - Optional array of icon sizes. Default: `[16, 32, 48, 64, 128, 256]`

**Returns:** ICO file data as `Uint8Array`

**Example:**
```javascript
const icoData = imgico(inputBuffer, new Uint32Array([16, 32]));
```

#### `imgsvg(input: Uint8Array, size?: number) -> Uint8Array`

Convert an image to SVG format.

**Parameters:**
- `input` - Input image data as `Uint8Array`  
- `size` - Optional target width/height

**Returns:** SVG file data as `Uint8Array`

**Example:**
```javascript
const svgData = imgsvg(inputBuffer, 512);
const svgString = new TextDecoder().decode(svgData);
```

## Supported Input Formats

- PNG
- JPEG
- WebP
- GIF
- BMP
- And more (via the `image` crate)

## Building from Source

```bash
# Clone the repository
git clone https://github.com/aciddust/imgico
cd imgico/rust

# Build Rust library/CLI
cargo build --release

# Build WebAssembly
wasm-pack build --target web --release

# Run tests
cargo test
```

## Performance

The library is optimized for both speed and size:
- Uses Lanczos3 filtering for high-quality resizing
- WebAssembly build is ~430KB (optimized)
- Fast processing with minimal memory overhead

## License

MIT OR Apache-2.0
