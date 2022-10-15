use crate::replay::*;
use gd;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerData {
    number_of_clicks: u8,
    clicks: u8,
}

impl PlayerData {
    pub fn create(player: gd::PlayerObject, clicks: Vec<bool>) -> Self {
        assert!(clicks.len() <= 8);
        Self {
            number_of_clicks: clicks.len() as u8,
            clicks: Self::create_clicks(clicks),
        }
    }

    fn create_clicks(clicks: Vec<bool>) -> u8 {
        let mut result = 0;
        for (i, &click) in clicks.iter().enumerate() {
            if click {
                result |= 1 << i;
            }
        }
        result
    }

    pub fn apply(&self) {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FrameData {
    location: Location,
    player1: PlayerData,
    player2: PlayerData,
    fps: f32,
    time: f64,
}

impl FrameData {
    pub fn create(location: Location, fps: f32, play_layer: gd::PlayLayer) -> Self {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FullReplay {
    initial_fps: f32,
    current_fps: f32,
    replay_type: ReplayType,
    current_frame: u32,
    current_clicks: (Vec<bool>, Vec<bool>),
    frames: Vec<FrameData>,
}

impl Replay for FullReplay {
    fn new(fps: f32, replay_type: ReplayType) -> Self {
        Self {
            initial_fps: fps,
            current_fps: fps,
            replay_type,
            current_frame: 0,
            current_clicks: (Vec::new(), Vec::new()),
            frames: Vec::new(),
        }
    }

    fn add_click(&mut self, click: Click) {
        todo!()
    }

    fn insert_click(&mut self, index: usize, click: Click) {
        todo!()
    }

    fn change_fps(&mut self, location: Location, fps: f32) {
        todo!()
    }

    fn reset(&mut self, location: Location, wipe: bool) {
        todo!()
    }

    fn finalise(&mut self) {
        self.frames.shrink_to_fit();
    }

    fn for_all_current_clicks(&mut self, location: Location, f: fn(&Click)) {
        todo!()
    }

    fn get_current_click(&mut self, location: Location) -> Option<&mut Click> {
        todo!()
    }

    fn get_current_clicks(&mut self, location: Location) -> Vec<Click> {
        todo!()
    }

    fn get_last_click(&mut self, player2: bool) -> Option<&Click> {
        todo!()
    }

    fn serialise(&self) -> Result<Vec<u8>, String> {
        let data = bincode::serialize(&self);
        if let Ok(data) = data {
            Ok(data)
        } else {
            Err(data.unwrap_err().to_string())
        }
    }

    fn get_type(&self) -> ReplayType {
        self.replay_type
    }

    fn get_current_fps(&self) -> f32 {
        self.current_fps
    }

    fn is_done(&self) -> bool {
        todo!()
    }
}

impl FullReplay {
    pub fn deserialise(data: Vec<u8>) -> Result<Self, String> {
        let res = bincode::deserialize(&data);
        if let Err(res) = res {
            Err(res.to_string())
        } else {
            Ok(res.unwrap())
        }
    }
}
