//! capslock-untoggle — makes Caps Lock behave as a held modifier on macOS.
//!
//! Requires Accessibility access: System Settings → Privacy & Security → Accessibility

mod hid;
mod tap;

use core_foundation::runloop::{kCFRunLoopCommonModes, CFRunLoop};
use std::ffi::c_void;

fn main() {
    unsafe {
        let run_loop_mode = kCFRunLoopCommonModes as *const _ as *const c_void;
        hid::start_keyboard_monitor(hid::CFRunLoopGetCurrent(), run_loop_mode);
    }

    let _caps_lock_tap = tap::install_caps_lock_tap();

    CFRunLoop::run_current();
}
