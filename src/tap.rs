//! CGEvent tap: suppresses the Caps Lock release so the OS toggle never fires.

use core_foundation::runloop::{kCFRunLoopCommonModes, CFRunLoop};
use core_graphics::event::{
    CGEvent, CGEventFlags, CGEventTap, CGEventTapLocation, CGEventTapOptions,
    CGEventTapPlacement, CGEventTapProxy, CGEventType, EventField,
};

const CAPS_LOCK_KEYCODE: i64 = 57;

fn suppress_caps_lock_release(_proxy: CGEventTapProxy, event_type: CGEventType, event: &CGEvent) -> Option<CGEvent> {
    match event_type {
        CGEventType::FlagsChanged => {}
        _ => return Some(event.clone()),
    }
    if event.get_integer_value_field(EventField::KEYBOARD_EVENT_KEYCODE) != CAPS_LOCK_KEYCODE {
        return Some(event.clone());
    }

    // Allow press through (activates .keylayout layer), suppress release (IOKit handles state reset)
    if event.get_flags().contains(CGEventFlags::CGEventFlagAlphaShift) {
        Some(event.clone())
    } else {
        None
    }
}

pub fn install_caps_lock_tap() -> CGEventTap<'static> {
    let tap = CGEventTap::new(
        CGEventTapLocation::HID,
        CGEventTapPlacement::HeadInsertEventTap,
        CGEventTapOptions::Default,
        vec![CGEventType::FlagsChanged],
        suppress_caps_lock_release,
    )
    .expect(
        "Failed to create CGEvent tap — grant Accessibility access:\n\
         System Settings → Privacy & Security → Accessibility",
    );

    let source = tap.mach_port.create_runloop_source(0).unwrap();
    CFRunLoop::get_current().add_source(&source, unsafe { kCFRunLoopCommonModes });
    tap.enable();

    tap
}
