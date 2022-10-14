use crate::gd;
use crate::replay::ClickType;
use crate::utils::IsNull;

#[derive(Debug, Clone, Copy)]
enum SpamBotState {
    Off,
    StraightFly,
    Spam,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpamBotPlayer {
    Player1,
    Player2,
    Both,
}

pub struct SpamBot {
    state: SpamBotState,
    pub spam_player: SpamBotPlayer,
    pub straight_fly_player: SpamBotPlayer,
    start_y_p1: f32,
    start_y_p2: f32,
    pub y_accuracy: f32,
    pub spam_press: u32,
    pub spam_release: u32,
    frame: u32,
}

impl Default for SpamBot {
    fn default() -> Self {
        SpamBot {
            state: SpamBotState::Off,
            spam_player: SpamBotPlayer::Player1,
            straight_fly_player: SpamBotPlayer::Player1,
            start_y_p1: 0.0,
            start_y_p2: 0.0,
            y_accuracy: 40.0,
            spam_press: 5,
            spam_release: 5,
            frame: 0,
        }
    }
}

impl SpamBot {
    pub fn update(&mut self, play_layer: gd::PlayLayer) -> Vec<ClickType> {
        let mut clicks = Vec::new();
        match self.state {
            SpamBotState::Off => (),
            SpamBotState::StraightFly => {
                if matches!(
                    self.straight_fly_player,
                    SpamBotPlayer::Player1 | SpamBotPlayer::Both
                ) {
                    clicks.push(self.straight_fly_update(play_layer.player1(), self.start_y_p1));
                }
                if matches!(
                    self.straight_fly_player,
                    SpamBotPlayer::Player2 | SpamBotPlayer::Both
                ) {
                    let c = self.straight_fly_update(play_layer.player2(), self.start_y_p2);
                    clicks.push(match c {
                        ClickType::Player1Down => ClickType::Player2Down,
                        ClickType::Player1Up => ClickType::Player2Up,
                        _ => c,
                    });
                }
            }
            SpamBotState::Spam => {
                self.frame += 1;
                let holding = if matches!(
                    self.spam_player,
                    SpamBotPlayer::Player1 | SpamBotPlayer::Both
                ) || !play_layer.level_settings().is_2player()
                {
                    unsafe {
                        *play_layer.player1().get_is_holding()
                            || *play_layer.player1().get_is_holding2()
                    }
                } else {
                    unsafe {
                        *play_layer.player2().get_is_holding()
                            || *play_layer.player2().get_is_holding2()
                    }
                };
                let player1 = matches!(
                    self.spam_player,
                    SpamBotPlayer::Player1 | SpamBotPlayer::Both
                ) || !play_layer.level_settings().is_2player();
                let player2 = matches!(
                    self.spam_player,
                    SpamBotPlayer::Player2 | SpamBotPlayer::Both
                ) && play_layer.level_settings().is_2player();

                if self.frame
                    >= if holding {
                        self.spam_press
                    } else {
                        self.spam_release
                    }
                {
                    self.frame = 0;

                    if player1 {
                        clicks.push(if holding {
                            ClickType::Player1Up
                        } else {
                            ClickType::Player1Down
                        });
                    }
                    if player2 {
                        clicks.push(if holding {
                            ClickType::Player2Up
                        } else {
                            ClickType::Player2Down
                        });
                    }

                    if if holding {
                        self.spam_release
                    } else {
                        self.spam_press
                    } == 0
                    {
                        let holding = !holding;
                        if player1 {
                            clicks.push(if holding {
                                ClickType::Player1Up
                            } else {
                                ClickType::Player1Down
                            });
                        }
                        if player2 {
                            clicks.push(if holding {
                                ClickType::Player2Up
                            } else {
                                ClickType::Player2Down
                            });
                        }
                    }
                }
            }
        }
        clicks
    }

    fn straight_fly_update(&self, player: gd::PlayerObject, start_y: f32) -> ClickType {
        let mut y = unsafe { *player.get_position_float().1 };
        let mut accel = unsafe { *player.get_y_accel() } as f32;
        let holding = unsafe { *player.get_is_holding() || *player.get_is_holding2() };

        if unsafe { *player.get_is_upside_down() } {
            y = start_y - (y - start_y);
            accel = -accel;
        }

        if accel > 0.0 && y > start_y - accel + self.y_accuracy / 100.0 && holding {
            ClickType::Player1Up
        } else if accel < 0.0 && y < start_y - accel - self.y_accuracy / 100.0 && !holding {
            ClickType::Player1Down
        } else {
            ClickType::None
        }
    }

    pub fn start_straight_fly(&mut self) {
        gd::GameManager::get().if_not_null(|gm| {
            gm.play_layer().if_not_null(|pl| {
                let p1 = pl.player1();
                let p2 = pl.player2();
                if !p1.is_null() && !p2.is_null() {
                    self.start_y_p1 = unsafe { *p1.get_position_float().1 };
                    self.start_y_p2 = unsafe { *p2.get_position_float().1 };
                    self.state = SpamBotState::StraightFly;
                }
            })
        });
    }

    pub fn start_spam(&mut self) {
        gd::GameManager::get().if_not_null(|gm| {
            gm.play_layer().if_not_null(|_| {
                self.frame = 0;
                self.state = SpamBotState::Spam;
            })
        });
    }

    pub fn stop(&mut self) -> Vec<ClickType> {
        let mut clicks = Vec::new();
        if matches!(self.state, SpamBotState::StraightFly | SpamBotState::Spam) {
            gd::GameManager::get().if_not_null(|gm| {
                gm.play_layer().if_not_null(|pl| {
                    let mut player = match self.state {
                        SpamBotState::StraightFly => self.straight_fly_player,
                        SpamBotState::Spam => self.spam_player,
                        SpamBotState::Off => unreachable!(),
                    };
                    if !pl.level_settings().is_2player() {
                        player = SpamBotPlayer::Player1;
                    }
                    if matches!(player, SpamBotPlayer::Player1 | SpamBotPlayer::Both) {
                        pl.player1().if_not_null(|p| {
                            if unsafe { *p.get_is_holding() || *p.get_is_holding2() } {
                                clicks.push(ClickType::Player1Up);
                            }
                        });
                    }
                    if matches!(player, SpamBotPlayer::Player2 | SpamBotPlayer::Both) {
                        pl.player2().if_not_null(|p| {
                            if unsafe { *p.get_is_holding() || *p.get_is_holding2() } {
                                clicks.push(ClickType::Player2Up);
                            }
                        });
                    }
                })
            });
        }
        self.state = SpamBotState::Off;
        clicks
    }

    pub fn is_running(&self) -> bool {
        !matches!(self.state, SpamBotState::Off)
    }

    pub fn is_straight_flying(&self) -> bool {
        matches!(self.state, SpamBotState::StraightFly)
    }

    pub fn is_spamming(&self) -> bool {
        matches!(self.state, SpamBotState::Spam)
    }
}
