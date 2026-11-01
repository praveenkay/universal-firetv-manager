# Backup and Restore

`firetvctl backup IP --output DIRECTORY --full` writes atomically and renames the backup only after capture succeeds. The manifest binds it to the exact device identity and hashes every listed file.

Restore rejects modified files or a different device. Missing original settings are deleted rather than saved as empty strings.

Non-root retail ADB cannot read third-party private app data, so logins, YouTube sessions, DRM licenses, and private settings cannot be backed up. It also cannot create a flashable stock firmware image.

Generate a restore hash with `firetvctl plan IP restore --backup PATH`, then run `firetvctl restore IP --backup PATH --yes --plan-hash HASH` and `firetvctl health IP`.
