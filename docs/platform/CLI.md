# Command-line guide

Run `firetvctl --help` for the complete command list. A safe first session is:

```text
firetvctl connect FIRE_TV_IP
firetvctl inspect FIRE_TV_IP --json
firetvctl profiles FIRE_TV_IP --json
firetvctl backup FIRE_TV_IP --output BASELINE_DIRECTORY --full
firetvctl health FIRE_TV_IP --json
```

Every mutation requires a verified device-bound backup plus the exact plan hash returned by `firetvctl plan`. Do not reuse a plan hash after an OS update, IP change, payload change, or backup change. `base-safe` deliberately refuses package-specific repair/uninstall operations without a validated profile.
