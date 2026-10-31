# Universal Fire TV Manager Design

Date: 2026-10-31

## Intent

Turn the current single-device shell toolkit into a guided, reversible Fire TV manager for Fire TV
Sticks and Cubes. A user enables ADB debugging, enters the device IP address, approves the Fire TV's
RSA authorization prompt, reviews compatibility findings, and then chooses backup, install, repair,
uninstall, or restore.

The product is distributed initially as sideloadable/test builds. Desktop packages are delivered for
macOS, Windows, and Linux; an Android companion APK follows; the iPhone/iPad package is last.

## Goals

- Detect the Fire TV model, Fire OS/Android version, ABI, available storage, package inventory,
  protected launcher, settings components, TV inputs, and supported ADB commands before changes.
- Require a successful device-specific baseline backup before installation or profile changes.
- Present a launcher catalog and install the user's compatible choice only when its APK/API/ABI,
  package, signing, and Fire OS integration requirements pass.
- Preserve Amazon launcher, settings, Appstore, accounts, DRM, keyboard, Bluetooth, remote, HDMI,
  Equipment Control, TV inputs, Alexa, recovery, and OTA/security services.
- Support install, update repair, health check, uninstall-custom-components, and exact restoration of
  every setting or package state changed by this product.
- Provide declarative, signed compatibility profiles that can be validated on individual models and
  Fire OS builds without releasing a new application binary.
- Produce reproducible macOS, Windows, Linux, Android, and eventually iOS test packages.

## Non-goals

- No rooting, bootloader unlocking, firmware flashing, replacement ROM, or system partition changes.
- No promise that private app data, logins, DRM keys, or Amazon account secrets can be backed up over
  retail non-root ADB.
- No destructive universal debloat list. Unknown builds receive the safe base installation only.
- No silent network scanning in the first release; users enter an IP address manually.
- No App Store or Play Store publication in the initial release.

## Anti-goals

- Never use one Fire TV's baseline snapshot to restore another device.
- Never disable a package merely because its name appears in a profile; all guards must pass at run
  time and the package must exist in the current baseline.
- Never claim success without post-operation verification.
- Never clear app data, factory-reset the device, or remove user-installed streaming applications.

## Constraints

- Fire TV requires ADB debugging, a same-network connection, an IP address, and explicit RSA approval.
- Retail ADB cannot read private application databases or make a full flashable firmware backup.
- Fire OS components and package names vary by model, Fire OS generation, build, and region.
- Amazon's launcher remains installed because it provides protected system functions and settings.
- iOS local TCP connections require Local Network permission and a user-visible usage description.
- Mobile clients need an embedded ADB protocol implementation; they cannot assume a host `adb`
  executable is installed.

## Approaches Considered

### Selected: shared Rust core with thin platform clients

A Rust library owns device detection, ADB operations, backup manifests, compatibility evaluation,
transactions, restore logic, and verification. Desktop uses a Tauri UI and may use a bundled,
checksum-verified Android Platform Tools `adb` transport initially. Android and iOS call the same core
through generated bindings and use an embedded TCP ADB transport.

This provides one safety implementation and small desktop packages, while allowing native network and
storage integration on mobile. The tradeoff is more initial engineering and mobile FFI work.

### Rejected: separate native implementations

Swift, Kotlin, PowerShell, Bash, and another desktop implementation would ship sooner per platform,
but duplicate the most safety-critical logic and cause backup/restore behavior to diverge.

### Rejected: browser or PWA

Browser security prevents a reliable raw TCP ADB workflow and dependable local backup storage across
all target platforms.

## Architecture

### Workspace

- `core/` — Rust library containing models, profile evaluation, operation planning, transactions,
  backup/restore, verification, and transport interfaces.
- `profiles/` — versioned JSON profiles plus JSON Schema and detached release checksums/signatures.
- `apps/desktop/` — Tauri application for macOS, Windows, and Linux.
- `apps/android/` — Android application and Rust bindings.
- `apps/ios/` — iPhone/iPad application and Rust bindings, implemented last.
- `packages/catalog/` — launcher/helper metadata with source, license, version, compatibility,
  acquisition method, package identity, signing identity, and SHA-256 metadata; proprietary APKs
  are never stored in Git.
- `tests/fixtures/` — redacted device inventories and ADB transcript fixtures.
- `tests/mock-adb/` — deterministic fake ADB server for failure and rollback testing.
- `scripts/` — reproducible build, signing, checksum, and packaging entrypoints.

### Core services

1. **Connection service** validates `IPv4[:port]` or `[IPv6]:port`, defaults to port 5555, connects,
   reports unauthorized/offline states, and guides RSA approval.
2. **Device inspector** captures immutable identity and capability facts before selecting a profile.
3. **Profile engine** returns `supported`, `base-only`, or `blocked`, with human-readable reasons.
4. **Planner** shows every proposed install, setting change, and package-state change before execution.
5. **Backup service** writes a device-bound manifest and all accessible metadata/APKs/shared files.
6. **Transaction engine** journals each successful mutation and rolls back completed steps on failure.
7. **Restore service** accepts only backups whose device identity and manifest integrity match.
8. **Verifier** checks launcher access, settings, Appstore, account records, keyboard, Bluetooth,
   remotes, HDMI/TV inputs, network, storage, and selected streaming-app launchability.

## Device Identity and Backup Format

Each backup directory contains:

- `manifest.json` with schema version, application version, capture time, device serial, model,
  codename, fingerprint, Fire OS version, Android SDK, ABI, region facts, and SHA-256 file inventory.
- Package lists and exact enabled/disabled/removed-for-user state.
- Relevant `global`, `secure`, and `system` setting values, including explicit absence.
- Home activity, accessibility services, TV-input services, input method, Bluetooth/remote, HDMI,
  audio, storage, memory, and network diagnostics.
- Readable third-party APKs and shared storage when full backup is selected.
- `changes.jsonl`, recording old value, requested value, result, and rollback result per mutation.
- `BACKUP-LIMITS.md`, clearly describing data unavailable through retail ADB.

Restore rejects a mismatched serial/fingerprint unless the user selects a separate, limited
"restore portable user APKs" operation. System package and settings restoration is never portable.

## Compatibility Profiles

Profiles are additive and ordered from safest to most specific:

1. `base-safe`: no Amazon package disabling; installs verified custom components and applies only
   reversible settings that exist and are writable.
2. Fire OS generation profile: declares known commands, settings, launcher behavior, and protected
   components for Fire OS 5, 6, 7, or later.
3. Model-family profile: Stick/Cube-specific capabilities such as Equipment Control, internal
   speaker, TV inputs, and launcher interception behavior.
4. Exact-build profile: optional package-disable candidates validated against a specific fingerprint
   or tightly bounded build range.

Every profile includes selectors, required facts, APK compatibility, protected packages, optional
changes, verification assertions, rollback actions, evidence records, and validation status.

Unknown models or builds use `base-safe`. Exact-build debloating is opt-in and unavailable until its
profile reaches `validated` status.

## Launcher Catalog

The Compatibility screen shows only launchers whose declared and observed requirements match the
connected device. Each entry identifies whether it is proprietary/free-to-use or open source, its
upstream repository/site, supported Fire OS range, Home-button method, settings/backup support, and
whether the manager may download it or the user must select an APK.

The initial catalog contains:

- **Projectivy Launcher** — polished proprietary/freemium option; the user selects an APK obtained
  from its official source. The manager never redistributes it.
- **dumbtv home** — small GPL open-source Fire TV launcher from GitHub with no internet permission;
  installed from a checksum-pinned upstream release or reproducible source build.
- **LTvLauncher** — open-source customizable Android/Fire TV launcher from GitHub; initially marked
  experimental until its Home integration passes each Fire OS profile's validation suite.
- **Fire TV Web Launcher** — open-source customizable web-based Fire TV launcher from GitHub;
  initially offered only on validated Fire OS 8 profiles.
- **User-supplied launcher APK** — advanced option requiring Leanback/Home activity inspection,
  package/signature reporting, explicit compatibility warning, and no automatic Home takeover unless
  a validated integration recipe exists.

Catalog entries are signed data and can be added without changing core safety rules. The user may
change launchers later; the transaction backs up the current launcher configuration and removes only
the integration owned by the prior selection.

## Package Profile Validation Procedure

1. **Acquire a clean baseline**
   - Update the test device, reboot, confirm normal Amazon Home, network, remote, audio, Bluetooth,
     Appstore installation, account access, settings, and representative streaming playback.
   - Capture a full accessible baseline backup and record the build fingerprint.
2. **Generate a candidate inventory**
   - Record every package, installer, version, enabled state, permissions, services, receivers, Home
     activity, TV inputs, settings components, and package dependencies visible to ADB.
3. **Apply critical guards**
   - Automatically exclude launchers, settings, accounts, DRM, Appstore, keyboard, Bluetooth,
     remotes, HDMI/device control, TV inputs, Alexa when present, recovery, and OTA/security packages.
4. **Classify each candidate**
   - Document purpose and evidence; mark it `protected`, `keep`, `optional-disable`, or `unknown`.
     Unknown packages are kept.
5. **Test one mutation at a time**
   - Snapshot, disable one candidate for user 0, reboot, and execute the complete verification suite.
     Never uninstall a system package during validation.
6. **Run functional verification**
   - Test Home/Back/D-pad/playback/voice/volume/power, settings, Bluetooth pairing, HDMI-CEC and
     Equipment Control, internal speaker when present, network reconnection, Appstore installation,
     login persistence, DRM playback, Prime Video, YouTube/SmartTube, and at least one third-party app.
7. **Run soak and update tests**
   - Use the device for at least 24 hours, reboot at least three times, sleep/wake it, change network,
     and verify behavior before and after an available Fire OS update.
8. **Restore and prove reversibility**
   - Restore the pre-test snapshot, reboot, rerun verification, and compare package/settings state to
     the baseline. Any unexplained delta fails validation.
9. **Repeat across devices/builds**
   - One successful device is `experimental`; two devices on the same fingerprint are `candidate`;
     at least two models or three physical devices in the declared selector range with complete
     restore evidence are required for `validated`.
10. **Sign and publish**
    - Review the profile diff, attach redacted evidence and test results, increment its version, sign
      the release manifest, and publish checksums. Failed assertions automatically quarantine it.

## User Workflows

### First connection

1. Show Fire TV instructions: enable Developer Options and ADB Debugging; locate the IP under About
   > Network.
2. Ask for the IP address and optional port.
3. Attempt connection and display "Approve this computer" instructions when unauthorized.
4. Inspect the device and show model, Fire OS, fingerprint, support level, and unavailable features.
5. Require user confirmation before backup or mutation.

### Install

1. Run preflight and require sufficient storage/network health.
2. Create and integrity-check a device-specific baseline backup.
3. Display the operation plan.
4. Present compatible launcher choices, install the selected verified launcher and its validated
   Home-button integration, and install Kara Settings where compatible.
5. Apply the safe privacy settings; apply validated profile changes only when separately enabled.
6. Guide the one-time Projectivy selection if required.
7. Reboot only when necessary and run verification.
8. Roll back automatically if a required verification fails.

### Uninstall

1. Snapshot current state.
2. Remove only accessibility entries added by the product.
3. Uninstall Projectivy, Home on Fire, and Kara Settings only if installed by the product or explicitly
   selected by the user.
4. Restore Amazon Home and the safe settings changed by the product.
5. Leave user apps and unrelated settings untouched; verify stock Home and critical services.

### Restore

1. Select a backup and verify hashes and device identity.
2. Show the exact restore plan.
3. Restore package enabled/disabled states and each recorded setting to its prior value or absence.
4. Restore optional custom components only when the selected backup contained them.
5. Reboot and compare post-restore facts to the backup baseline.

### Fire OS update repair

1. Detect fingerprint change and automatically demote an exact-build profile until revalidated.
2. Back up the updated state.
3. Apply `base-safe`, reinstall/repair verified custom components, and verify.
4. Offer model-specific changes only if a validated profile covers the new build.

## Security and Privacy

- Keep ADB keys in platform-protected storage: Keychain on Apple platforms, Keystore on Android, and
  OS credential storage on desktop.
- Keep backups local by default; provide explicit export/import with hashes.
- Never upload device inventory, IP addresses, account information, or backups.
- Verify all bundled APKs and platform tools against pinned SHA-256 values before use.
- Display third-party source and license information in the application.
- Redact account names, Wi-Fi identifiers, Bluetooth addresses, and device serials from exported
  diagnostic reports unless the user explicitly includes them.

## UI

The clients share the same seven-step flow: Welcome, Enable ADB, Connect, Compatibility, Backup,
Choose Operation, Results. Advanced users can inspect the full plan and export diagnostics. Dangerous
experimental profile actions require a second confirmation and are visually separated from the safe
installation.

## Packaging Order

1. Extract and test the shared core with mock ADB fixtures.
2. Ship a portable CLI used by automated tests and recovery.
3. Build the desktop UI and unsigned/test macOS, Windows, and Linux packages.
4. Build and sideload the Android APK; test on physical Android phones and the current Fire TV Cube.
5. Expand the physical Fire TV validation matrix and publish validated profiles.
6. Build the iPhone/iPad app last, with Local Network permission and TestFlight/ad-hoc signing.

## Verification and Release Gates

- Unit tests cover IP validation, profile selection, planner guards, manifest hashing, identity
  matching, transaction rollback, and settings absence restoration.
- Integration tests run every workflow against the mock ADB server, including disconnects,
  unauthorized devices, unsupported commands, corrupt APKs, low storage, and rollback failures.
- Desktop smoke tests run on current macOS, Windows, and Ubuntu runners.
- Android tests run on an emulator for UI/storage plus a physical phone for local-network ADB.
- iOS tests run on a simulator for UI/backup handling and a physical iPhone for Local Network ADB.
- Real-device tests first use the current `raven` Cube, then at least one Stick from each supported
  Fire OS generation before a profile is marked validated.
- A package is releasable only when install, health check, update repair, uninstall, and exact restore
  pass; the restore comparison must show no unexplained package or setting deltas.

## Migration of the Existing Toolkit

The existing scripts and backups remain unchanged as evidence and emergency recovery tools. Their
hardcoded IP, original snapshot, and device-specific restore behavior are not carried into the new
engine. The current Cube supplies the first redacted fixture and experimental `raven` profile only
after a fresh baseline and full validation run.

## References

- Amazon Fire TV documentation: connecting through ADB and sideloading applications.
- Android Developers: ADB authorization, device states, and wireless connectivity.
- Apple Developer TN3179: Local Network privacy and `NSLocalNetworkUsageDescription`.

