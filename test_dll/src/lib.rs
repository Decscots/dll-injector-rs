#![crate_type = "cdylib"]

use windows::{
    core::*,
    Win32::UI::WindowsAndMessaging::*,
    Win32::Foundation::HINSTANCE,
};
use std::ffi::c_void;
use windows::core::imp::CloseHandle;
use windows::Win32::System::LibraryLoader::FreeLibraryAndExitThread;
use windows::Win32::System::Threading::{CreateThread, THREAD_CREATION_FLAGS};

fn to_pcstr(string: String) -> PCSTR {
    let mut string_clone = string.clone();
    string_clone.push('\0');
    PCSTR::from_raw(string_clone.as_ptr())
}

unsafe extern "system" fn test_thread(instance: *mut c_void) -> u32 {
    MessageBoxA(None, s!("Hello from a thread!"), s!("Hello again!!"), MB_OK);
    FreeLibraryAndExitThread(std::mem::transmute::<*mut c_void, HINSTANCE>(instance), 1);
}

#[no_mangle]
pub extern "system" fn DllMain(inst: HINSTANCE, reason: u32, _: *mut usize) -> i32 {
    match reason {
        1 /* DLL_PROCESS_ATTACH */ => unsafe {
            MessageBoxA(None, s!("Hello from DLL_PROCESS_ATTACH!"), s!("Hello!!"), MB_OK);
            match CreateThread(None, 0, Some(test_thread), Some(std::mem::transmute(inst)), THREAD_CREATION_FLAGS(0), None) {
                Ok(handle) => { CloseHandle(handle.0); },
                Err(error) => {
                    MessageBoxA(None, to_pcstr(format!("An error occurred while calling CreateThread: {}", error)), s!("Error"), MB_OK);
                }
            }
        }
        default => unsafe {
            let text = to_pcstr(format!("DllMain called with reason: {}", default));
            MessageBoxA(None, text, text, MB_OK);
        }
    }
    1
}