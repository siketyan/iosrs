# 🦀 iosrs
An example iOS App with Swift using Rust as a static library.

## 🚚 Prerequisites
- Rust Toolchain v1.53.0+
  - cargo-lipo
- Xcode v12.5.1+

## 📦 Usage
1. In `lib` directory, build the static library using cargo-lipo.
   ```
   $ cargo lipo --release
   ```

2. Open `app` directory with Xcode, then build the project.
3. Try on your an iOS device or an emulator!

## 🔗 References
- Building and Deploying a Rust library on iOS  
  https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-06-rust-on-ios.html
