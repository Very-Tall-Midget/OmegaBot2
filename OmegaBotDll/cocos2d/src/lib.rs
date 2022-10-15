#[macro_use]
#[allow(dead_code)]
mod utils;
pub use utils::IsNull;
mod ccapplication;
pub use ccapplication::CCApplication;
use winapi::shared::minwindef::HMODULE;
mod shared_director;
pub use shared_director::SharedDirector;

pub fn get_hmod() -> HMODULE {
    unsafe { winapi::um::libloaderapi::GetModuleHandleA(lpcstr!("libcocos2d.dll")) }
}
