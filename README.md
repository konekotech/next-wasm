# Next.js + WebAssembly (Rust)

This is a simple example of how to use WebAssembly with Next.js. It uses Rust to compile a simple function that returns a string.

## Getting Started

First, you need to install Rust and wasm-pack. You can find the instructions [here](https://rustwasm.github.io/wasm-pack/installer/).

## Compile the Rust code

```bash
cd ./front-wasm
./build.sh
```

## Run the Next.js app

```bash
npm run dev
```
