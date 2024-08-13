# Android

requires `cross` to be installed:

```bash
cargo install cross
```

## Building

Depending on which CPU architecture you will need to specify different targets. Please refer to the
[Android ABIs](https://developer.android.com/ndk/guides/abis) for more details.

```bash
mkdir -p ./sdk/src/main/jniLibs/{arm64-v8a,armeabi-v7a,x86_64,x86}

cross build -p sdkrs-uniffi --release --target=aarch64-linux-android
mv ../../target/aarch64-linux-android/release/libsdkrs_uniffi.so ./sdk/src/main/jniLibs/arm64-v8a/libsdkrs_uniffi.so

cross build -p sdkrs-uniffi --release --target=armv7-linux-androideabi
mv ../../target/armv7-linux-androideabi/release/libsdkrs_uniffi.so ./sdk/src/main/jniLibs/armeabi-v7a/libsdkrs_uniffi.so

cross build -p sdkrs-uniffi --release --target=x86_64-linux-android
mv ../../target/x86_64-linux-android/release/libsdkrs_uniffi.so ./sdk/src/main/jniLibs/x86_64/libsdkrs_uniffi.so

cross build -p sdkrs-uniffi --release --target=i686-linux-android
mv ../../target/i686-linux-android/release/libsdkrs_uniffi.so ./sdk/src/main/jniLibs/x86/libsdkrs_uniffi.so
```

### Schemas

```bash
./build-schemas.sh
```

### Publish

1. Create a new release on GitHub
```bash
export GITHUB_ACTOR=username
export GITHUB_TOKEN=token

./gradlew sdk:publish
```

2. Build the SDK as an AAR
2.1 Open the project in Android Studio
2.2 Select build variant: Build -> Select Build Variant, change `:sdk` to `release`
2.3 Build the project: Build -> Rebuild Project
2.4 Find the AAR file in `sdk/build/outputs/aar/sdk-release.aar`
