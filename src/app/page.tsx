"use client";
import { useEffect, useRef } from 'react';
import { CanvasDrawer } from '../../public/static/wasm/pkg/front_wasm_bg';

export default function Home() {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const directionRef = useRef({ x: 0, y: 0 });
  let animationFrameId: number | null = null;

  const handleKeyDown = (event: KeyboardEvent) => {
          switch (event.key) {
            case 'ArrowUp':
              directionRef.current = { x: 0, y: -1 };
              break;
            case 'ArrowDown':
              directionRef.current = { x: 0, y: 1 };
              break;
            case 'ArrowLeft':
              directionRef.current = { x: -1, y: 0 };
              break;
            case 'ArrowRight':
              directionRef.current = { x: 1, y: 0 };
              break;
            default:
              break;
          }
        };

        const handleKeyUp = (event: KeyboardEvent) => {
          switch (event.key) {
            case 'ArrowUp':
            case 'ArrowDown':
            case 'ArrowLeft':
            case 'ArrowRight':
              directionRef.current = { x: 0, y: 0 };
              break;
            default:
              break;
          }
        };

  const step = 500; // 1秒あたりの移動量

  useEffect(() => {
    // Wasmの初期化
    import('../../public/static/wasm/pkg/front_wasm').then(() => {
      if (canvasRef.current) {
        // CanvasDrawerのインスタンス作成
        const drawer = new CanvasDrawer('myCanvas', '/image.png');
        
        // 初期描画
        drawer.draw_image();

        let lastTime = performance.now();


        const animate = () => {
          const now = performance.now();
          const deltaTime = (now - lastTime) / 1000; // 秒単位の経過時間
          lastTime = now;

          const { x, y } = directionRef.current;
          if (x !== 0 || y !== 0) {
            drawer.move_image(x * step * deltaTime, y * step * deltaTime);
          }
          animationFrameId = requestAnimationFrame(animate);
        };

        window.addEventListener('keydown', handleKeyDown);
        window.addEventListener('keyup', handleKeyUp);
        animate(); // Start the animation loop

        // クリーンアップ
        return () => {
          window.removeEventListener('keydown', handleKeyDown);
          window.removeEventListener('keyup', handleKeyUp);
          if (animationFrameId) cancelAnimationFrame(animationFrameId);
        };
      }
    }).catch(console.error);
  }, []);

  return <canvas id="myCanvas" ref={canvasRef} width="900" height="500"></canvas>;
}
