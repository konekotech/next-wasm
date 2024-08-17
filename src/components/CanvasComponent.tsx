import React, { useEffect } from 'react';

const CanvasComponent = () => {
  useEffect(() => {
  const loadWasm = async () => {
    try {
      const wasm = await import('../../public/static/wasm/pkg/front_wasm.js');
      if (wasm && wasm.start) {
        console.log('Wasm module loaded');
        wasm.start();
      } else {
        console.error('Wasm module failed to load or start function not found');
      }
    } catch (err) {
      console.error('Failed to load Wasm module:', err);
    }
  };

  loadWasm();
}, []);


  return (
    <canvas id="canvas" width="512" height="512">
      Your browser does not support the HTML canvas tag.
    </canvas>
  );
};

export default CanvasComponent;
