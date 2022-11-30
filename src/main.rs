#![no_main]
#![no_std]
//[package]
// name = "spinlockp"
// version = "0.1.0"
// edition = "2021"
//
// # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
//
// [dependencies]
//
// [dependencies.windows-sys]
// version = "0.42.0"
// features = [
//     "Win32_Foundation",
//     "Win32_UI_WindowsAndMessaging"
// ]
//
// [profile.dev]
// panic = "abort"
//
// [profile.release]
// panic = "abort"
// opt-level = "z"
// lto = true
// strip = true
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