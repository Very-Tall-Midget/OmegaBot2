use crate::gd;
use crate::replay::ClickType;
use crate::utils::IsNull;
use crate::OMEGABOT;

struct CheckpointData {
    x_accel: f64,
    y_accel: f64,
    jump_accel: f64,
    is_holding: bool,
    has_just_held: bool,
    is_holding2: bool,
    has_just_held2: bool,
    can_robot_jump: bool,
    is_upside_down: bool,
    is_on_ground: bool,
    is_dashing: bool,
    is_sliding: bool,
    is_rising: bool,
    black_orb: bool,
    unk662: bool,
    unk630: bool,
    unk631: bool,
    vehicle_size: f32,
    player_speed: f32,
    rotation_x: f32,
    rotation_y: f32,
    x_pos: f32,
    y_pos: f32,
    game_mode: gd::GameMode,
}

impl CheckpointData {
    fn create(player: gd::PlayerObject) -> Self {
        unsafe {
            Self {
                x_accel: *player.get_x_accel(),
                y_accel: *player.get_y_accel(),
                jump_accel: *player.get_jump_accel(),
                is_holding: *player.get_is_holding(),
                has_just_held: *player.get_has_just_held(),
                is_holding2: *player.get_is_holding2(),
                has_just_held2: *player.get_has_just_held2(),
                can_robot_jump: *player.get_can_robot_jump(),
                is_upside_down: *player.get_is_upside_down(),
                is_on_ground: *player.get_is_on_ground(),
                is_dashing: *player.get_is_dashing(),
                is_sliding: *player.get_is_sliding(),
                is_rising: *player.get_is_rising(),
                black_orb: *player.get_black_orb(),
                unk662: *player.get_unk662(),
                unk630: *player.get_unk630(),
                unk631: *player.get_unk631(),
                vehicle_size: *player.get_vehicle_size(),
                player_speed: *player.get_player_speed(),
                rotation_x: *player.get_rotation_x(),
                rotation_y: *player.get_rotation_y(),
                x_pos: *player.get_position_float().0,
                y_pos: *player.get_position_float().1,
                game_mode: player.get_game_mode(),
            }
        }
    }

    fn apply(&self, player: gd::PlayerObject) -> u8 {
        let mut out = 0;
        unsafe {
            *player.get_x_accel() = self.x_accel;
            *player.get_y_accel() = self.y_accel;
            *player.get_jump_accel() = self.jump_accel;
            if self.is_holding != *player.get_is_holding() {
                out = if *player.get_is_holding() { 2 } else { 1 }; // 2 == press, 1 == release
            }
            *player.get_is_holding() = self.is_holding;
            *player.get_has_just_held() = self.has_just_held;
            *player.get_is_holding2() = self.is_holding2;
            *player.get_has_just_held2() = self.has_just_held2;
            *player.get_can_robot_jump() = self.can_robot_jump;
            *player.get_is_upside_down() = self.is_upside_down;
            *player.get_is_on_ground() = self.is_on_ground;
            *player.get_is_dashing() = self.is_dashing;
            *player.get_is_sliding() = self.is_sliding;
            *player.get_is_rising() = self.is_rising;
            *player.get_black_orb() = self.black_orb;
            *player.get_unk662() = self.unk662;
            *player.get_unk630() = self.unk630;
            *player.get_unk631() = self.unk631;
            *player.get_vehicle_size() = self.vehicle_size;
            *player.get_player_speed() = self.player_speed;
            *player.get_rotation_x() = self.rotation_x;
            *player.get_rotation_y() = self.rotation_y;
            *player.get_position_float().0 = self.x_pos;
            *player.get_position_float().1 = self.y_pos;
            player.set_game_mode(self.game_mode);
        }
        out
    }
}

#[cfg(feature = "count_frames")]
struct Checkpoint {
    pub player1: CheckpointData,
    pub player2: CheckpointData,
    pub number_of_activated_objects: usize,
    pub number_of_activated_objects_p2: usize,
    pub time: f64,
    pub frame: u32,
}

#[cfg(not(feature = "count_frames"))]
struct Checkpoint {
    pub player1: CheckpointData,
    pub player2: CheckpointData,
    pub number_of_activated_objects: usize,
    pub number_of_activated_objects_p2: usize,
    pub time: f64,
    pub time_offset: f64,
    pub frame_offset: u32,
}

impl Checkpoint {
    pub fn apply(&self, play_layer: gd::PlayLayer, auto_hold: bool) {
        let click = self.player1.apply(play_layer.player1());
        if !auto_hold && click != 0 {
            unsafe {
                OMEGABOT.queue_function(Box::new(move || {
                    OMEGABOT.click_h(
                        play_layer,
                        if click == 2 {
                            ClickType::Player1Down
                        } else {
                            ClickType::Player1Up
                        },
                    )
                }));
            }
        }

        let click = self.player2.apply(play_layer.player2());
        if !auto_hold && click != 0 && !play_layer.player2().is_null() {
            unsafe {
                OMEGABOT.queue_function(Box::new(move || {
                    OMEGABOT.click_h(
                        play_layer,
                        if click == 2 {
                            ClickType::Player2Down
                        } else {
                            ClickType::Player2Up
                        },
                    )
                }));
            }
        }

        #[cfg(feature = "count_frames")]
        unsafe {
            *play_layer.time() = self.time;
            OMEGABOT.set_frame(self.frame);
        }

        #[cfg(not(feature = "count_frames"))]
        unsafe {
            *play_layer.time() = self.time;
            OMEGABOT.set_frame(self.frame_offset, self.time_offset);
        }
    }
}

pub struct PracticeFix {
    checkpoints: Vec<Checkpoint>,
    activated_objects: Vec<gd::GameObject>,
    activated_objects_p2: Vec<gd::GameObject>,
    active: bool,
    auto_hold: bool,
}

impl Default for PracticeFix {
    fn default() -> Self {
        Self {
            checkpoints: Vec::new(),
            activated_objects: Vec::new(),
            activated_objects_p2: Vec::new(),
            active: true,
            auto_hold: true,
        }
    }
}

impl PracticeFix {
    #[cfg(feature = "count_frames")]
    pub fn add_checkpoint(&mut self, play_layer: gd::PlayLayer) {
        self.checkpoints.push(Checkpoint {
            player1: CheckpointData::create(play_layer.player1()),
            player2: CheckpointData::create(play_layer.player2()),
            number_of_activated_objects: self.activated_objects.len(),
            number_of_activated_objects_p2: self.activated_objects_p2.len(),
            time: unsafe { *play_layer.time() },
            frame: unsafe { OMEGABOT.get_frame() },
        });
    }

    #[cfg(not(feature = "count_frames"))]
    pub fn add_checkpoint(&mut self, play_layer: gd::PlayLayer) {
        self.checkpoints.push(Checkpoint {
            player1: CheckpointData::create(play_layer.player1()),
            player2: CheckpointData::create(play_layer.player2()),
            number_of_activated_objects: self.activated_objects.len(),
            number_of_activated_objects_p2: self.activated_objects_p2.len(),
            time: unsafe { *play_layer.time() },
            time_offset: unsafe { OMEGABOT.get_time_offset() },
            frame_offset: unsafe { OMEGABOT.get_frame_offset() },
        });
    }

    pub fn remove_checkpoint(&mut self) {
        if self.checkpoints.is_empty() {
            self.activated_objects.clear();
            self.activated_objects_p2.clear();
        } else {
            self.checkpoints.pop();
            self.activated_objects
                .truncate(self.checkpoints.last().unwrap().number_of_activated_objects);
            self.activated_objects_p2.truncate(
                self.checkpoints
                    .last()
                    .unwrap()
                    .number_of_activated_objects_p2,
            );
        }
    }

    pub fn apply_checkpoint(&self, play_layer: gd::PlayLayer) {
        if !self.checkpoints.is_empty() {
            if self.active {
                self.checkpoints
                    .last()
                    .unwrap()
                    .apply(play_layer, self.auto_hold);
            } else {
                let checkpoint = self.checkpoints.last().unwrap();

                #[cfg(feature = "count_frames")]
                unsafe {
                    *play_layer.time() = checkpoint.time;
                    OMEGABOT.set_frame(checkpoint.frame);
                }

                #[cfg(not(feature = "count_frames"))]
                unsafe {
                    *play_layer.time() = checkpoint.time;
                    OMEGABOT.set_frame(checkpoint.frame_offset, checkpoint.time_offset);
                }
            }
        }
    }

    pub fn clear_checkpoints(&mut self) {
        self.checkpoints.clear();
        self.activated_objects.clear();
        self.activated_objects_p2.clear();
    }

    pub fn add_activated_object(&mut self, object: gd::GameObject, player2: bool) {
        if player2 {
            self.activated_objects_p2.push(object);
        } else {
            self.activated_objects.push(object);
        }
    }

    pub fn on_reset(&mut self, play_layer: gd::PlayLayer) {
        if self.checkpoints.is_empty() {
            self.activated_objects.clear();
            self.activated_objects_p2.clear();
        } else if play_layer.is_practice_mode() {
            self.activated_objects
                .truncate(self.checkpoints.last().unwrap().number_of_activated_objects);
            self.activated_objects_p2.truncate(
                self.checkpoints
                    .last()
                    .unwrap()
                    .number_of_activated_objects_p2,
            );

            for object in &self.activated_objects {
                unsafe {
                    *object.activated() = true;
                }
            }
            for object in &self.activated_objects_p2 {
                unsafe {
                    *object.activated_p2() = true;
                }
            }

            self.apply_checkpoint(play_layer);
        } else {
            self.clear_checkpoints();
        }
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn set_auto_hold(&mut self, auto_hold: bool) {
        self.auto_hold = auto_hold;
    }
}
