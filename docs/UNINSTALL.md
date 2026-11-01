# Uninstall

1. Locate the last verified baseline for the same physical device and build.
2. Run `firetvctl plan IP uninstall --backup PATH`.
3. Review the plan and protected-package list.
4. Use the guided uninstall or confirmed operation with `--yes --plan-hash HASH`.
5. Verify Amazon Home, Settings, Appstore, Bluetooth, HDMI inputs, keyboard, audio, and playback.
6. Remove the manager app only after the Fire TV is healthy.

Never factory reset merely to remove this tool. A custom launcher may remain installed as a normal app.
