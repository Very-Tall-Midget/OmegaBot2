use super::*;
use crate::utils::*;
use std::ffi::CString;

#[derive(Debug, Clone, Copy)]
pub struct GameManager {
    address: usize,
}

impl GameManager {
    pub fn get() -> Self {
        GameManager {
            address: unsafe {
                (std::mem::transmute::<usize, unsafe extern "stdcall" fn() -> usize>(
                    get_base() + 0xC4A50,
                ))()
            },
        }
    }

    pub fn play_layer(&self) -> PlayLayer {
        PlayLayer::from_address(unsafe { read_ptr(self.address + 0x164) })
    }

    pub fn get_game_variable(&self, key: &str) -> bool {
        let key = CString::new(key).unwrap();
        unsafe {
            (std::mem::transmute::<
                usize,
                unsafe extern "fastcall" fn(usize, usize, *const u8) -> bool,
            >(get_base() + 0xC9D30))(self.address, 0, key.as_ptr() as *const u8)
        }
    }
}

impl_is_null!(GameManager);
