'use client';
import { useEffect } from 'react';

export default function Home() {
  useEffect(() => {
    const initWasm = async () => {
      try {
        const wasm = await import('../../public/static/wasm/pkg/front_wasm');
        await wasm.setup('canvas', '/image.png');
        wasm.draw_image(0, 0);
        
        let x = 0;
        let y = 0;
        
        const move = (dx: number, dy: number) => {
          x += dx;
          y += dy;
          wasm.draw_image(x, y);
        };

        document.addEventListener('keydown', (e) => {
          switch (e.key) {
            case 'ArrowUp':
              move(0, -50);
              break;
            case 'ArrowDown':
              move(0, 50);
              break;
            case 'ArrowLeft':
              move(-50, 0);
              break;
            case 'ArrowRight':
              move(50, 0);
              break;
          }
        });
      } catch (error) {
        console.error('Error loading wasm:', error);
      }
    };

    initWasm();
  }, []);

  return (
    <div>
      <canvas id="canvas" width="800" height="600"></canvas>
    </div>
  );
}
