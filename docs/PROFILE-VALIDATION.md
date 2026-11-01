# Package Profile Validation

1. Capture read-only identity and inventory from a fully working stock device.
2. Remove serials, IP/MAC addresses, accounts, tokens, and personal paths.
3. Record exact fingerprint, Fire OS, SDK, and ABIs.
4. Prove every proposed package is noncritical and has a tested inverse.
5. Test install, remote/Home, full Settings, Appstore, accounts, keyboard, Bluetooth, HDMI inputs, volume/audio, playback, OTA visibility, uninstall, and restore.
6. Repeat on at least two devices of the exact build before `validated` status.
7. Test authorization denial, disconnect, corrupt APK, low storage, and verification failure.
8. Submit only redacted evidence and signed profile metadata.

Any fingerprint change returns the device to `base-safe` until validation repeats.
