rustc -C opt-level=s --target wasm32-unknown-unknown ./src/lib.rs

wasm-opt -Oz -o libopt.wasm lib.wasm