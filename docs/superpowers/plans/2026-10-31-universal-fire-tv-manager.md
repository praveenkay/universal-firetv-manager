# Universal Fire TV Manager Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build and publish a reversible Fire TV management suite with a verified launcher catalog, shared Rust safety core, portable CLI, macOS/Windows/Linux desktop packages, an Android sideload APK, and an iPhone/iPad test package implemented last.

**Architecture:** A Rust workspace owns ADB transport, device inspection, compatibility profiles, backups, transaction journaling, rollback, and verification. Tauri, Android, and iOS clients expose the same guided workflow through UniFFI bindings; the first desktop and Android releases use direct TCP ADB through `rsadb`. Third-party proprietary APKs and Google Platform Tools are not redistributed.

**Tech Stack:** Rust 1.85+, Tokio, rsadb, Serde/JSON Schema, Ed25519 profile signatures, Clap, Tauri 2, TypeScript/React, Kotlin/Jetpack Compose, Swift/SwiftUI, UniFFI, GitHub Actions.

**Spec:** `docs/superpowers/specs/2026-10-31-universal-fire-tv-manager-design.md`

## Global Constraints

- The publishable repository root is `/Users/om/Documents/Firetv-Debloat/universal-firetv-manager`; the existing parent toolkit, backups, installers, screenshots, and device reports remain outside its Git history.
- Never root, unlock, flash, alter system partitions, clear app data, factory-reset, or remove user applications.
- Unknown Fire TV builds receive `base-safe` only; package disabling requires a validated exact-build profile and explicit opt-in.
- A verified, device-bound baseline backup is mandatory before every mutating operation.
- Amazon launcher, settings, accounts, DRM, Appstore, keyboard, Bluetooth, remote, HDMI/device control, TV inputs, recovery, and OTA/security packages remain protected.
- System/package-state restore is accepted only for the matching device serial and build identity.
- Private app data, logins, DRM keys, and account secrets are explicitly outside backup capability.
- Projectivy is never stored in or distributed from the repository; users select a locally obtained official APK and its package/signing identity is verified.
- The launcher chooser initially covers Projectivy, dumbtv home, LTvLauncher, Fire TV Web Launcher,
  and an advanced user-supplied APK; only profile-compatible choices are enabled.
- Google Platform Tools are not redistributed; direct TCP ADB uses the open-source `rsadb` library.
- Initial releases are unsigned/test or sideload builds; iPhone/iPad is implemented after desktop and Android pass real-device tests.

## Review Focus

- Malformed, IPv6, missing-port, and out-of-subnet addresses must fail safely without command injection; Task 2 tests all forms.
- An unauthorized or disconnected Fire TV must produce actionable status and perform no mutation; Task 3 tests both states.
- A backup with valid JSON but altered files or a different device identity must be rejected; Task 5 tests hashes and identity.
- Mid-operation disconnects and failed verification must roll back only completed mutations and retain the journal; Task 6 tests both failures.
- A new Fire OS fingerprint after an update must demote exact-build changes to `base-safe`; Task 4 tests profile quarantine.

---

### Task 1: Initialize the publishable workspace

**Files:**
- Create: `Cargo.toml`
- Create: `rust-toolchain.toml`
- Create: `.gitignore`
- Create: `.editorconfig`
- Create: `LICENSE`
- Create: `NOTICE.md`
- Create: `SECURITY.md`
- Create: `CONTRIBUTING.md`
- Create: `core/Cargo.toml`
- Create: `core/src/lib.rs`
- Create: `scripts/check-repository.sh`
- Create: `tests/fixtures/README.md`
- Create: `docs/superpowers/specs/2026-10-31-universal-fire-tv-manager-design.md`
- Create: `docs/superpowers/plans/2026-10-31-universal-fire-tv-manager.md`
- Modify: `README.md`

**Interfaces:**
- Consumes: approved design and existing recovery scripts as reference only.
- Produces: Cargo workspace member `firetv-core` and repository policy that excludes APKs, keys, backups, device identifiers, build outputs, and the legacy private toolkit.

- [ ] **Step 1: Write the repository policy test**

Create `scripts/check-repository.sh` asserting tracked files do not contain `*.apk`, `adbkey*`, `backups/`, unredacted serials, or the current device IP.

- [ ] **Step 2: Run the policy test and verify failure**

Run: `bash scripts/check-repository.sh`
Expected: FAIL because workspace policy files and publishable structure do not yet exist.

- [ ] **Step 3: Initialize the workspace and repository policy**

Create the clean `universal-firetv-manager/` repository directory. Use Rust edition 2024, MSRV 1.85, dual MIT/Apache-2.0 licensing for original code, and explicit third-party notices. Copy the approved design and plan into its `docs/superpowers/` tree while preserving the legacy toolkit outside Git history.

- [ ] **Step 4: Run workspace and policy checks**

Run: `cargo metadata --no-deps --format-version 1 && bash scripts/check-repository.sh`
Expected: both succeed and no proprietary APK or device backup is publishable.

- [ ] **Step 5: Commit**

```bash
git add Cargo.toml rust-toolchain.toml .gitignore .editorconfig LICENSE NOTICE.md SECURITY.md CONTRIBUTING.md core scripts README.md tests/fixtures/README.md docs
git commit -m "chore: initialize universal Fire TV manager workspace"
```

### Task 2: Define device, address, and support models

**Files:**
- Create: `core/src/address.rs`
- Create: `core/src/device.rs`
- Create: `core/src/error.rs`
- Modify: `core/src/lib.rs`
- Test: `core/tests/address_device.rs`

**Interfaces:**
- Consumes: none.
- Produces: `TargetAddress::parse(&str) -> Result<TargetAddress>`, `DeviceIdentity`, `DeviceFacts`, `SupportLevel::{Supported, BaseOnly, Blocked}`, and `ManagerError`.

- [ ] **Step 1: Write failing address and model tests**

Test default port 5555, explicit IPv4 port, bracketed IPv6, whitespace rejection, hostname rejection, shell metacharacter rejection, invalid ports, and stable serialization of `DeviceIdentity`.

- [ ] **Step 2: Verify tests fail**

Run: `cargo test -p firetv-core --test address_device`
Expected: FAIL because the types do not exist.

- [ ] **Step 3: Implement the exact public types and parsers**

Ensure addresses are passed as structured socket values and never interpolated into shell commands.

- [ ] **Step 4: Verify tests pass**

Run: `cargo test -p firetv-core --test address_device`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add core
git commit -m "feat: add device identity and safe target parsing"
```

### Task 3: Implement direct TCP ADB and device inspection

**Files:**
- Create: `core/src/adb/mod.rs`
- Create: `core/src/adb/rsadb_transport.rs`
- Create: `core/src/adb/mock.rs`
- Create: `core/src/inspect.rs`
- Modify: `core/Cargo.toml`
- Modify: `core/src/lib.rs`
- Test: `core/tests/connection_inspection.rs`
- Create: `tests/fixtures/raven-inspection.json`

**Interfaces:**
- Consumes: `TargetAddress`, `DeviceFacts`, and `ManagerError` from Task 2.
- Produces: async trait `AdbTransport::{connect, shell, push, pull, install, uninstall}`, `ConnectionState::{Connected, Unauthorized, Offline}`, and `inspect_device(&dyn AdbTransport) -> Result<DeviceFacts>`.

- [ ] **Step 1: Write failing connection-state and inspection tests**

Use `MockAdbTransport` to test unauthorized, offline, missing properties, command failure, and a redacted `raven` fixture. Assert unauthorized/offline cases execute no inspection mutations.

- [ ] **Step 2: Verify tests fail**

Run: `cargo test -p firetv-core --test connection_inspection`
Expected: FAIL because transport and inspector do not exist.

- [ ] **Step 3: Implement the transport abstraction and `rsadb` adapter**

Use direct TCP/RSA authentication, platform-provided secure key paths, bounded timeouts, shell v2 exit codes, and sync push/pull. Do not invoke a local `adb` binary.

- [ ] **Step 4: Implement read-only device inspection**

Capture model, codename, serial, fingerprint, Fire OS value, SDK, ABI list, packages/states, resolved Home, settings activities, TV inputs, input method, accessibility, Bluetooth/remote, HDMI/audio, network, and storage facts.

- [ ] **Step 5: Verify tests pass**

Run: `cargo test -p firetv-core --test connection_inspection`
Expected: PASS.

- [ ] **Step 6: Commit**

```bash
git add core tests/fixtures/raven-inspection.json
git commit -m "feat: add direct ADB transport and device inspection"
```

### Task 4: Add signed compatibility profiles and critical guards

**Files:**
- Create: `profiles/schema/profile.schema.json`
- Create: `profiles/base-safe.json`
- Create: `profiles/fire-os-7.json`
- Create: `profiles/experimental/raven.json`
- Create: `profiles/PROFILE-VALIDATION.md`
- Create: `core/src/profile.rs`
- Create: `core/src/guards.rs`
- Modify: `core/src/lib.rs`
- Test: `core/tests/profile_selection.rs`
- Test: `core/tests/critical_guards.rs`

**Interfaces:**
- Consumes: `DeviceFacts` and `SupportLevel`.
- Produces: `Profile`, `ProfileStatus`, `ProfileStore::select(&DeviceFacts) -> Selection`, and `CriticalGuards::allows(&Mutation, &DeviceFacts) -> GuardDecision`.

- [ ] **Step 1: Write failing profile-selection tests**

Assert exact fingerprint selection, unknown build fallback to `base-safe`, changed fingerprint quarantine, invalid signature rejection, and unsupported APK SDK/ABI rejection.

- [ ] **Step 2: Write failing critical-guard tests**

Assert protected categories, current Home, active input method, registered TV-input packages, and absent/unknown packages can never be disabled.

- [ ] **Step 3: Verify tests fail**

Run: `cargo test -p firetv-core --test profile_selection --test critical_guards`
Expected: FAIL because profiles and guards do not exist.

- [ ] **Step 4: Implement schema, signed profile loading, selection, and guards**

Keep `raven` status `experimental`; `base-safe` contains no system-package disabling. Include the approved ten-step validation procedure in `profiles/PROFILE-VALIDATION.md`.

- [ ] **Step 5: Verify tests and schema pass**

Run: `cargo test -p firetv-core --test profile_selection --test critical_guards`
Expected: tests pass and all bundled profiles validate against the embedded schema/signing rules.

- [ ] **Step 6: Commit**

```bash
git add profiles core
git commit -m "feat: add signed compatibility profiles and safety guards"
```

### Task 5: Implement device-bound backup manifests

**Files:**
- Create: `core/src/backup.rs`
- Create: `core/src/redaction.rs`
- Modify: `core/src/lib.rs`
- Test: `core/tests/backup_integrity.rs`

**Interfaces:**
- Consumes: `AdbTransport`, `DeviceFacts`, `DeviceIdentity`.
- Produces: `BackupManifest`, `BackupOptions`, `capture_backup(...) -> Result<BackupManifest>`, `verify_backup(path, identity) -> Result<VerifiedBackup>`, and `export_redacted_report(...)`.

- [ ] **Step 1: Write failing backup tests**

Assert metadata capture, explicit missing-setting values, SHA-256 inventory, tampered-file rejection, mismatched-device rejection, atomic incomplete-backup cleanup, and diagnostic redaction.

- [ ] **Step 2: Verify tests fail**

Run: `cargo test -p firetv-core --test backup_integrity`
Expected: FAIL because backup APIs do not exist.

- [ ] **Step 3: Implement atomic backup capture and verification**

Use a temporary directory plus final rename; include `BACKUP-LIMITS.md`; make full backup APK/shared-storage failures nonfatal but recorded.

- [ ] **Step 4: Verify tests pass**

Run: `cargo test -p firetv-core --test backup_integrity`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add core
git commit -m "feat: add device-bound backup integrity"
```

### Task 6: Build planner, transaction journal, rollback, and restore

**Files:**
- Create: `core/src/operation.rs`
- Create: `core/src/planner.rs`
- Create: `core/src/transaction.rs`
- Create: `core/src/restore.rs`
- Modify: `core/src/lib.rs`
- Test: `core/tests/transaction_restore.rs`

**Interfaces:**
- Consumes: `VerifiedBackup`, `Selection`, `CriticalGuards`, and `AdbTransport`.
- Produces: `Operation::{Install, Repair, Uninstall, Restore}`, `OperationPlan`, `TransactionJournal`, `Planner::plan(...)`, `Executor::execute(...)`, and `restore_backup(...)`.

- [ ] **Step 1: Write failing planning and rollback tests**

Assert mandatory backup, plan transparency, old-value capture, sequential journaling, mid-operation disconnect rollback, failed-verification rollback, rollback-failure retention, and exact restoration of absent settings.

- [ ] **Step 2: Verify tests fail**

Run: `cargo test -p firetv-core --test transaction_restore`
Expected: FAIL because operations do not exist.

- [ ] **Step 3: Implement guarded planning and transactional execution**

Only execute mutations included in a confirmed plan. Journal before/after/result for each step and rollback completed reversible steps in reverse order.

- [ ] **Step 4: Implement identity-bound restore**

Restore package state and settings only from a verified matching backup; portable APK restoration is a separate operation and never restores system state.

- [ ] **Step 5: Verify tests pass**

Run: `cargo test -p firetv-core --test transaction_restore`
Expected: PASS.

- [ ] **Step 6: Commit**

```bash
git add core
git commit -m "feat: add transactional install uninstall and restore"
```

### Task 7: Add APK acquisition, identity checks, and health verification

**Files:**
- Create: `packages/manifest.json`
- Create: `packages/catalog/launchers.json`
- Create: `packages/catalog/launcher.schema.json`
- Create: `packages/README.md`
- Create: `core/src/apk.rs`
- Create: `core/src/verify.rs`
- Modify: `core/src/lib.rs`
- Test: `core/tests/apk_verification.rs`
- Test: `core/tests/health_verification.rs`

**Interfaces:**
- Consumes: `AdbTransport`, `DeviceFacts`, `OperationPlan`.
- Produces: `LauncherCatalog`, `LauncherEntry`, `compatible_launchers(&DeviceFacts)`,
  `ApkRequirement`, `VerifiedApk`, `verify_apk(path, requirement)`, `HealthReport`, and `verify_device(...)`.

- [ ] **Step 1: Write failing APK verification tests**

Assert catalog filtering, Projectivy user-selection guidance, pinned open-source release metadata,
wrong package, wrong signer, unsupported SDK/ABI, altered checksum, and advanced user-supplied APK
warnings.

- [ ] **Step 2: Write failing health tests**

Assert Home, settings, Appstore, account records, keyboard, Bluetooth remote, HDMI/TV inputs, network, storage, and launcher hook checks; unavailable model capabilities report `not-applicable`, not failure.

- [ ] **Step 3: Verify tests fail**

Run: `cargo test -p firetv-core --test apk_verification --test health_verification`
Expected: FAIL.

- [ ] **Step 4: Implement APK and health verification**

Store only metadata and official source URLs in Git. Require the user to select Projectivy. Add
dumbtv home as the recommended fully open-source option, LTvLauncher as experimental, and Fire TV Web
Launcher only for validated Fire OS 8 profiles. Allow GPL Home on Fire and Kara Settings to be built
from pinned source during releases instead of committing binaries.

- [ ] **Step 5: Verify tests pass**

Run: `cargo test -p firetv-core --test apk_verification --test health_verification`
Expected: PASS.

- [ ] **Step 6: Commit**

```bash
git add packages core
git commit -m "feat: verify installer payloads and device health"
```

### Task 8: Ship the portable recovery CLI

**Files:**
- Create: `cli/Cargo.toml`
- Create: `cli/src/main.rs`
- Create: `cli/src/commands/*.rs`
- Modify: `Cargo.toml`
- Test: `cli/tests/cli.rs`

**Interfaces:**
- Consumes: all `firetv-core` services.
- Produces: `firetvctl connect|inspect|profiles|backup|plan|install|repair|health|uninstall|restore` with JSON and human-readable output.

- [ ] **Step 1: Write failing CLI contract tests**

Test help, safe IP parsing, unauthorized guidance, JSON output, required confirmation token, backup-before-mutation, dry-run plan, and nonzero exit codes.

- [ ] **Step 2: Verify tests fail**

Run: `cargo test -p firetvctl`
Expected: FAIL because the CLI does not exist.

- [ ] **Step 3: Implement command modules**

Every mutation requires `--backup` plus interactive confirmation, or an explicit `--yes --plan-hash` pair for automation.

- [ ] **Step 4: Verify tests pass**

Run: `cargo test -p firetvctl && cargo run -p firetvctl -- --help`
Expected: PASS and complete command help.

- [ ] **Step 5: Commit**

```bash
git add Cargo.toml cli
git commit -m "feat: add portable Fire TV recovery CLI"
```

### Task 9: Build the desktop application and packages

**Files:**
- Create: `apps/desktop/src-tauri/**`
- Create: `apps/desktop/src/**`
- Create: `apps/desktop/package.json`
- Create: `apps/desktop/vite.config.ts`
- Create: `apps/desktop/tests/**`
- Create: `scripts/package-desktop.sh`
- Modify: `Cargo.toml`

**Interfaces:**
- Consumes: UniFFI/API wrapper over `firetv-core`.
- Produces: guided seven-screen Tauri application and macOS `.dmg`, Windows installer, Linux AppImage/deb artifacts.

- [ ] **Step 1: Write failing desktop workflow tests**

Test Enable ADB instructions, manual IP entry, unauthorized retry, compatibility result, compatible
launcher selection and source/license display, mandatory backup, plan confirmation, results,
uninstall, restore, and unknown-device `base-safe` messaging.

- [ ] **Step 2: Verify tests fail**

Run: `npm --prefix apps/desktop test`
Expected: FAIL because the UI does not exist.

- [ ] **Step 3: Implement Tauri commands and React workflow**

Keep advanced plan details accessible, separate experimental changes visually, and store keys/backups in OS-appropriate application directories.

- [ ] **Step 4: Verify desktop tests and local package**

Run: `npm --prefix apps/desktop test && npm --prefix apps/desktop run tauri build`
Expected: tests pass and a macOS test package is produced on the current host.

- [ ] **Step 5: Commit**

```bash
git add apps/desktop scripts/package-desktop.sh Cargo.toml
git commit -m "feat: add cross-platform desktop manager"
```

### Task 10: Build the Android sideload companion

**Files:**
- Create: `bindings/uniffi/**`
- Create: `apps/android/**`
- Create: `scripts/package-android.sh`
- Modify: `Cargo.toml`
- Test: `apps/android/app/src/test/**`
- Test: `apps/android/app/src/androidTest/**`

**Interfaces:**
- Consumes: UniFFI bindings and direct TCP ADB core.
- Produces: Android APK with local-network permission, secure ADB key storage, Android document-provider backup import/export, and the seven-screen workflow.

- [ ] **Step 1: Write failing Android unit/UI tests**

Test IP input, RSA approval instructions, lifecycle reconnect, compatible launcher selection, backup
location selection, operation confirmation, background interruption, and no mutation before baseline
backup.

- [ ] **Step 2: Verify tests fail**

Run: `./gradlew -p apps/android testDebugUnitTest`
Expected: FAIL because the Android application does not exist.

- [ ] **Step 3: Generate UniFFI bindings and implement Compose UI**

Store the ADB private key in Android Keystore-backed storage and keep operations foreground-only with explicit progress/cancellation.

- [ ] **Step 4: Verify unit, instrumentation, and package builds**

Run: `./gradlew -p apps/android testDebugUnitTest connectedDebugAndroidTest assembleDebug`
Expected: tests pass and a sideloadable debug APK is produced.

- [ ] **Step 5: Commit**

```bash
git add bindings apps/android scripts/package-android.sh Cargo.toml
git commit -m "feat: add Android sideload companion"
```

### Task 11: Validate the current Cube and create the first profile evidence

**Files:**
- Create: `profiles/evidence/raven/README.md`
- Create: `profiles/evidence/raven/redacted-*.json`
- Create: `tests/fixtures/raven-*.json`
- Modify: `profiles/experimental/raven.json`
- Test: `tests/real-device/raven.sh`

**Interfaces:**
- Consumes: desktop/CLI packages, current `raven` Cube, approved validation procedure.
- Produces: redacted real-device evidence and an experimental profile; status remains experimental until the multi-device threshold is met.

- [ ] **Step 1: Capture a fresh baseline and run read-only checks**

Run: `firetvctl inspect`, `firetvctl backup --full`, and `firetvctl health` against the manually entered Cube IP.
Expected: identity, backup integrity, remote, settings, audio/HDMI, network, and critical services pass.

- [ ] **Step 2: Run base-safe install and verification**

Use the user-selected Projectivy APK; install/repair open-source helpers; verify Home routing, full settings, Bluetooth, Appstore installation, login persistence, and representative playback.

- [ ] **Step 3: Run uninstall and exact restore tests**

Expected: Amazon Home returns, custom accessibility entries are removed, and package/settings comparison has no unexplained delta.

- [ ] **Step 4: Run failure injection through mock fixtures**

Replay authorization rejection, disconnect, corrupt APK, low storage, and failed verification using the captured redacted inventory.

- [ ] **Step 5: Commit redacted evidence only**

```bash
git add profiles/evidence/raven profiles/experimental/raven.json tests/fixtures tests/real-device
git commit -m "test: add redacted raven validation evidence"
```

### Task 12: Write detailed documentation and demo

**Files:**
- Modify: `README.md`
- Create: `docs/INSTALL.md`
- Create: `docs/UNINSTALL.md`
- Create: `docs/BACKUP-RESTORE.md`
- Create: `docs/PROFILE-VALIDATION.md`
- Create: `docs/TROUBLESHOOTING.md`
- Create: `docs/PRIVACY.md`
- Create: `docs/DEMO.md`
- Create: `docs/assets/demo/**`
- Create: `scripts/demo.sh`
- Test: `scripts/check-docs.sh`

**Interfaces:**
- Consumes: shipped commands, UI labels, real-device evidence.
- Produces: complete public README, deterministic terminal demo, screenshots/GIF recorded from redacted fixtures, installation/uninstallation/restore instructions, feature matrix, limitations, and contributor profile-validation guide.

- [ ] **Step 1: Write failing documentation checks**

Assert README links resolve and contains Quick Start, Enable ADB, Compatibility, Install, Demo, Features, Backup, Restore, Uninstall, Updates, Package Profiles, Limitations, Security, Privacy, and Troubleshooting.

- [ ] **Step 2: Verify documentation checks fail**

Run: `bash scripts/check-docs.sh`
Expected: FAIL with missing sections/files.

- [ ] **Step 3: Write documentation and fixture-driven demo**

Include exact macOS/Windows/Linux/Android commands and UI steps, screenshots without device identifiers, and explicit Projectivy acquisition/license guidance.

- [ ] **Step 4: Verify documentation and demo**

Run: `bash scripts/check-docs.sh && bash scripts/demo.sh --fixture tests/fixtures/raven-inspection.json`
Expected: PASS and a deterministic demo transcript/assets directory.

- [ ] **Step 5: Commit**

```bash
git add README.md docs scripts/check-docs.sh scripts/demo.sh
git commit -m "docs: add installation restoration and demo guide"
```

### Task 13: Add reproducible CI, release packaging, and GitHub repository

**Files:**
- Create: `.github/workflows/ci.yml`
- Create: `.github/workflows/release.yml`
- Create: `.github/ISSUE_TEMPLATE/profile-validation.yml`
- Create: `.github/pull_request_template.md`
- Create: `scripts/checksums.sh`
- Create: `scripts/release.sh`

**Interfaces:**
- Consumes: all packages and tests.
- Produces: CI for Rust, desktop, Android, schema/docs/policy checks; signed checksums; private GitHub repository initially; tagged test releases without proprietary APKs.

- [ ] **Step 1: Write failing release-policy test**

Extend `scripts/check-repository.sh` to inspect release staging and reject APK payloads other than this project's Android companion, backups, keys, device identifiers, and unsigned profile manifests.

- [ ] **Step 2: Configure CI matrices and release staging**

Build/test Rust on macOS, Windows, and Ubuntu; build desktop packages per host; build Android debug APK; upload checksums and SBOMs; keep signing secrets external.

- [ ] **Step 3: Run the full local gate**

Run: `cargo fmt --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && bash scripts/check-repository.sh && bash scripts/check-docs.sh`
Expected: PASS.

- [ ] **Step 4: Initialize and publish Git history**

Create a private GitHub repository named `universal-firetv-manager`, push the default branch, verify Actions, repository description, topics, README rendering, and license detection. Make it public only after checking third-party notices and the user's explicit visibility approval.

- [ ] **Step 5: Create the first test release**

Publish desktop and Android test artifacts, checksums, SBOMs, known limitations, and restore instructions. Do not attach Projectivy or Google Platform Tools.

- [ ] **Step 6: Commit**

```bash
git add .github scripts/checksums.sh scripts/release.sh
git commit -m "ci: add cross-platform validation and test releases"
```

### Task 14: Build the iPhone/iPad test package last

**Files:**
- Create: `apps/ios/**`
- Create: `scripts/package-ios.sh`
- Test: `apps/ios/FireTVManagerTests/**`
- Test: `apps/ios/FireTVManagerUITests/**`
- Modify: `README.md`
- Modify: `.github/workflows/ci.yml`

**Interfaces:**
- Consumes: stable UniFFI bindings and all core workflows proven by desktop/Android.
- Produces: SwiftUI iPhone/iPad application with Local Network permission, Keychain ADB key storage, document-based backup export/import, and ad-hoc/TestFlight package.

- [ ] **Step 1: Write failing Swift and UI tests**

Test `NSLocalNetworkUsageDescription`, permission denial/retry, IP connection, Keychain persistence, background cancellation, file export/import, and all safe workflow screens.

- [ ] **Step 2: Verify tests fail**

Run: `xcodebuild test -project apps/ios/FireTVManager.xcodeproj -scheme FireTVManager -destination 'platform=iOS Simulator,name=iPhone 17'`
Expected: FAIL because the iOS project does not exist.

- [ ] **Step 3: Generate Swift bindings and implement SwiftUI application**

Attempt local TCP only while foregrounded; present the Apple Local Network prompt before ADB connection; cancel cleanly when the app backgrounds.

- [ ] **Step 4: Verify simulator and physical-device tests**

Expected: simulator UI/backup tests pass; a signed physical iPhone connects to the current Cube, backs up, runs base-safe health checks, and performs install/uninstall/restore without unexplained deltas.

- [ ] **Step 5: Produce ad-hoc/TestFlight archive**

Run: `bash scripts/package-ios.sh`
Expected: signed `.xcarchive`/`.ipa` when signing credentials are available, otherwise a reproducible unsigned simulator build plus documented signing steps.

- [ ] **Step 6: Commit and publish iOS test release**

```bash
git add apps/ios scripts/package-ios.sh README.md .github/workflows/ci.yml
git commit -m "feat: add iPhone and iPad companion"
```

