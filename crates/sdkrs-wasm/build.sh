# Move to the root of the repository
cd "$(dirname "$0")"
cd ../../

if [ "$1" != "-r" ]; then
  # Dev
  cargo build -p sdkrs-wasm --target wasm32-unknown-unknown
  wasm-bindgen --target bundler --out-dir languages/js/wasm ./target/wasm32-unknown-unknown/debug/sdkrs_wasm.wasm
  wasm-bindgen --target nodejs --out-dir languages/js/wasm/node ./target/wasm32-unknown-unknown/debug/sdkrs_wasm.wasm
else
  # Release
  cargo build -p sdkrs-wasm --target wasm32-unknown-unknown --release
  wasm-bindgen --target bundler --out-dir languages/js/wasm ./target/wasm32-unknown-unknown/release/sdkrs_wasm.wasm
  wasm-bindgen --target nodejs --out-dir languages/js/wasm/node ./target/wasm32-unknown-unknown/release/sdkrs_wasm.wasm
fi

# Optimize size
wasm-opt -Os ./languages/js/wasm/sdkrs_wasm_bg.wasm -o ./languages/js/wasm/sdkrs_wasm_bg.wasm
wasm-opt -Os ./languages/js/wasm/node/sdkrs_wasm_bg.wasm -o ./languages/js/wasm/node/sdkrs_wasm_bg.wasm

# Transpile to JS
wasm2js ./languages/js/wasm/sdkrs_wasm_bg.wasm -o ./languages/js/wasm/sdkrs_wasm_bg.wasm.js
npx terser ./languages/js/wasm/sdkrs_wasm_bg.wasm.js -o ./languages/js/wasm/sdkrs_wasm_bg.wasm.js