use winapi::shared::minwindef::FARPROC;

use super::*;
use crate::utils::*;

pub struct SharedDirector {
    address: usize,
}

impl SharedDirector {
    pub fn get() -> Self {
        Self {
            address: unsafe {
                (std::mem::transmute::<FARPROC, unsafe extern "cdecl" fn() -> usize>(
                    winapi::um::libloaderapi::GetProcAddress(
                        get_hmod(),
                        lpcstr!("?sharedDirector@CCDirector@cocos2d@@SAPAV12@XZ"),
                    ),
                ))()
            }
        }
    }

    pub fn get_scheduler(&self) -> usize {
        unsafe { *read_value(self.address + 0x48) }
    }
}

impl_is_null!(SharedDirector);
