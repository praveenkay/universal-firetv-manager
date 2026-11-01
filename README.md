# Universal Fire TV Manager

Backup-first launcher setup, conservative debloat, health checks, uninstall, and restore for Amazon Fire TV Sticks and Cubes.

> **Test release:** use `base-safe` on unknown devices. The project does not flash a replacement operating system and cannot back up private app logins on retail, non-root Fire OS.

## Quick Start

1. Enable ADB debugging on the Fire TV.
2. Install the desktop app, Android companion, or `firetvctl` CLI.
3. Enter the Fire TV's numeric IP address and approve the RSA prompt on the TV.
4. Inspect compatibility, select a launcher, and create the mandatory baseline backup.
5. Review the plan, install, then run health checks.

See [installation instructions](docs/INSTALL.md) for macOS, Windows, Linux, and Android.

## Enable ADB

On Fire TV, open **Settings → My Fire TV → Developer Options → ADB Debugging**. If Developer Options is hidden, open **About**, highlight the device name, and press Select seven times. Keep both devices on the same trusted network. The manager requires a manually entered numeric IP and never scans the network.

## Compatibility

- Exact signed profiles can enable model/build-specific changes.
- Unknown or changed builds fall back to `base-safe`.
- `base-safe` preserves Amazon Settings, accounts, DRM, Appstore, keyboard, Bluetooth, HDMI/TV inputs, audio services, and OTA components.
- Unsupported SDK/ABI combinations are blocked.

Run `firetvctl profiles FIRE_TV_IP` or follow the desktop compatibility screen. See [package profile validation](docs/PROFILE-VALIDATION.md).

## Install

| Launcher | Source | Status | Notes |
|---|---|---|---|
| Projectivy | [official releases](https://github.com/spocky/miproja1/releases) | Supported | User supplies the official APK; premium license is not bundled |
| dumbtv Home | [GitHub](https://github.com/humblemuzzu/dumbtv) | Recommended open source | Free GPL option |
| LTvLauncher | [GitHub](https://github.com/leanbitlab-org/LtvLauncher) | Experimental | Verify Home and Settings before keeping changes |
| Fire TV Web Launcher | [GitHub](https://github.com/pickdani/fire-tv-web-launcher) | Experimental | Only offered for validated Fire OS 8 profiles |

APKs are checked for package identity, signer, SDK, ABI, and optional SHA-256 before installation. Projectivy and Google Platform Tools are never redistributed.

## Demo

```bash
bash scripts/demo.sh --fixture tests/fixtures/raven-inspection.json
```

The deterministic demo uses redacted fixture data and performs no network or device mutation. See [demo details](docs/DEMO.md).

## Features

- Direct TCP ADB with a locally generated RSA key; no shell invocation of an `adb` executable.
- Manual IPv4/IPv6 target entry and authorization guidance.
- Read-only identity, launcher, settings, keyboard, TV input, capability, storage, and package inventory.
- Device-bound, hash-verified baseline backups with explicit backup limits.
- Guarded plans, confirmation hashes, transaction journals, rollback, uninstall, and settings restore.
- Post-change checks for Home, Settings, accounts, Appstore, keyboard, Bluetooth, TV inputs, storage, and Cube speaker capability.
- macOS app/DMG, Android arm64/armv7 sideload APKs, portable CLI, and source-build paths for Windows/Linux.

## Backup

Every mutation requires a matching baseline. A backup records device identity, package state, relevant settings (including missing values), hashes, and warnings. It does **not** contain private app databases, logins, DRM keys, Amazon account secrets, or a flashable Fire OS image. See [backup and restore](docs/BACKUP-RESTORE.md).

## Restore

Restore verifies every backup hash and exact device identity before applying saved settings. Missing original settings are deleted rather than written as empty values. Package state and launcher changes are restored only when the plan/profile contains a verified inverse operation.

## Uninstall

Use the guided **Uninstall changes** action or the confirmed CLI plan. The manager retains Amazon Home and critical Fire OS packages throughout. See [uninstall instructions](docs/UNINSTALL.md).

## Updates

After a Fire OS update, reconnect and inspect before changing anything. A fingerprint change returns the device to `base-safe`; capture a new baseline, generate a new plan hash, and rerun health checks.

## Package Profiles

Profiles declare supported SDK/ABI ranges, exact fingerprints, protected packages, optional changes, and launcher choices. Remote profiles require an Ed25519 signature. Evidence must be redacted and reproducible. See [profile validation](docs/PROFILE-VALIDATION.md).

## Limitations

- This is not a custom ROM and does not replace signed Fire OS firmware.
- Retail ADB cannot preserve or restore third-party app private data or logins.
- Amazon can change launcher behavior and protected services in an update.
- Home interception, Cube speaker routing, HDMI-CEC, and volume behavior vary by model/build and require real-device validation.
- The current test package connects and inspects; unknown builds remain conservative.
- The first prerelease does not include iOS; full Xcode and Apple signing are required and `scripts/package-ios.sh` is provided for that final package.

## Security

ADB keys remain in the OS application-data directory. Targets must be numeric addresses. Mutations require a baseline backup and either interactive approval or `--yes --plan-hash`. Report vulnerabilities through [SECURITY.md](SECURITY.md).

## Privacy

The manager operates locally and does not scan the LAN or upload device data. Exported reports redact serial, IP, and MAC identifiers. See [privacy details](docs/PRIVACY.md).

## Troubleshooting

If offline, confirm the current IP, ADB setting, same-network connectivity, and port `5555`. If unauthorized, accept the Fire TV RSA prompt. If Home, Settings, Appstore, playback, or audio fails, stop changes and restore the verified baseline. See [troubleshooting](docs/TROUBLESHOOTING.md).

## Development

```bash
cargo test --workspace
npm --prefix apps/desktop test
bash scripts/check-repository.sh
bash scripts/check-docs.sh
```

Original code is dual-licensed under MIT or Apache-2.0. Third-party launchers retain their upstream licenses.
