#![no_main]
#![no_std]

mod alloc;

use core::ptr::null;
use windows_sys::core::PCSTR;
use windows_sys::Win32::Foundation::HWND;
use windows_sys::Win32::UI::WindowsAndMessaging::MessageBoxA;

macro_rules! cstr {
    ($str:expr) => (concat!($str, "\0").as_bytes())
}

#[no_mangle]
pub extern "C" fn _start() -> i32 {
    let message = cstr!("Holy shit this is 3kb");
    let caption = cstr!("Holy Shit!");

    unsafe {
        MessageBoxA(HWND::default(), PCSTR::from(message.as_ptr()), PCSTR::from(caption.as_ptr()), 0);
    }

    0
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}