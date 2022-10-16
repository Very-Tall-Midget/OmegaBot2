use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Location {
    XPos(u32),
    Frame(u32),
}

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Location::XPos(x), Location::XPos(y)) => x == y,
            (Location::Frame(x), Location::Frame(y)) => x == y,
            _ => false,
        }
    }
}

impl Eq for Location {}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Location::XPos(x), Location::XPos(y)) => x.partial_cmp(y),
            (Location::Frame(x), Location::Frame(y)) => x.partial_cmp(y),
            _ => None,
        }
    }
}

impl Ord for Location {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other)
            .expect("Cannot compare locations of different types")
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum ClickType {
    None,
    FpsChange(f32),
    Player1Down,
    Player1Up,
    Player2Down,
    Player2Up,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Click {
    pub location: Location,
    pub click_type: ClickType,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayType {
    XPos,
    Frame,
}

pub trait Replay {
    fn new(fps: f32, replay_type: ReplayType) -> Self
    where
        Self: Sized;

    fn add_click(&mut self, click: Click);
    fn insert_click(&mut self, index: usize, click: Click);
    fn change_fps(&mut self, location: Location, fps: f32);

    fn reset(&mut self, location: Location, wipe: bool);
    fn finalise(&mut self);

    fn get_current_click(&mut self, location: Location) -> Option<&mut Click>;
    fn get_current_clicks(&mut self, location: Location) -> Vec<Click>;
    fn get_last_click(&mut self, player2: bool) -> Option<&Click>;

    fn serialise(&self) -> Result<Vec<u8>, String>;

    fn get_type(&self) -> ReplayType;
    fn get_current_fps(&self) -> f32;
    fn is_done(&self) -> bool;
}
