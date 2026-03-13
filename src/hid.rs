//! IOKit HID layer: detects physical Caps Lock press/release and turns it off on release.

use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, Ordering};

static CAPS_LOCK_HELD: AtomicBool = AtomicBool::new(false);
static mut HID_SYSTEM_CLIENT: *mut c_void = std::ptr::null_mut();

type IOHIDManagerRef = *mut c_void;
type IOHIDValueRef   = *mut c_void;
type IOHIDElementRef = *mut c_void;
type IOReturn        = i32;

const USAGE_PAGE_KEYBOARD: u32     = 0x07;
const USAGE_CAPS_LOCK: u32         = 0x39;
const CF_STRING_ENCODING_UTF8: u32 = 0x0800_0100;
const CF_NUMBER_SINT32: i64        = 3;

#[link(name = "IOKit", kind = "framework")]
extern "C" {
    fn IOHIDManagerCreate(allocator: *mut c_void, options: u32) -> IOHIDManagerRef;
    fn IOHIDManagerSetDeviceMatching(manager: IOHIDManagerRef, matching: *mut c_void);
    fn IOHIDManagerRegisterInputValueCallback(
        manager: IOHIDManagerRef,
        callback: extern "C" fn(*mut c_void, IOReturn, *mut c_void, IOHIDValueRef),
        context: *mut c_void,
    );
    fn IOHIDManagerScheduleWithRunLoop(manager: IOHIDManagerRef, run_loop: *mut c_void, mode: *const c_void);
    fn IOHIDManagerOpen(manager: IOHIDManagerRef, options: u32) -> IOReturn;
    fn IOHIDValueGetElement(value: IOHIDValueRef) -> IOHIDElementRef;
    fn IOHIDValueGetIntegerValue(value: IOHIDValueRef) -> i64;
    fn IOHIDElementGetUsage(element: IOHIDElementRef) -> u32;
    fn IOHIDElementGetUsagePage(element: IOHIDElementRef) -> u32;
    fn IOHIDEventSystemClientCreateSimpleClient(allocator: *mut c_void) -> *mut c_void;
    fn IOHIDEventSystemClientSetProperty(client: *mut c_void, key: *mut c_void, property: *mut c_void) -> bool;
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn CFDictionaryCreateMutable(alloc: *mut c_void, cap: isize, kcb: *const c_void, vcb: *const c_void) -> *mut c_void;
    fn CFDictionarySetValue(dict: *mut c_void, key: *const c_void, val: *const c_void);
    fn CFNumberCreate(alloc: *mut c_void, t: i64, val: *const c_void) -> *mut c_void;
    fn CFStringCreateWithCString(alloc: *mut c_void, s: *const u8, enc: u32) -> *mut c_void;
    pub fn CFRunLoopGetCurrent() -> *mut c_void;
    static kCFTypeDictionaryKeyCallBacks: c_void;
    static kCFTypeDictionaryValueCallBacks: c_void;
    static kCFBooleanFalse: *mut c_void;
}

unsafe fn cf_str(s: &[u8]) -> *mut c_void {
    CFStringCreateWithCString(std::ptr::null_mut(), s.as_ptr(), CF_STRING_ENCODING_UTF8)
}

unsafe fn cf_num(n: i32) -> *mut c_void {
    CFNumberCreate(std::ptr::null_mut(), CF_NUMBER_SINT32, &n as *const _ as *const c_void)
}

unsafe fn turn_caps_lock_off() {
    IOHIDEventSystemClientSetProperty(
        HID_SYSTEM_CLIENT,
        cf_str(b"HIDCapsLockState\0"),
        kCFBooleanFalse,
    );
}

extern "C" fn on_caps_lock_key_event(_ctx: *mut c_void, _result: IOReturn, _sender: *mut c_void, value: IOHIDValueRef) {
    unsafe {
        let element = IOHIDValueGetElement(value);
        if IOHIDElementGetUsagePage(element) != USAGE_PAGE_KEYBOARD
            || IOHIDElementGetUsage(element) != USAGE_CAPS_LOCK
        {
            return;
        }

        let pressed  = IOHIDValueGetIntegerValue(value) != 0;
        let was_held = CAPS_LOCK_HELD.load(Ordering::Relaxed);

        if pressed && !was_held {
            CAPS_LOCK_HELD.store(true, Ordering::Relaxed);
        } else if !pressed && was_held {
            CAPS_LOCK_HELD.store(false, Ordering::Relaxed);
            turn_caps_lock_off();
        }
    }
}

unsafe fn keyboard_device_filter() -> *mut c_void {
    let filter = CFDictionaryCreateMutable(
        std::ptr::null_mut(),
        0,
        &kCFTypeDictionaryKeyCallBacks as *const _ as *const c_void,
        &kCFTypeDictionaryValueCallBacks as *const _ as *const c_void,
    );
    CFDictionarySetValue(filter, cf_str(b"DeviceUsagePage\0"), cf_num(1));
    CFDictionarySetValue(filter, cf_str(b"DeviceUsage\0"),     cf_num(6));
    filter
}

pub unsafe fn start_keyboard_monitor(run_loop: *mut c_void, run_loop_mode: *const c_void) {
    HID_SYSTEM_CLIENT = IOHIDEventSystemClientCreateSimpleClient(std::ptr::null_mut());
    assert!(!HID_SYSTEM_CLIENT.is_null(), "Failed to create IOHIDEventSystemClient");

    let keyboard_monitor = IOHIDManagerCreate(std::ptr::null_mut(), 0);
    IOHIDManagerSetDeviceMatching(keyboard_monitor, keyboard_device_filter());
    IOHIDManagerRegisterInputValueCallback(keyboard_monitor, on_caps_lock_key_event, std::ptr::null_mut());
    IOHIDManagerScheduleWithRunLoop(keyboard_monitor, run_loop, run_loop_mode);
    IOHIDManagerOpen(keyboard_monitor, 0);
}
