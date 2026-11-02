# macOS guide

Download the DMG matching the Mac architecture. Open it and drag **Universal Fire TV Manager** to Applications. The current community build is not Apple-notarized; Control-click the app and choose **Open** the first time.

Connect using the Fire TV's numeric IP, accept its RSA prompt, inspect, and create a backup. The portable `firetvctl-macos-arm64` binary is available for Terminal users; run `chmod +x firetvctl-macos-arm64` if the executable bit was removed during download.

Verify downloads with `shasum -a 256 -c SHA256SUMS` from the release directory.
