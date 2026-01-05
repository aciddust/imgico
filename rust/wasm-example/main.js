import init, { imgico, imgsvg } from './pkg/imgico.js';

async function main() {
  // Initialize the Wasm module
  await init();

  // Load image as Uint8Array
  const response = await fetch('input.png');
  const arrayBuffer = await response.arrayBuffer();
  const inputBuffer = new Uint8Array(arrayBuffer);

  // 1. Convert to ICO
  try {
    // Default sizes: [16, 32, 48, 64, 128, 256]
    const icoData = imgico(inputBuffer);

    // Download or use the ICO data
    download(icoData, 'icon.ico');
    // Custom sizes
    const customIcoData = imgico(inputBuffer, new Uint32Array([16, 32]));
  } catch (e) {
    console.error('ICO conversion failed:', e);
  }

  // 2. Convert to SVG
  try {
    // Optional size argument
    const svgData = imgsvg(inputBuffer, 512);

    // Convert bytes to string if needed
    const svgString = new TextDecoder().decode(svgData);
    console.log(svgString);
  } catch (e) {
    console.error('SVG conversion failed:', e);
  }
}

function download(data, filename) {
  const blob = new Blob([data], { type: 'application/octet-stream' });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = filename;
  a.click();
  URL.revokeObjectURL(url);
}

main();
