use crate::gd;
use minhook_sys::*;
use std::ffi::CStr;

macro_rules! define_hook {
    ($hook_name:ident|$orig_name:ident $callconv:literal($($n:ident: $t:ty),+) $(-> $ret:ty)? $body:block) => {
        pub static mut $orig_name: *mut std::ffi::c_void = 0 as _;
        pub unsafe extern $callconv fn $hook_name($($n: $t),+) $(-> $ret)? {
            $body
        }
    }
}

macro_rules! create_hook (
    ($orig_name:ident -> $hook_name:ident @ $addr:literal) => {
        let status = MH_CreateHook(
            $addr as _,
            $hook_name as _,
            &mut $orig_name,
        );
        if status != MH_OK {
            panic!(
                "Failed to hook {}, error: {}",
                stringify!($hook_name),
                CStr::from_ptr(MH_StatusToString(status)).to_str().unwrap()
            );
        }
    };
    ($orig_name:ident -> $hook_name:ident @ gd + $addr:literal) => {
        let status = MH_CreateHook(
            (gd::get_base() + $addr) as _,
            $hook_name as _,
            &mut $orig_name,
        );
        if status != MH_OK {
            panic!(
                "Failed to hook {}, error: {}",
                stringify!($hook_name),
                CStr::from_ptr(MH_StatusToString(status)).to_str().unwrap()
            );
        }
    };
    ($orig_name:ident -> $hook_name:ident @ $lib:literal.$func:literal) => {
        let hmod = winapi::um::libloaderapi::GetModuleHandleA(lpcstr!($lib));
        assert!(hmod as usize != 0, "Failed to get module handle for {}", $lib);
        let status = MH_CreateHook(
            winapi::um::libloaderapi::GetProcAddress(hmod, lpcstr!($func)) as _,
            $hook_name as _,
            &mut $orig_name,
        );
        if status != MH_OK {
            panic!(
                "Failed to hook {}, error: {}",
                stringify!($hook_name),
                CStr::from_ptr(MH_StatusToString(status)).to_str().unwrap()
            );
        }
    }
);

macro_rules! remove_hook (
    ($addr:literal) => {
        MH_RemoveHook($addr as _);
    };
    (gd + $addr:literal) => {
        MH_RemoveHook((gd::get_base() + $addr) as _);
    };
    ($lib:literal.$func:literal) => {
        let hmod = winapi::um::libloaderapi::GetModuleHandleA(lpcstr!($lib));
        assert!(hmod as usize != 0, "Failed to get module handle for {}", $lib);
        MH_RemoveHook(winapi::um::libloaderapi::GetProcAddress(hmod, lpcstr!($func)) as _);
    }
);

pub mod cocos2d {
    use crate::{gd, utils::IsNull, OMEGABOT};

    define_hook! { schedule_update_h|SCHEDULE_UPDATE_O "fastcall"(scheduler: usize, _edx: usize, real_dt: f32) {
        if !OMEGABOT.wait_for_update() {
            return;
        }
        OMEGABOT.block_update();

        OMEGABOT.on_main_thread();

        if OMEGABOT.frame_advance() && gd::GameManager::get().if_not_null(
            |gm| gm.play_layer().if_not_null(
                |pl| !pl.is_dead())
                .unwrap_or(false))
            .unwrap_or(false) {
            OMEGABOT.unblock_update();
            return;
        }

        OMEGABOT.play_update(scheduler, real_dt);

        OMEGABOT.unblock_update();
    }}

    define_hook! { dispatch_keyboard_msg_h|DISPATCH_KEYBOARD_MSG_O "fastcall"(dispatcher: usize, _edx: usize, key_code: u32, press: bool) -> bool {
        let mut used = false;

        if let Some(c) = char::from_u32(key_code) {
            if press {
                if c == OMEGABOT.get_frame_advance_toggle_key() {
                    OMEGABOT.toggle_frame_advance();
                    used = true;
                } else if c == OMEGABOT.get_frame_advance_key() {
                    OMEGABOT.advance_frame();
                    used = true;
                } else if c == OMEGABOT.get_straight_fly_key() {
                    OMEGABOT.toggle_straight_fly();
                    used = true;
                } else if c == OMEGABOT.get_spam_key() {
                    OMEGABOT.toggle_spam();
                    used = true;
                }
            }
        }
        used || get_orig!(DISPATCH_KEYBOARD_MSG_O "fastcall"(usize, usize, u32, bool) -> bool)(dispatcher, 0, key_code, press)
    }}
}

pub mod play_layer {
    use crate::gd::PlayerObject;
    use crate::gd::{self, PlayLayer};
    use crate::OMEGABOT;
    use std::arch::asm;
    use std::ffi::c_void;

    pub static mut RENDER: bool = true;

    define_hook! { init_h|INIT_O "fastcall"(play_layer: usize, _edx: usize, level: usize) {
        get_orig!(INIT_O "fastcall"(usize, usize, usize) -> usize)(play_layer, 0, level);
        // idfk it works so i keep it
        asm!(
            "push eax",
            "push ebx",
            "push ecx",
            "push edx",
        );
        OMEGABOT.on_init(level);
        asm!(
            "pop edx",
            "pop ecx",
            "pop ebx",
            "pop eax",
        );
    }}

    define_hook! { update_h|UPDATE_O "fastcall"(play_layer: usize, _edx: usize, dt: f32) {
        OMEGABOT.on_update(gd::PlayLayer::from_address(play_layer));
        get_orig!(UPDATE_O "fastcall"(usize, usize, f32))(play_layer, 0, dt);
    }}

    define_hook! { reset_level_h|RESET_LEVEL_O "fastcall"(play_layer: usize) {
        get_orig!(RESET_LEVEL_O "fastcall"(usize))(play_layer);
        OMEGABOT.on_reset_level(gd::PlayLayer::from_address(play_layer));
    }}

    define_hook! { push_button_h|PUSH_BUTTON_O "fastcall"(play_layer: usize, _edx: usize, param: i32, button: bool) -> u32 {
        OMEGABOT.on_click(gd::PlayLayer::from_address(play_layer), true, button);
        get_orig!(PUSH_BUTTON_O "fastcall"(usize, usize, i32, bool) -> u32)(play_layer, 0, param, button)
    }}

    define_hook! { release_button_h|RELEASE_BUTTON_O "fastcall"(play_layer: usize, _edx: usize, param: i32, button: bool) -> u32 {
        OMEGABOT.on_click(gd::PlayLayer::from_address(play_layer), false, button);
        get_orig!(RELEASE_BUTTON_O "fastcall"(usize, usize, i32, bool) -> u32)(play_layer, 0, param, button)
    }}

    define_hook! { destroy_player_h|DESTROY_PLAYER_O "fastcall"(play_layer: usize, _edx: usize, player: usize, param: i32) {
        if OMEGABOT.allow_death(PlayLayer::from_address(play_layer), PlayerObject::from_address(player)) {
            get_orig!(DESTROY_PLAYER_O "fastcall"(usize, usize, usize, i32))(play_layer, 0, player, param);
        }
    }}

    define_hook! { on_quit_h|ON_QUIT_O "fastcall"(play_layer: usize) -> *const c_void {
        OMEGABOT.on_quit();
        get_orig!(ON_QUIT_O "fastcall"(usize) -> *const c_void)(play_layer)
    }}

    define_hook! { level_complete_h|LEVEL_COMPLETE_O "fastcall"(play_layer: usize) -> *const c_void {
        OMEGABOT.get_practice_fix().clear_checkpoints();
        get_orig!(LEVEL_COMPLETE_O "fastcall"(usize) -> *const c_void)(play_layer)
    }}

    define_hook! { on_editor_h|ON_EDITOR_O "fastcall"(play_layer: usize, _edx: usize, param: usize) -> *const c_void {
        OMEGABOT.on_quit();
        get_orig!(ON_EDITOR_O "fastcall"(usize, usize, usize) -> *const c_void)(play_layer, 0, param)
    }}

    define_hook! { toggle_practice_mode_h|TOGGLE_PRACTICE_MODE_O "fastcall"(play_layer: usize, _edx: usize, on: bool) {
        OMEGABOT.get_practice_fix().clear_checkpoints();
        get_orig!(TOGGLE_PRACTICE_MODE_O "fastcall"(usize, usize, bool))(play_layer, 0, on);
    }}

    define_hook! { create_checkpoint_h|CREATE_CHECKPOINT_O "fastcall"(play_layer: usize) -> *const c_void {
        let pl = gd::PlayLayer::from_address(play_layer);
        if !pl.is_dead() {
            OMEGABOT.get_practice_fix().add_checkpoint(pl);
        }
        get_orig!(CREATE_CHECKPOINT_O "fastcall"(usize) -> *const c_void)(play_layer)
    }}

    define_hook! { remove_checkpoint_h|REMOVE_CHECKPOINT_O "fastcall"(play_layer: usize) {
        OMEGABOT.get_practice_fix().remove_checkpoint();
        get_orig!(REMOVE_CHECKPOINT_O "fastcall"(usize))(play_layer);
    }}

    define_hook! { draw_h|DRAW_O "fastcall"(layer: usize) {
        if RENDER {
            get_orig!(DRAW_O "fastcall"(usize))(layer);
        }
    }}

    define_hook! { visit_h|VISIT_O "fastcall"(node: usize) {
        if RENDER {
            get_orig!(VISIT_O "fastcall"(usize))(node);
        }
    }}
}

pub mod objects {
    use crate::gd;
    use crate::OMEGABOT;

    define_hook! { ring_jump_h|RING_JUMP_O "fastcall"(player_object: usize, _edx: usize, ring: usize) {
        let object = gd::GameObject::from_address(ring);
        let before = *object.activated();
        let before_p2 = *object.activated_p2();
        get_orig!(RING_JUMP_O "fastcall"(usize, usize, usize))(player_object, 0, ring);

        if *object.activated() && !before {
            OMEGABOT.get_practice_fix().add_activated_object(object, false);
        }
        if *object.activated_p2() && !before_p2 {
            OMEGABOT.get_practice_fix().add_activated_object(object, true);
        }
    }}

    define_hook! { activated_object_h|ACTIVATED_OBJECT_O "fastcall"(game_object: usize, _edx: usize, player_object: usize) {
        let object = gd::GameObject::from_address(game_object);
        let before = *object.activated();
        let before_p2 = *object.activated_p2();
        get_orig!(ACTIVATED_OBJECT_O "fastcall"(usize, usize, usize))(game_object, 0, player_object);

        if *object.activated() && !before {
            OMEGABOT.get_practice_fix().add_activated_object(object, false);
        }
        if *object.activated_p2() && !before_p2 {
            OMEGABOT.get_practice_fix().add_activated_object(object, true);
        }
    }}

    define_hook! { bump_player_h|BUMP_PLAYER_O "fastcall"(game_layer: usize, _edx: usize, player_object: usize, game_object: usize) {
        let object = gd::GameObject::from_address(game_object);
        let before = *object.activated();
        let before_p2 = *object.activated_p2();
        get_orig!(BUMP_PLAYER_O "fastcall"(usize, usize, usize, usize))(game_layer, 0, player_object, game_object);

        if *object.activated() && !before {
            OMEGABOT.get_practice_fix().add_activated_object(object, false);
        }
        if *object.activated_p2() && !before_p2 {
            OMEGABOT.get_practice_fix().add_activated_object(object, true);
        }
    }}
}

pub unsafe fn hook() {
    let status = MH_Initialize();
    if status != MH_OK {
        panic!(
            "Failed to initialize minhook, error: {}",
            CStr::from_ptr(MH_StatusToString(status)).to_str().unwrap()
        );
    }

    // cocos2d hooks
    {
        use cocos2d::*;
        create_hook!(SCHEDULE_UPDATE_O -> schedule_update_h @ "libcocos2d.dll"."?update@CCScheduler@cocos2d@@UAEXM@Z");
        create_hook!(DISPATCH_KEYBOARD_MSG_O -> dispatch_keyboard_msg_h @ "libcocos2d.dll"."?dispatchKeyboardMSG@CCKeyboardDispatcher@cocos2d@@QAE_NW4enumKeyCodes@2@_N@Z");
    }

    // play_layer hooks
    {
        use play_layer::*;
        create_hook!(INIT_O -> init_h @ gd + 0x1FB780);
        create_hook!(UPDATE_O -> update_h @ gd + 0x2029C0);
        create_hook!(RESET_LEVEL_O -> reset_level_h @ gd + 0x20BF00);
        create_hook!(PUSH_BUTTON_O -> push_button_h @ gd + 0x111500);
        create_hook!(RELEASE_BUTTON_O -> release_button_h @ gd + 0x111660);
        create_hook!(DESTROY_PLAYER_O -> destroy_player_h @ gd + 0x20A1A0);
        create_hook!(ON_QUIT_O -> on_quit_h @ gd + 0x20D810);
        create_hook!(LEVEL_COMPLETE_O -> level_complete_h @ gd + 0x1FD3D0);
        create_hook!(ON_EDITOR_O -> on_editor_h @ gd + 0x1E60E0);
        create_hook!(TOGGLE_PRACTICE_MODE_O -> toggle_practice_mode_h @ gd + 0x20D0D0);
        create_hook!(CREATE_CHECKPOINT_O -> create_checkpoint_h @ gd + 0x20B050);
        create_hook!(REMOVE_CHECKPOINT_O -> remove_checkpoint_h @ gd + 0x20B830);
        create_hook!(DRAW_O -> draw_h @ gd + 0x208870);
        create_hook!(VISIT_O -> visit_h @ gd + 0x200020);
    }

    // objects hooks
    {
        use objects::*;
        create_hook!(RING_JUMP_O -> ring_jump_h @ gd + 0x1F4FF0);
        create_hook!(ACTIVATED_OBJECT_O -> activated_object_h @ gd + 0xEF0E0);
        create_hook!(BUMP_PLAYER_O -> bump_player_h @ gd + 0x10ED50);
    }
}

pub unsafe fn enable_hooks() {
    let status = MH_EnableHook(0 as _);
    if status != MH_OK {
        panic!(
            "Failed to enable hooks, error: {}",
            CStr::from_ptr(MH_StatusToString(status)).to_str().unwrap()
        );
    }
}

pub unsafe fn unhook() {
    MH_DisableHook(0 as _);

    remove_hook!("libcocos2d.dll"."?update@CCScheduler@cocos2d@@UAEXM@Z");
    remove_hook!("libcocos2d.dll"."?dispatchKeyboardMSG@CCKeyboardDispatcher@cocos2d@@QAE_NW4enumKeyCodes@2@_N@Z");

    remove_hook!(gd + 0x1FB780);
    remove_hook!(gd + 0x2029C0);
    remove_hook!(gd + 0x20BF00);
    remove_hook!(gd + 0x111500);
    remove_hook!(gd + 0x111660);
    remove_hook!(gd + 0x20A1A0);
    remove_hook!(gd + 0x20D810);
    remove_hook!(gd + 0x1FD3D0);
    remove_hook!(gd + 0x1E60E0);
    remove_hook!(gd + 0x20D0D0);
    remove_hook!(gd + 0x20B050);
    remove_hook!(gd + 0x20B830);
    remove_hook!(gd + 0x208870);
    remove_hook!(gd + 0x200020);

    remove_hook!(gd + 0x1F4FF0);
    remove_hook!(gd + 0xEF0E0);
    remove_hook!(gd + 0x10ED50);

    MH_Uninitialize();
}
