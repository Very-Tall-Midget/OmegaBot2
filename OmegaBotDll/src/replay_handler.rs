use crate::{gd, gd::IsNull, pipe::Message, replay::*, spam_bot::*, OMEGABOT};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ReplayHandlerState {
    Idle,
    Recording,
    Playing,
}

#[cfg(feature = "count_frames")]
pub struct ReplayHandler {
    state: ReplayHandlerState,
    default_replay_type: ReplayType,
    replay: Replay,
    default_fps: f32,
    frame: u32,
    spam_bot: SpamBot,
}

#[cfg(not(feature = "count_frames"))]
pub struct ReplayHandler {
    state: ReplayHandlerState,
    default_replay_type: ReplayType,
    replay: Replay,
    default_fps: f32,
    frame_offset: u32,
    time_offset: f64,
    spam_bot: SpamBot,
}

impl Default for ReplayHandler {
    #[cfg(feature = "count_frames")]
    fn default() -> Self {
        Self {
            state: ReplayHandlerState::Idle,
            default_replay_type: ReplayType::Frame,
            replay: Replay::new(60., ReplayType::Frame),
            default_fps: 60.,
            frame: 0,
            spam_bot: Default::default(),
        }
    }

    #[cfg(not(feature = "count_frames"))]
    fn default() -> Self {
        Self {
            state: ReplayHandlerState::Idle,
            default_replay_type: ReplayType::Frame,
            replay: Replay::new(60., ReplayType::Frame).into(),
            default_fps: 60.,
            frame_offset: 0,
            time_offset: 0.,
            spam_bot: Default::default(),
        }
    }
}

impl ReplayHandler {
    pub fn start_playback(&mut self) {
        self.state = ReplayHandlerState::Playing;
    }

    pub fn stop_playback(&mut self) {
        self.state = ReplayHandlerState::Idle;
    }

    pub fn start_recording(&mut self) {
        self.state = ReplayHandlerState::Recording;
        self.replay = Replay::new(self.default_fps, self.default_replay_type);
    }

    pub fn stop_recording(&mut self) {
        self.state = ReplayHandlerState::Idle;
        self.replay.finalise();
    }

    pub fn append(&mut self) -> Message {
        if self.state != ReplayHandlerState::Playing {
            Message::Error("Not playing".to_string())
        } else {
            self.state = ReplayHandlerState::Recording;
            Message::Received
        }
    }

    #[cfg(feature = "count_frames")]
    pub fn on_update(&mut self, play_layer: gd::PlayLayer) {
        if unsafe { !play_layer.is_dead() && !*play_layer.is_paused() } {
            if self.state == ReplayHandlerState::Playing {
                self.replay.for_all_current_clicks(
                    match self.replay.get_type() {
                        ReplayType::XPos => {
                            Location::XPos(unsafe { *play_layer.player1().get_position().0 })
                        }
                        ReplayType::Frame => Location::Frame(self.frame),
                    },
                    |c| unsafe {
                        if let ClickType::FpsChange(_) = c.click_type {
                            OMEGABOT.update_fps();
                        } else {
                            OMEGABOT.click(gd::GameManager::get().play_layer(), c.click_type);
                        }
                    },
                );
            }
            for click in self.spam_bot.update(play_layer) {
                self.click_h(play_layer, click);
            }
        }
        if play_layer
            .if_not_null(|pl| {
                pl.player1()
                    .if_not_null(|p| unsafe { *p.get_position_float().0 != 0. })
                    .unwrap_or(false)
            })
            .unwrap_or(false)
        {
            self.frame += 1;
        }
    }

    #[cfg(not(feature = "count_frames"))]
    pub fn on_update(&mut self, play_layer: gd::PlayLayer) {
        if unsafe { !play_layer.is_dead() && !*play_layer.is_paused() } {
            let old_fps = self.get_fps();
            if self.state == ReplayHandlerState::Playing {
                let frame = self.get_frame();
                self.replay.for_all_current_clicks(
                    match self.replay.get_type() {
                        ReplayType::XPos => {
                            Location::XPos(unsafe { *play_layer.player1().get_position().0 })
                        }
                        ReplayType::Frame => Location::Frame(frame),
                    },
                    |c| unsafe {
                        if let ClickType::FpsChange(_) = c.click_type {
                            OMEGABOT.update_fps();
                            OMEGABOT.on_fps_change(play_layer, old_fps);
                        } else {
                            OMEGABOT.click(gd::GameManager::get().play_layer(), c.click_type);
                        }
                    },
                );
            }
        }

        for click in self.spam_bot.update(play_layer) {
            self.click_h(play_layer, click);
        }
    }

    #[cfg(not(feature = "count_frames"))]
    pub fn on_fps_change(&mut self, play_layer: gd::PlayLayer, old_fps: f32) {
        if self.state == ReplayHandlerState::Playing {
            self.frame_offset = self.get_frame_from_fps(old_fps);
            self.time_offset = unsafe { *play_layer.time() };
        }
    }

    #[cfg(feature = "count_frames")]
    pub fn change_fps(&mut self, fps: f32) {
        if self.state == ReplayHandlerState::Recording {
            self.replay.change_fps(
                match self.replay.get_type() {
                    ReplayType::XPos => Location::XPos(unsafe {
                        gd::GameManager::get()
                            .if_not_null(|gm| {
                                gm.play_layer()
                                    .if_not_null(|pl| *pl.player1().get_position().0)
                                    .unwrap_or(0)
                            })
                            .unwrap_or(0)
                    }),
                    ReplayType::Frame => Location::Frame(self.frame - 1),
                },
                fps,
            );
        }
        self.default_fps = fps;
    }

    #[cfg(not(feature = "count_frames"))]
    pub fn change_fps(&mut self, fps: f32) {
        if self.state == ReplayHandlerState::Recording {
            self.frame_offset = self.get_frame();
            self.time_offset = gd::GameManager::get()
                .if_not_null(|gm| {
                    gm.play_layer()
                        .if_not_null(|pl| unsafe { *pl.time() })
                        .unwrap()
                })
                .unwrap();
            self.replay.change_fps(
                match self.replay.get_type() {
                    ReplayType::XPos => Location::XPos(unsafe {
                        gd::GameManager::get()
                            .if_not_null(|gm| {
                                gm.play_layer()
                                    .if_not_null(|pl| *pl.player1().get_position().0)
                                    .unwrap_or(0)
                            })
                            .unwrap_or(0)
                    }),
                    ReplayType::Frame => Location::Frame(self.frame_offset),
                },
                fps,
            );
        }
        self.default_fps = fps;
    }

    pub fn click(&self, play_layer: gd::PlayLayer, click_type: ClickType) {
        use crate::hooks::play_layer::*;

        match click_type {
            ClickType::FpsChange(_) | ClickType::None => {}
            _ => {
                let press = matches!(click_type, ClickType::Player1Down | ClickType::Player2Down);
                let func = unsafe {
                    if press {
                        get_orig!(PUSH_BUTTON_O "fastcall"(usize, usize, i32, bool) -> u32)
                    } else {
                        get_orig!(RELEASE_BUTTON_O "fastcall"(usize, usize, i32, bool) -> u32)
                    }
                };
                let player2 = matches!(click_type, ClickType::Player2Down | ClickType::Player2Up);
                let flip = gd::GameManager::get().get_game_variable("0010");
                unsafe {
                    func(play_layer.into(), 0, 0, !player2 ^ flip);
                }
            }
        }
    }

    pub fn click_h(&self, play_layer: gd::PlayLayer, click_type: ClickType) {
        use crate::hooks::play_layer::*;

        match click_type {
            ClickType::FpsChange(_) | ClickType::None => {}
            _ => {
                let press = matches!(click_type, ClickType::Player1Down | ClickType::Player2Down);
                let func = if press {
                    push_button_h
                } else {
                    release_button_h
                };
                let player2 = matches!(click_type, ClickType::Player2Down | ClickType::Player2Up);
                let flip = gd::GameManager::get().get_game_variable("0010");
                unsafe {
                    func(play_layer.into(), 0, 0, !player2 ^ flip);
                }
            }
        }
    }

    pub fn on_click(&mut self, play_layer: gd::PlayLayer, press: bool, button: bool) {
        if self.state == ReplayHandlerState::Recording {
            let is2player = play_layer.level_settings().is_2player();
            let flip = is2player && gd::GameManager::get().get_game_variable("0010");
            let player1 = !is2player || (button ^ flip);
            let click_type = if player1 {
                if press {
                    ClickType::Player1Down
                } else {
                    ClickType::Player1Up
                }
            } else if press {
                ClickType::Player2Down
            } else {
                ClickType::Player2Up
            };

            let frame = self.get_frame();
            self.replay.add_click(Click {
                location: match self.replay.get_type() {
                    ReplayType::XPos => {
                        Location::XPos(unsafe { *play_layer.player1().get_position().0 })
                    }
                    ReplayType::Frame => Location::Frame(frame),
                },
                click_type,
            });
        }
    }

    pub fn on_reset_level(&mut self, play_layer: gd::PlayLayer) {
        #[cfg(feature = "count_frames")]
        {
            self.frame = 0;
        }
        #[cfg(not(feature = "count_frames"))]
        {
            self.frame_offset = 0;
            self.time_offset = 0.;
        }

        if self.spam_bot.is_running() {
            self.spam_bot.stop();
        }

        unsafe {
            OMEGABOT.get_practice_fix().on_reset(play_layer);
        }

        if self.state != ReplayHandlerState::Idle {
            let frame = self.get_frame();
            self.replay.reset(
                match self.replay.get_type() {
                    ReplayType::XPos => {
                        Location::XPos(unsafe { *play_layer.player1().get_position().0 })
                    }
                    ReplayType::Frame => Location::Frame(frame),
                },
                self.state == ReplayHandlerState::Recording,
            );
        }
    }

    pub fn on_quit(&mut self) {
        self.spam_bot.stop();
    }

    pub fn toggle_straight_fly(&mut self) {
        if self.spam_bot.is_straight_flying() {
            for click in self.spam_bot.stop() {
                self.click_h(gd::GameManager::get().play_layer(), click);
            }
        } else {
            self.spam_bot.start_straight_fly();
        }
    }

    pub fn toggle_spam(&mut self) {
        if self.spam_bot.is_spamming() {
            for click in self.spam_bot.stop() {
                self.click_h(gd::GameManager::get().play_layer(), click);
            }
        } else {
            self.spam_bot.start_spam();
        }
    }

    pub fn save_replay(&self, filename: String) -> Option<Message> {
        let res = self.replay.serialise();
        if let Ok(data) = res {
            let res = std::fs::write(filename, data);
            if let Err(e) = res {
                Some(Message::Error(e.to_string()))
            } else {
                Some(Message::Received)
            }
        } else {
            Some(Message::Error(res.unwrap_err()))
        }
    }

    pub fn load_replay(&mut self, filename: String) -> Option<Message> {
        let data = std::fs::read(filename);
        if let Ok(data) = data {
            let res = Replay::deserialise(data);
            if let Ok(res) = res {
                self.replay = res;
                Some(Message::Received)
            } else {
                Some(Message::Error(res.unwrap_err()))
            }
        } else {
            Some(Message::Error(data.unwrap_err().to_string()))
        }
    }
}

// Getters and setters
impl ReplayHandler {
    pub fn get_state(&self) -> ReplayHandlerState {
        self.state
    }

    pub fn get_fps(&self) -> f32 {
        match self.state {
            ReplayHandlerState::Idle => self.default_fps,
            ReplayHandlerState::Recording | ReplayHandlerState::Playing => {
                self.replay.get_current_fps()
            }
        }
    }

    pub fn get_frame(&mut self) -> u32 {
        #[cfg(feature = "count_frames")]
        {
            self.frame
        }
        #[cfg(not(feature = "count_frames"))]
        {
            self.get_frame_from_fps(self.get_fps())
        }
    }

    #[cfg(not(feature = "count_frames"))]
    pub fn get_frame_from_fps(&mut self, fps: f32) -> u32 {
        gd::GameManager::get()
            .if_not_null(|gm| {
                gm.play_layer()
                    .if_not_null(|pl| {
                        let time = unsafe { *pl.time() } - self.time_offset;
                        let frame = time * (fps as f64) + (self.frame_offset as f64);
                        frame.round() as u32
                    })
                    .unwrap_or(0)
            })
            .unwrap_or(0)
    }

    #[cfg(feature = "count_frames")]
    pub fn set_frame(&mut self, frame: u32) {
        self.frame = frame;
    }

    #[cfg(not(feature = "count_frames"))]
    pub fn set_frame(&mut self, frame_offset: u32, time_offset: f64) {
        self.time_offset = time_offset;
        self.frame_offset = frame_offset;
    }

    #[cfg(not(feature = "count_frames"))]
    pub fn get_time_offset(&self) -> f64 {
        self.time_offset
    }

    #[cfg(not(feature = "count_frames"))]
    pub fn get_frame_offset(&self) -> u32 {
        self.frame_offset
    }

    pub fn set_replay_type(&mut self, replay_type: ReplayType) {
        self.default_replay_type = replay_type;
    }

    pub fn get_spam_bot(&mut self) -> &mut SpamBot {
        &mut self.spam_bot
    }
}
