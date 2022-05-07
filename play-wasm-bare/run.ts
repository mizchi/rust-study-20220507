// @ts-ignore
const x = fetch(new URL("./libopt.wasm", import.meta.url));
// @ts-ignore
const wasm = await WebAssembly.instantiateStreaming(x);
// @ts-ignore
console.log(wasm.instance.exports.the_answer());