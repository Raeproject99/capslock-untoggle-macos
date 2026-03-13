# capslock-untoggle

Makes Caps Lock behave as a **held modifier** instead of a toggle on macOS.
While Caps Lock is physically held, its modifier layer in your `.keylayout` is active.
The moment you release it, the caps lock state is reset — no LED, no stuck caps.

Tiny footprint: ~600KB binary, ~3MB RAM, ~0% CPU at idle.

## Install
```bash
curl -sSL https://raw.githubusercontent.com/Raeproject99/CapsLock-as-a-modifier/main/install.sh | bash
```

The installer will:
1. Download and sign the universal binary (Apple Silicon + Intel)
2. Install the LaunchAgent for auto-start at login
3. Open System Settings to the Accessibility page
4. Prompt you to grant access, then start the daemon

## How it works

Two components run together on the main run loop:

1. **IOKit HID manager** — watches raw keyboard hardware events to detect the exact
   moment Caps Lock is physically pressed and released.
2. **CGEvent tap** — sits at the HID level and suppresses the natural `FlagsChanged`
   release event, preventing macOS from running its own toggle logic.

On release, `IOHIDEventSystemClientSetProperty("HIDCapsLockState", false)` resets
the caps lock state directly in the HID event system.

## Requirements

- macOS 12+
- Accessibility access (the installer handles this)

## Uninstall
```bash
launchctl bootout gui/$(id -u)/com.local.capslock-untoggle
rm ~/Library/LaunchAgents/com.local.capslock-untoggle.plist
rm ~/.local/bin/capslock-untoggle
```

## Build from source
```bash
git clone https://github.com/Raeproject99/CapsLock-as-a-modifier
cd CapsLock-as-a-modifier
cargo build --release
```
