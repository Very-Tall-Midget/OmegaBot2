extern crate cocos2d;
extern crate macros;
extern crate replay;

use winapi::shared::minwindef::{BOOL, DWORD, FALSE, HMODULE, LPDWORD, LPVOID, TRUE};
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::minwinbase::LPSECURITY_ATTRIBUTES;
use winapi::um::winnt::DLL_PROCESS_ATTACH;

#[macro_use]
#[allow(dead_code)]
mod utils;
pub use gd;
mod hack;
mod hooks;
mod mutex_count;
mod omegabot;
mod patch;
mod pipe;
mod practice_fix;
mod replay_handler;
mod spam_bot;
#[macro_use]
mod hack_handler;

pub use omegabot::OMEGABOT;

unsafe extern "system" fn on_attach(hmod: LPVOID) -> DWORD {
    if OMEGABOT.initialise() {
        OMEGABOT.run();
        OMEGABOT.uninitialise();
    }

    winapi::um::libloaderapi::FreeLibraryAndExitThread(hmod as HMODULE, 0);
    0
}

#[no_mangle]
extern "system" fn DllMain(hmod: HMODULE, reason: DWORD, _: LPVOID) -> BOOL {
    if reason == DLL_PROCESS_ATTACH {
        unsafe {
            winapi::um::libloaderapi::DisableThreadLibraryCalls(hmod);
            let h = winapi::um::processthreadsapi::CreateThread(
                0 as LPSECURITY_ATTRIBUTES,
                0,
                Some(on_attach),
                hmod as LPVOID,
                0,
                0 as LPDWORD,
            );
            if h != INVALID_HANDLE_VALUE {
                winapi::um::handleapi::CloseHandle(h);
            } else {
                return FALSE;
            }
        }
    }
    TRUE
}
