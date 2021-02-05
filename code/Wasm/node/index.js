const wasm = require('../pkg/Wasm')
const md5js = require('./md5/index');

const generateStr = (len) => {
  let str = ""
  let chars = "qwertyuiopasdfghjklzxcvbnm1234567890QWERTYUIOPASDFGHJKLZXCVBNM"
  for(let i = 0; i < len; i++) {
    str += chars[~~(Math.random() * 62)]
  }
  return str;
}


const testName = generateStr(100000);

console.time("js")
let jsResult = md5js.MD5(testName)
console.timeEnd("js")
console.time("wasm")
let wasmResult = wasm.md5rust(testName)
console.timeEnd("wasm")

console.log(jsResult, wasmResult)
// console.log(wasm.say_hello())