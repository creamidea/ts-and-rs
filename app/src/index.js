// import fs from 'node:fs';
// WebAssembly.instantiate(fs.readFileSync('../target/wasm32-unknown-unknown/debug/b.wasm'))
//   .then(wasm => {
//     console.log('wasm:', wasm)
//   })

// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './pkg';`
// will work here one day as well!
// const rust = import('./pkg')
const rust = import('hello-rust')

rust
  .then(m => {
    //  m.greet('World!') 
    console.log('m:', m, m.abs(-12))
  })
  .catch(console.error);