# Installation

## macOS

Open the test DMG and drag **Universal Fire TV Manager** to Applications. Test builds are not notarized, so Control-click and choose Open the first time. Source build: `bash scripts/package-desktop.sh`.

## Windows

Install Rust 1.90+, Node 22+, WebView2, and Tauri prerequisites. Run `npm --prefix apps/desktop ci` then `npm --prefix apps/desktop run tauri build`, and use the generated MSI/NSIS package.

## Linux

Install Rust 1.90+, Node 22+, WebKitGTK, and Tauri system dependencies. Run `bash scripts/package-desktop.sh`; install the AppImage or deb.

## Android

Allow unknown-source installation for the app opening the APK. Use the `arm64` APK on modern devices and `armv7` on older 32-bit ARM devices. Test APKs are release-optimized but signed with a disclosed test key, not a store identity. This companion controls another Fire TV over the LAN; it is not a launcher.

## CLI

Run `cargo build --release -p firetvctl`; the binary is `target/release/firetvctl` (`.exe` on Windows).

## First connection

Enable ADB debugging, find the numeric IP under **Settings → My Fire TV → About → Network**, enter it manually, and approve the RSA prompt on the television.
