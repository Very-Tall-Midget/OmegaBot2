use winapi::um::winnt::LPCSTR;

mod play_layer;
pub use play_layer::PlayLayer;
mod game_manager;
pub use game_manager::GameManager;
mod player_object;
pub use player_object::PlayerObject;
pub use player_object::GameMode;
mod level_settings;
pub use level_settings::LevelSettings;
mod game_object;
pub use game_object::GameObject;

pub fn get_base() -> usize {
    unsafe { winapi::um::libloaderapi::GetModuleHandleA(0 as LPCSTR) as usize }
}
