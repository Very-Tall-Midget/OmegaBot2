use once_cell::sync::Lazy;

use crate::{
    cocos2d, create_hack, gd, hack_handler::*, hooks, mutex_count::*, pipe::*,
    practice_fix::PracticeFix, replay::*, replay_handler::*, utils::*,
};

pub static mut OMEGABOT: Lazy<Box<OmegaBot>> = Lazy::new(|| Box::new(OmegaBot::new()));

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum NoClip {
    Off,
    Player1,
    Player2,
    Both,
}

impl From<u16> for NoClip {
    fn from(value: u16) -> Self {
        match value {
            1 => NoClip::Off,
            2 => NoClip::Player1,
            3 => NoClip::Player2,
            4 => NoClip::Both,
            _ => unreachable!(),
        }
    }
}

impl From<NoClip> for u16 {
    fn from(value: NoClip) -> Self {
        match value {
            NoClip::Off => 1,
            NoClip::Player1 => 2,
            NoClip::Player2 => 3,
            NoClip::Both => 4,
        }
    }
}

type QueuedMessage = Box<dyn FnOnce(&mut Pipe)>;

pub struct OmegaBot {
    initialised: bool,
    running: bool,
    pipe: Pipe,

    replay_handler: ReplayHandler,
    hack_handler: HackHandler,

    speedhack: f32,
    respawn_time: Box<f32>,
    no_clip: NoClip,
    queued_functions: Vec<Box<dyn FnOnce()>>,

    update_lock: MutexCount,
    queued_messages: Vec<QueuedMessage>,
    practice_fix: PracticeFix,

    frame_count_float: f32,
    frame_count_int: u32,

    frame_advance: bool,
}

impl OmegaBot {
    fn new() -> Self {
        Self {
            initialised: false,
            running: false,
            pipe: Pipe::new("OmegaBotPipe"),
            replay_handler: Default::default(),
            hack_handler: Default::default(),
            speedhack: 1.,
            respawn_time: Box::new(1.),
            no_clip: NoClip::Off,
            update_lock: Default::default(),
            queued_functions: Vec::new(),
            queued_messages: Vec::new(),
            practice_fix: Default::default(),
            frame_count_float: 0.,
            frame_count_int: 0,
            frame_advance: false,
        }
    }

    fn load_hacks(&mut self) {
        self.hack_handler.add_hack(
            HackName::NoSpike,
            create_hack!({ gd::get_base() + 0x205347, { 0x75 }, { 0x74 } }),
        );

        self.hack_handler.add_hack(
            HackName::PracticeMusicFix,
            create_hack!(
                { gd::get_base() + 0x20C925, { 0x90, 0x90, 0x90, 0x90, 0x90, 0x90 }, { 0x0F, 0x85, 0xF7, 0x00, 0x00, 0x00 } },
                { gd::get_base() + 0x20D143, { 0x90, 0x90 }, { 0x75, 0x41 } },
                { gd::get_base() + 0x20A563, { 0x90, 0x90 }, { 0x75, 0x3E } },
                { gd::get_base() + 0x20A595, { 0x90, 0x90 }, { 0x75, 0x0C } },
                // Practice user coins
                { gd::get_base() + 0x204F10, { 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90 }, { 0x80, 0xBE, 0x95, 0x04, 0x00, 0x00, 0x00, 0x0F, 0x85, 0xDE, 0x00, 0x00, 0x00 } },
                // Practice pulse
                { gd::get_base() + 0x205536, { 0x90, 0x90 }, { 0x75, 0x08 } },
                { gd::get_base() + 0x20553E, { 0xEB, 0x1F }, { 0x74, 0x13 } },
            ),
        );

        self.hack_handler.add_hack(
            HackName::AntiCheatBypass,
            create_hack!(
                { gd::get_base() + 0x202AAA, { 0xEB, 0x2E }, { 0x74, 0x2E } },

                { gd::get_base() + 0x15FC2E, { 0xEB }, { 0x74 } },
                { gd::get_base() + 0x1FD335, { 0xEB }, { 0x74 } },

                { gd::get_base() + 0x1FD557, { 0xEB, 0x0C }, { 0x74, 0x0C } },
                { gd::get_base() + 0x1FD742, { 0xC7, 0x87, 0xE0, 0x02, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0xC7, 0x87, 0xE4, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90 }, { 0x80, 0xBF, 0xDD, 0x02, 0x00, 0x00, 0x00, 0x0F, 0x85, 0x0A, 0xFE, 0xFF, 0xFF, 0x80, 0xBF, 0x34, 0x05, 0x00, 0x00, 0x00, 0x0F, 0x84, 0xFD, 0xFD, 0xFF, 0xFF } },
                { gd::get_base() + 0x1FD756, { 0x90, 0x90, 0x90, 0x90, 0x90, 0x90 }, { 0x0F, 0x84, 0xFD, 0xFD, 0xFF, 0xFF } },
                { gd::get_base() + 0x1FD79A, { 0x90, 0x90, 0x90, 0x90, 0x90, 0x90 }, { 0x0F, 0x84, 0xB9, 0xFD, 0xFF, 0xFF } },
                { gd::get_base() + 0x1FD7AF, { 0x90, 0x90, 0x90, 0x90, 0x90, 0x90 }, { 0x0F, 0x85, 0xA4, 0xFD, 0xFF, 0xFF } },

                { gd::get_base() + 0x20D3B3, { 0x90, 0x90, 0x90, 0x90, 0x90 }, { 0xE8, 0x58, 0x04, 0x00, 0x00 } },
                { gd::get_base() + 0x1FF7A2, { 0x90, 0x90 }, { 0x74, 0x6E } },

                { gd::get_base() + 0x18B2B4, { 0xB0, 0x01 }, { 0x88, 0xD8 } },

                { gd::get_base() + 0x20C4E6, { 0xE9, 0xD7, 0x00, 0x00, 0x00, 0x90 }, { 0x0F, 0x85, 0xD6, 0x00, 0x00, 0x00 } },
            ),
        );

        self.hack_handler.add_hack(
            HackName::NoRespawnFlash,
            create_hack!({ gd::get_base() + 0x1EF36D, { 0xE9, 0xA8, 0x00, 0x00, 0x00, 0x90 }, { 0x0F, 0x85, 0xA7, 0x00, 0x00, 0x00 } }),
        );

        self.hack_handler.add_hack(
            HackName::IgnoreEscape,
            create_hack!({ gd::get_base() + 0x1E644C, { 0x90, 0x90, 0x90, 0x90, 0x90 }, { 0xE8, 0xBF, 0x73, 0x02, 0x00 } }),
        );

        self.hack_handler.add_hack(
            HackName::DisableDeathEffect,
            create_hack!({ gd::get_base() + 0x1EFBA4, { 0x90, 0x90, 0x90, 0x90, 0x90 }, { 0xE8, 0x37, 0x00, 0x00, 0x00 } }),
        );
    }

    pub fn initialise(&mut self) -> bool {
        if self.initialised {
            return false;
        }

        self.load_hacks();

        self.pipe.connect();
        unsafe {
            self.create_console();
            println!("Initialising...");
            hooks::hook();
        }
        self.initialised = true;

        true
    }

    pub fn run(&mut self) {
        if !self.initialised || self.running {
            return;
        }

        println!("Running...");
        self.running = true;
        self.update_lock.clear();
        unsafe {
            hooks::enable_hooks();
        }

        self.hack_handler.apply(HackName::AntiCheatBypass);

        let thread = std::thread::spawn(|| unsafe {
            while OMEGABOT.is_running() {
                OMEGABOT.update_fps();
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        });

        while self.running && self.pipe.exists() {
            let msg = self.pipe.read();
            if let Some(msg) = self.parse_message(msg) {
                self.pipe.write(msg, false);
            }
        }
        thread.join().unwrap();

        self.update_lock.deactivate();
        self.uninitialise();
    }

    pub fn uninitialise(&mut self) {
        if !self.initialised {
            return;
        }

        self.running = false;
        println!("Uninitialising...");
        self.pipe.disconnect();
        unsafe {
            hooks::unhook();
            winapi::um::wincon::FreeConsole();
        }
    }

    pub fn play_update(&mut self, scheduler: usize, real_dt: f32) {
        use hooks::cocos2d::*;

        if self.replay_handler.get_state() == ReplayHandlerState::Playing
            && gd::GameManager::get()
                .if_not_null(|gm| {
                    gm.play_layer()
                        .if_not_null(|pl| unsafe { !*pl.is_paused() })
                        .unwrap_or(false)
                })
                .unwrap_or(false)
        {
            let real_fps = 1. / real_dt;
            let target_fps = self.get_fps();
            self.frame_count_float += (target_fps / (real_fps / self.speedhack)).min(300.);
            let mut times = self.frame_count_float as u32 - self.frame_count_int;
            self.frame_count_int += times;

            if self.frame_count_float >= target_fps {
                times += self.frame_count_float.round() as u32 - self.frame_count_int;
                self.frame_count_float = 0.;
                self.frame_count_int = 0;
            }

            if times > 0 {
                unsafe {
                    hooks::play_layer::RENDER = false;
                }
                let play_layer = gd::GameManager::get().play_layer();
                for _ in 0..(times - 1) {
                    if play_layer.if_not_null(|pl| pl.is_dead()).unwrap_or(true) {
                        break;
                    }
                    self.wait_for_update();
                    unsafe {
                        get_orig!(SCHEDULE_UPDATE_O "fastcall"(usize, usize, f32))(
                            scheduler,
                            0,
                            self.get_delta_time(),
                        );
                    }
                }
                unsafe {
                    hooks::play_layer::RENDER = true;
                }

                self.wait_for_update();
                unsafe {
                    get_orig!(SCHEDULE_UPDATE_O "fastcall"(usize, usize, f32))(
                        scheduler,
                        0,
                        self.get_delta_time(),
                    );
                }
            }
        } else {
            unsafe {
                get_orig!(SCHEDULE_UPDATE_O "fastcall"(usize, usize, f32))(
                    scheduler,
                    0,
                    self.get_delta_time(),
                );
            }
        }
    }

    fn parse_message(&mut self, message: Message) -> Option<Message> {
        if !matches!(message, Message::Ping) {
            println!("Message: {:?}", message);
        }
        match message {
            Message::Ping => {
                for func in self.queued_messages.drain(..) {
                    func(&mut self.pipe);
                }
                Some(Message::Ping)
            }
            Message::Error(_) | Message::Received => unreachable!(),
            Message::Exit => {
                self.running = false;
                None
            }
            Message::ChangeFPS(fps) => {
                self.replay_handler.change_fps(fps);
                self.update_fps();
                Some(Message::Received)
            }
            Message::Speedhack(speedhack) => {
                self.speedhack = speedhack;
                self.update_fps();
                Some(Message::Received)
            }
            Message::RespawnTime(respawn_time) => {
                self.set_respawn_time(respawn_time);
                Some(Message::Received)
            }
            Message::FrameAdvance(frame_advance) => {
                if self.frame_advance != frame_advance {
                    self.toggle_frame_advance();
                }
                Some(Message::Received)
            }
            Message::AccuracyFix(accuracy_fix) => {
                self.replay_handler.set_accuracy_fix(accuracy_fix);
                Some(Message::Received)
            }
            Message::PracticeFix(practice_fix) => {
                self.practice_fix.set_active(practice_fix);
                Some(Message::Received)
            }
            Message::SetNoClip(no_clip) => {
                self.no_clip = no_clip;
                Some(Message::Received)
            }
            Message::StartPlayback => {
                self.replay_handler.start_playback();
                Some(Message::Received)
            }
            Message::StopPlayback => {
                self.replay_handler.stop_playback();
                Some(Message::Received)
            }
            Message::StartRecording => {
                self.replay_handler.start_recording();
                Some(Message::Received)
            }
            Message::StopRecording => {
                self.replay_handler.stop_recording();
                Some(Message::Received)
            }
            Message::Append => Some(self.replay_handler.append()),
            Message::SaveReplay(filename) => self.replay_handler.save_replay(filename),
            Message::LoadReplay(filename) => self.replay_handler.load_replay(filename),
            Message::ApplyHack(hack) => {
                self.hack_handler.apply(hack);
                Some(Message::Received)
            }
            Message::RestoreHack(hack) => {
                self.hack_handler.restore(hack);
                Some(Message::Received)
            }
            Message::SetReplayType(replay_type) => {
                self.replay_handler.set_replay_type(replay_type);
                Some(Message::Received)
            }
        }
    }

    pub fn update_fps(&self) {
        let dt = 1. / (self.get_fps() as f64 * self.speedhack as f64);
        cocos2d::CCApplication::shared_application().set_animation_interval(dt);
    }

    unsafe fn create_console(&self) {
        winapi::um::consoleapi::AllocConsole();
        winapi::um::wincon::SetConsoleTitleA(lpcstr!("OmegaBot Console"));
        println!("Console initialised");
    }

    pub fn on_init(&mut self, _level: usize) {}

    pub fn on_update(&mut self, play_layer: gd::PlayLayer) {
        self.block_update();

        self.replay_handler.on_update(play_layer);

        self.unblock_update();
    }

    pub fn allow_death(&mut self, play_layer: gd::PlayLayer, player: gd::PlayerObject) -> bool {
        match self.no_clip {
            NoClip::Off => true,
            NoClip::Both => false,
            NoClip::Player1 => play_layer.player1() != player,
            NoClip::Player2 => play_layer.player2() != player,
        }
    }

    pub fn click(&self, play_layer: gd::PlayLayer, click_type: ClickType) {
        self.replay_handler.click(play_layer, click_type);
    }

    pub fn click_h(&self, play_layer: gd::PlayLayer, click_type: ClickType) {
        self.replay_handler.click_h(play_layer, click_type);
    }

    pub fn get_frame(&mut self) -> u32 {
        self.replay_handler.get_frame()
    }

    #[cfg(not(feature = "count_frames"))]
    pub fn on_fps_change(&mut self, play_layer: gd::PlayLayer, old_fps: f32) {
        self.block_update();

        self.replay_handler.on_fps_change(play_layer, old_fps);

        self.unblock_update();
    }

    pub fn on_click(&mut self, play_layer: gd::PlayLayer, press: bool, button: bool) {
        self.block_update();

        self.replay_handler.on_click(play_layer, press, button);

        self.unblock_update();
    }

    pub fn on_reset_level(&mut self, play_layer: gd::PlayLayer) {
        self.block_update();

        // unsafe {
        //     if self.player.is_null() {
        //         // Allocation size of player
        //         let addr = winapi::um::memoryapi::VirtualAlloc(
        //             0 as _,
        //             0x9E0,
        //             winapi::um::winnt::MEM_RESERVE | winapi::um::winnt::MEM_COMMIT,
        //             winapi::um::winnt::PAGE_EXECUTE_READWRITE,
        //         ) as usize;
        //         // Call constructor
        //         let addr = std::mem::transmute::<usize, unsafe extern "fastcall" fn(usize) -> usize>(
        //             gd::get_base() + 0x1E6650,
        //         )(addr);
        //         // Call init
        //         std::mem::transmute::<
        //             usize,
        //             unsafe extern "fastcall" fn(usize, usize, usize, usize, usize),
        //         >(gd::get_base() + 0x1E6DA0)(addr, 0, 133, 114, 0);
        //         self.player = gd::PlayerObject::from_address(addr);
        //     }
        // }

        self.replay_handler.on_reset_level(play_layer);
        self.update_fps();

        if self.replay_handler.get_state() == ReplayHandlerState::Recording {
            let fps = self.get_fps();
            self.queue_message(Box::new(move |pipe| {
                pipe.write(Message::ChangeFPS(fps), true)
            }));
        }

        self.unblock_update();
    }

    pub fn on_main_thread(&mut self) {
        self.block_update();
        for func in self.queued_functions.drain(..) {
            func();
        }
        self.unblock_update();
    }

    pub fn on_quit(&mut self) {
        self.practice_fix.clear_checkpoints();
    }

    pub fn toggle_frame_advance(&mut self) {
        self.frame_advance = !self.frame_advance;
    }

    pub fn advance_frame(&mut self) {
        if !self.frame_advance {
            self.frame_advance = true;
        }

        use hooks::cocos2d::*;
        unsafe {
            get_orig!(SCHEDULE_UPDATE_O "fastcall"(usize, usize, f32))(
                cocos2d::SharedDirector::get().get_scheduler(),
                0,
                self.get_delta_time(),
            );
        }
    }
}

// Getters and setters
impl OmegaBot {
    pub fn get_fps(&self) -> f32 {
        self.replay_handler.get_fps()
    }

    pub fn get_speedhack(&self) -> f32 {
        self.speedhack
    }

    pub fn get_delta_time(&self) -> f32 {
        1. / (self.get_fps() * self.get_speedhack()) * self.get_speedhack()
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn wait_for_update(&self) -> bool {
        self.update_lock.wait()
    }

    pub fn block_update(&mut self) {
        self.update_lock.lock()
    }

    pub fn unblock_update(&mut self) {
        self.update_lock.unlock();
    }

    pub fn queue_function(&mut self, f: Box<dyn FnOnce()>) {
        self.queued_functions.push(f);
    }

    pub fn queue_message(&mut self, f: QueuedMessage) {
        self.queued_messages.push(f);
    }

    #[cfg(feature = "count_frames")]
    pub fn set_frame(&mut self, frame: u32) {
        self.replay_handler.set_frame(frame);
    }

    #[cfg(not(feature = "count_frames"))]
    pub fn set_frame(&mut self, frame_offset: u32, time_offset: f64) {
        self.replay_handler.set_frame(frame_offset, time_offset);
    }

    #[cfg(not(feature = "count_frames"))]
    pub fn get_time_offset(&self) -> f64 {
        self.replay_handler.get_time_offset()
    }

    #[cfg(not(feature = "count_frames"))]
    pub fn get_frame_offset(&self) -> u32 {
        self.replay_handler.get_frame_offset()
    }

    pub fn get_practice_fix(&mut self) -> &mut PracticeFix {
        &mut self.practice_fix
    }

    pub fn set_respawn_time(&mut self, respawn_time: f32) {
        unsafe {
            *self.respawn_time = respawn_time;
            let mut addr = self.respawn_time.as_mut() as *mut f32;
            // requires WriteProcessMemory instead of using type punning bc data may not be initialised yet or some shit idfk
            if winapi::um::memoryapi::WriteProcessMemory(
                winapi::um::processthreadsapi::GetCurrentProcess(),
                (gd::get_base() + 0x20A677) as _,
                (&mut addr as *mut *mut f32) as _,
                4,
                std::ptr::null_mut(),
            ) == 0
            {
                self.queue_message(Box::new(move |pipe| {
                    pipe.write(
                        Message::Error("Failed to write respawn time to memory".to_string()),
                        false,
                    );
                }));
            }
        }
    }

    pub fn frame_advance(&self) -> bool {
        self.frame_advance
    }

    pub fn get_frame_advance_toggle_key(&self) -> char {
        'V'
    }

    pub fn get_frame_advance_key(&self) -> char {
        'C'
    }
}
