# capslock-untoggle

Makes Caps Lock behave as a **held modifier** instead of a toggle on macOS.

While Caps Lock is physically held, its modifier layer in your `.keylayout` is active.
The moment you release it, the caps lock state is reset — no LED, no stuck caps.

Tiny footprint: ~2MB binary, ~3MB RAM, ~0% CPU at idle.

## Requirements

- macOS 12+
- Rust toolchain (`brew install rust`)
- Accessibility access granted to the binary

## Install

```bash
cargo build --release
sudo cp target/release/capslock-untoggle /usr/local/bin/

# Grant Accessibility access when prompted, then set up auto-start:
cp com.local.capslock-untoggle.plist ~/Library/LaunchAgents/
launchctl load ~/Library/LaunchAgents/com.local.capslock-untoggle.plist
```

## How it works

Two components run together on the main run loop:

1. **IOKit HID manager** — watches raw keyboard hardware events to detect the exact
   moment Caps Lock is physically pressed and released.

2. **CGEvent tap** — sits at the HID level and suppresses the natural `FlagsChanged`
   release event, preventing macOS from running its own toggle logic.

On release, `IOHIDEventSystemClientSetProperty("HIDCapsLockState", false)` resets
the caps lock state directly in the HID event system.

## Permissions

The CGEvent tap requires Accessibility access. On first run macOS will prompt you,
or grant it manually in:

> System Settings → Privacy & Security → Accessibility

## Uninstall

```bash
launchctl unload ~/Library/LaunchAgents/com.local.capslock-untoggle.plist
rm ~/Library/LaunchAgents/com.local.capslock-untoggle.plist
sudo rm /usr/local/bin/capslock-untoggle
```
