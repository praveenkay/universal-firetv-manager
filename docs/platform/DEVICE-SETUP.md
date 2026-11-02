# Fire TV device setup

1. On the Fire TV open **Settings → My Fire TV → About**. Highlight the device name and press Select seven times if Developer Options is hidden.
2. Open **Developer Options** and enable **ADB Debugging**.
3. Find the numeric address under **About → Network**. Keep the controller and Fire TV on the same trusted network.
4. Open Universal Fire TV Manager, enter the address, connect, and accept the RSA prompt on the television.
5. Run **Inspect**. Unknown builds use `base-safe`; never force a model-specific profile onto another fingerprint.
6. Create and verify a baseline backup before applying a launcher or package plan.

The manager does not scan the LAN. Retail Fire OS cannot expose private app data, login tokens, DRM keys, or a flashable system image over ADB.
