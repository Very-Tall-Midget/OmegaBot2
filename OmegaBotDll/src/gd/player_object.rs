use crate::utils::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerObject {
    address: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
pub enum GameMode {
    Cube,
    Ship,
    UFO,
    Ball,
    Wave,
    Robot,
    Spider,
}

impl PlayerObject {
    pub fn from_address(address: usize) -> Self {
        PlayerObject { address }
    }

    pub fn get_position(&self) -> (*mut u32, *mut u32) {
        (
            read_value(self.address + 0x67C),
            read_value(self.address + 0x680),
        )
    }

    pub fn get_position_float(&self) -> (*mut f32, *mut f32) {
        (
            read_value(self.address + 0x67C),
            read_value(self.address + 0x680),
        )
    }

    pub fn get_x_accel(&self) -> *mut f64 {
        read_value(self.address + 0x518)
    }

    pub fn get_y_accel(&self) -> *mut f64 {
        read_value(self.address + 0x628)
    }

    pub fn get_jump_accel(&self) -> *mut f64 {
        read_value(self.address + 0x520)
    }

    pub fn get_is_holding(&self) -> *mut bool {
        read_value(self.address + 0x611)
    }

    pub fn get_has_just_held(&self) -> *mut bool {
        read_value(self.address + 0x612)
    }

    pub fn get_is_holding2(&self) -> *mut bool {
        read_value(self.address + 0x613)
    }

    pub fn get_has_just_held2(&self) -> *mut bool {
        read_value(self.address + 0x614)
    }

    pub fn get_can_robot_jump(&self) -> *mut bool {
        read_value(self.address + 0x624)
    }

    pub fn get_is_upside_down(&self) -> *mut bool {
        read_value(self.address + 0x63E)
    }

    pub fn get_is_on_ground(&self) -> *mut bool {
        read_value(self.address + 0x640)
    }

    pub fn get_is_dashing(&self) -> *mut bool {
        read_value(self.address + 0x641)
    }

    pub fn get_is_sliding(&self) -> *mut bool {
        read_value(self.address + 0x660)
    }

    pub fn get_is_rising(&self) -> *mut bool {
        read_value(self.address + 0x661)
    }

    pub fn get_black_orb(&self) -> *mut bool {
        read_value(self.address + 0x5FE)
    }

    pub fn get_unk662(&self) -> *mut bool {
        read_value(self.address + 0x662)
    }

    pub fn get_unk630(&self) -> *mut bool {
        read_value(self.address + 0x630)
    }

    pub fn get_unk631(&self) -> *mut bool {
        read_value(self.address + 0x631)
    }

    pub fn get_vehicle_size(&self) -> *mut f32 {
        read_value(self.address + 0x644)
    }

    pub fn get_player_speed(&self) -> *mut f32 {
        read_value(self.address + 0x648)
    }

    pub fn get_rotation_x(&self) -> *mut f32 {
        read_value(self.address + 0x20)
    }

    pub fn get_rotation_y(&self) -> *mut f32 {
        read_value(self.address + 0x24)
    }

    pub fn get_game_mode(&self) -> GameMode {
        unsafe {
            if *read_value(self.address + 0x638) {
                GameMode::Ship
            } else if *read_value(self.address + 0x639) {
                GameMode::UFO
            } else if *read_value(self.address + 0x63A) {
                GameMode::Ball
            } else if *read_value(self.address + 0x63B) {
                GameMode::Wave
            } else if *read_value(self.address + 0x63C) {
                GameMode::Robot
            } else if *read_value(self.address + 0x63D) {
                GameMode::Spider
            } else {
                GameMode::Cube
            }
        }
    }

    pub fn set_game_mode(&self, player_type: GameMode) {
        unsafe {
            *read_value(self.address + 0x638) = player_type == GameMode::Ship;
            *read_value(self.address + 0x639) = player_type == GameMode::UFO;
            *read_value(self.address + 0x63A) = player_type == GameMode::Ball;
            *read_value(self.address + 0x63B) = player_type == GameMode::Wave;
            *read_value(self.address + 0x63C) = player_type == GameMode::Robot;
            *read_value(self.address + 0x63D) = player_type == GameMode::Spider;
        }
    }
}

impl_is_null!(PlayerObject);

impl From<PlayerObject> for usize {
    fn from(player_object: PlayerObject) -> Self {
        player_object.address
    }
}

impl From<usize> for PlayerObject {
    fn from(address: usize) -> Self {
        PlayerObject { address }
    }
}
