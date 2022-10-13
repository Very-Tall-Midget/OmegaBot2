use super::*;
use winapi::shared::minwindef::FARPROC;

#[derive(Debug, Clone, Copy)]
pub struct CCApplication {
    address: usize,
}

impl CCApplication {
    pub fn shared_application() -> Self {
        Self {
            address: unsafe {
                (std::mem::transmute::<FARPROC, unsafe extern "cdecl" fn() -> usize>(
                    winapi::um::libloaderapi::GetProcAddress(
                        get_hmod(),
                        lpcstr!("?sharedApplication@CCApplication@cocos2d@@SAPAV12@XZ"),
                    ),
                ))()
            },
        }
    }

    pub fn set_animation_interval(&self, interval: f64) {
        unsafe {
            (std::mem::transmute::<FARPROC, unsafe extern "fastcall" fn(usize, usize, f64)>(
                winapi::um::libloaderapi::GetProcAddress(
                    get_hmod(),
                    lpcstr!("?setAnimationInterval@CCApplication@cocos2d@@UAEXN@Z"),
                ),
            ))(self.address, 0, interval)
        }
    }
}

impl_is_null!(CCApplication);
