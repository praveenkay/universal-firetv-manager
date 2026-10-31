# Universal Fire TV Manager

A reversible, compatibility-aware manager for Amazon Fire TV Sticks and Cubes.

The guided clients connect through authorized network ADB, inspect the device, require a device-bound backup, show every planned change, and support installation, health checks, update repair, uninstall, and exact restoration of settings changed by the manager.

## Status

Early development. Do not use on a daily device until a test release and matching compatibility profile are available.

## Launcher choices

The manager is designed to offer only choices compatible with the connected device:

- Projectivy Launcher, selected by the user from an official download.
- dumbtv home, a small open-source Fire TV launcher.
- LTvLauncher, initially experimental.
- Fire TV Web Launcher, initially limited to validated Fire OS 8 profiles.
- An advanced user-supplied launcher APK with explicit warnings.

Projectivy and Google Android SDK Platform Tools are not redistributed.

## Safety model

- No root, bootloader unlock, firmware flashing, factory reset, or app-data clearing.
- Unknown builds use the non-destructive `base-safe` profile.
- Amazon launcher and critical Fire OS services remain installed.
- Every mutation requires an integrity-checked, device-specific baseline backup.
- Retail ADB cannot back up private app data, logins, DRM keys, or Amazon account secrets.

The complete design and implementation plan are under `docs/superpowers/`.
