use windows::{
    core::*,
    Win32::System::Diagnostics::ToolHelp::*,
    Win32::Foundation::CloseHandle,
    Win32::System::Threading::*,
    Win32::System::LibraryLoader::*,
    Win32::System::Memory::*,
    Win32::System::Diagnostics::Debug::*
};

use std::{
    result::Result,
    ffi::c_void,
};

fn to_pcstr(string: String) -> PCSTR {
    let mut string_clone = string.clone();
    string_clone.push('\0');
    PCSTR::from_raw(string_clone.as_ptr())
}

unsafe fn find_process_id(target_name: String) -> Result<u32, Box<dyn std::error::Error>> {
    let proc_entry_size = std::mem::size_of::<PROCESSENTRY32>() as u32;
    let mut entry = PROCESSENTRY32 { dwSize: proc_entry_size, ..Default::default() };

    let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)?;

    let mut target_process_id: Option<u32> = None;

    match Process32First(snapshot, &mut entry) {
        Ok(_) => {
            loop {
                let exe_file = &entry.szExeFile;
                let len = exe_file.iter().position(|&b| b == 0).unwrap_or(exe_file.len());
                let exe_file_str = String::from_utf8_lossy(&exe_file[..len]);
                if exe_file_str.to_lowercase() == target_name.to_lowercase() {
                    target_process_id = Some(entry.th32ProcessID);
                    break;
                }

                if Process32Next(snapshot, &mut entry).is_err() {
                    break;
                }
            }
        },
        Err(_) => return Err(From::from("Process32First failed")),
    }

    CloseHandle(snapshot)?;
    match target_process_id {
        Some(pid) => Ok(pid),
        None => Err(From::from("Process not open")),
    }
}

unsafe fn inject_dll(process_name: String, dll_path: String) -> Result<(), Box<dyn std::error::Error>> {
    let dll_path_size = dll_path.len() + 1;
    let dll_path_p = to_pcstr(dll_path);

    let pid = find_process_id(process_name)?;
    let handle = OpenProcess(PROCESS_ALL_ACCESS, false, pid)?;

    let dll_path_addr = VirtualAllocEx(handle, None, dll_path_size, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE);
    WriteProcessMemory(handle, dll_path_addr, dll_path_p.as_ptr() as *const c_void, dll_path_size, None)?;

    let kernel_dll = GetModuleHandleA(s!("kernel32.dll"))?;
    let load_library = GetProcAddress(kernel_dll, s!("LoadLibraryA"))
        .ok_or("Error while getting LoadLibraryA address")?;

    let thread_handle = CreateRemoteThread(handle, None, 0, Some(std::mem::transmute(load_library)), Some(dll_path_addr), 0, None)?;

    WaitForSingleObject(thread_handle, INFINITE);

    CloseHandle(thread_handle)?;
    CloseHandle(handle)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} target_program.exe dll_path", args[0]);
        return Ok(());
    }

    let process_name = args[1].clone();
    let dll_path = args[2].clone();
    unsafe { inject_dll(process_name, dll_path) }
}
