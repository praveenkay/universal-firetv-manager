# Launch and Promotion Kit

This kit keeps outreach accurate, transparent, and useful. Say that you are the maintainer when posting. Do not ask for coordinated upvotes, mass-post identical copy, or imply support from Amazon.

Primary link: <https://github.com/praveenkay/universal-firetv-manager>

Release link: <https://github.com/praveenkay/universal-firetv-manager/releases/latest>

Social card: [`assets/universal-firetv-manager-social-card.png`](assets/universal-firetv-manager-social-card.png)

## X / Twitter

### Single post

I built Universal Fire TV Manager because most Fire TV cleanup scripts are easy to apply and hard to undo.

It connects over Wi-Fi ADB, checks compatibility, requires a backup before changes, protects critical Fire OS packages, and supports restore/uninstall.

macOS, Windows, Linux, Android + CLI:
https://github.com/praveenkay/universal-firetv-manager

Looking for Fire TV Stick/Cube testers across models and Fire OS versions.

### Thread

1. I built an open-source, backup-first manager for Fire TV Stick and Cube devices. The goal: make launcher changes and conservative debloating safer to test and practical to undo.

2. Before any mutation it inspects the model/build, checks SDK and ABI compatibility, and creates a device-bound baseline. Unknown builds fall back to a conservative `base-safe` profile.

3. It protects Settings, accounts, DRM, Appstore, keyboard, Bluetooth, HDMI/TV inputs, audio, and OTA components. Restore verifies the backup and exact device identity before applying anything.

4. Packages are available for macOS, Windows, Linux, Android, and a portable CLI. It supports Projectivy as a user-supplied option and includes open-source launcher choices.

5. This is not a custom ROM, and retail ADB cannot back up private app logins or DRM data. I would value test reports from more Fire TV models and builds: https://github.com/praveenkay/universal-firetv-manager

Suggested tags: `#FireTV #OpenSource #AndroidTV #ADB`

## Reddit

Suggested communities: `r/firetvstick` if its current rules permit a standalone open-source tool post, and the current `r/GoogleTV` self-promotion megathread. Read each community's rules immediately before posting.

Title: **I built an open-source, backup-first Fire TV debloat and launcher manager**

Body:

> Maintainer here. I built Universal Fire TV Manager after running into the usual Fire TV problem: cleanup scripts are easy to apply and hard to undo safely.
>
> It connects over Wi-Fi ADB, checks the device and build first, creates a required baseline backup, protects critical Settings/Appstore/Bluetooth/HDMI/audio/OTA packages, and supports restore and uninstall. It offers Projectivy as a user-supplied option plus open-source launcher choices.
>
> Packages are available for Windows, macOS, Linux, Android, and CLI. It does not flash a custom ROM, and retail ADB cannot preserve private app logins or DRM data.
>
> Repo and downloads: https://github.com/praveenkay/universal-firetv-manager
>
> I have validated the current profile on a Fire TV Cube `raven` running Fire OS 7. I would appreciate reports from other Stick/Cube models, especially redacted fingerprint/profile output and any restore edge cases.

## Hacker News

Title: **Show HN: Backup-first Fire TV debloat and launcher manager**

Submission URL: <https://github.com/praveenkay/universal-firetv-manager>

First comment:

> I built this after discovering how many Fire TV cleanup scripts assume every model and Fire OS build exposes the same packages. A package that looks optional on one device can carry Settings, inputs, or audio on another.
>
> Universal Fire TV Manager connects over direct TCP ADB, inspects the device, requires a hash-verified baseline before mutations, and falls back to a conservative profile for unknown builds. Plans have confirmation hashes, changes are journaled, and restore is bound to the original device identity.
>
> There are desktop packages for macOS, Windows, and Linux, Android APKs, and portable CLI builds. The current real-device validation is a Fire TV Cube `raven` on Fire OS 7, so reports from other hardware are particularly useful.
>
> It deliberately does not claim to be a custom OS: retail ADB cannot replace signed Fire OS firmware or preserve private app data. I would welcome feedback on the safety model, packaging, and profile contribution flow.

Follow the official Show HN rules: the maintainer should be available to answer questions, and nobody should be asked to upvote or coordinate comments.

## LinkedIn

I have released Universal Fire TV Manager, an open-source tool for safer Fire TV launcher changes and conservative debloating.

The design starts with recovery: it inspects compatibility, requires a device-bound baseline backup before any changes, protects critical Fire OS components, journals operations, and supports restore and uninstall. Unknown builds fall back to a conservative profile instead of guessing.

The first public release includes packages for macOS, Windows, Linux, Android, and a portable CLI. It supports user-supplied Projectivy and open-source launcher alternatives.

I am looking for testers across Fire TV Stick and Cube models, as well as contributors who can provide redacted device evidence for new signed profiles.

Project and downloads: https://github.com/praveenkay/universal-firetv-manager

#OpenSource #FireTV #AndroidTV #Rust #Tauri

## Product Hunt

Name: **Universal Fire TV Manager**

Tagline: **Backup-first Fire TV cleanup, launcher setup, and restore**

Description:

Universal Fire TV Manager helps owners inspect, back up, conservatively debloat, customize, health-check, uninstall, and restore compatible Fire TV Stick and Cube devices over Wi-Fi ADB. It ships for macOS, Windows, Linux, Android, and CLI, with guarded profiles that protect critical Fire OS services.

Maker comment:

I built this because Fire TV customization often begins with a shell script and ends without a reliable path back. The manager requires a matching baseline before changes, validates device/build compatibility, and makes limitations explicit. It is open source, local-first, and does not scan your network or upload device data. I am launching early to collect real-device reports across the fragmented Fire TV hardware lineup.

## YouTube Demo

Title: **Debloat a Fire TV Safely—and Restore It | Universal Fire TV Manager**

Thumbnail text: **BACKUP FIRST. DEBLOAT SAFELY.**

Description:

Universal Fire TV Manager is an open-source, backup-first manager for compatible Fire TV Sticks and Cubes. This demo covers ADB setup, compatibility inspection, baseline backup, launcher selection, a guarded install plan, health checks, uninstall, and restore.

Download: https://github.com/praveenkay/universal-firetv-manager/releases/latest

Source: https://github.com/praveenkay/universal-firetv-manager

Important: this is not a replacement Fire OS ROM. Model/build behavior varies, and retail ADB cannot back up private app logins or DRM data.

Chapter outline:

- 00:00 What the manager does
- 00:35 Enable ADB debugging and find the IP
- 01:20 Connect and inspect compatibility
- 02:10 Create the mandatory baseline
- 03:00 Select a launcher and review the plan
- 04:10 Apply changes and run health checks
- 05:20 Uninstall and restore
- 06:20 Limitations and contributing a device profile

## Outreach Targets

- GitHub Discussions: invite test reports and profile contributions.
- Relevant launcher projects: open a technical discussion only when there is a concrete integration or compatibility question; do not drop promotional links into unrelated issues.
- Fire TV and Android TV communities: tailor the post to each community's rules and answer support questions after posting.
- Release notes and future changelogs: include screenshots, supported device evidence, known limitations, and direct download links.

## Launch Checklist

- Verify every public binary and checksum.
- Keep the release marked stable only if its documented smoke tests passed.
- Publish the GitHub announcement and open a feedback discussion.
- Post to one community at a time and remain available for replies.
- Record post URLs and outcomes below; use them to improve later posts rather than repeating weak copy.

| Channel | URL | Date | Notes |
|---|---|---|---|
| GitHub | https://github.com/praveenkay/universal-firetv-manager/discussions/1 | 2026-11-01 | Launch announcement and tester call |
| X |  |  |  |
| Reddit |  |  |  |
| Hacker News |  |  |  |
| LinkedIn |  |  |  |
| Product Hunt |  |  |  |
