cargo run -p uniffi-bindgen generate \
  ./sdk/src/main/jniLibs/arm64-v8a/libsdkrs_uniffi.so \
  --library \
  --language kotlin \
  --no-format \
  --out-dir sdk/src/main/java
