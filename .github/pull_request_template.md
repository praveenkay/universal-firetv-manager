## Summary

## Safety impact

- [ ] No protected package or service is weakened.
- [ ] Mutations retain backup, confirmation, verification, and rollback.
- [ ] Fixtures/evidence are redacted.
- [ ] No proprietary APK, backup, installer, device ID, or ADB key is included.

## Validation

- [ ] `cargo test --workspace`
- [ ] `npm --prefix apps/desktop test`
- [ ] `bash scripts/check-repository.sh`
- [ ] `bash scripts/check-docs.sh`
