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

import calc from './lib/index.js';
// const add = require('./lib/calc');
console.log('calc:', calc)


rust
  .then(m => {
    //  m.greet('World!')
    console.log('m:', m.abs(-12))
    // var a = 100;
    console.log('eval:', m.eval('arg0,arg1'))
    // console.log(eval('a+1'))
    console.log(m.decode('%2fhello?'))
  })
  .catch(console.error);

// import { abs } from 'hello-rust'
// console.log(abs(-12))
// console.log('hhhh')
