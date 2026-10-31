# Security policy

## Reporting

Do not open a public issue for a vulnerability involving ADB keys, backup contents, device identifiers, signature verification, or rollback. Use GitHub private vulnerability reporting after the repository is published.

## Supported releases

Only the latest tagged test release receives security fixes during initial development.

## Safety boundaries

This project never requests root, unlocks a bootloader, flashes firmware, clears app data, or uploads backups. Unknown Fire TV builds receive the conservative `base-safe` profile only.
