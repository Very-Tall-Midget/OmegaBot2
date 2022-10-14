use crate::replay::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct StandardReplay {
    initial_fps: f32,
    current_fps: f32,
    replay_type: ReplayType,
    current_click: usize,
    clicks: Vec<Click>,
}

#[allow(dead_code)]
impl Replay for StandardReplay {
    fn new(fps: f32, replay_type: ReplayType) -> Self {
        Self {
            initial_fps: fps,
            current_fps: fps,
            replay_type,
            current_click: 0,
            clicks: Vec::new(),
        }
    }

    fn add_click(&mut self, click: Click) {
        assert!(
            match click.location {
                Location::XPos(_) => self.replay_type == ReplayType::XPos,
                Location::Frame(_) => self.replay_type == ReplayType::Frame,
            },
            "Click location does not match replay type"
        );
        self.clicks.push(click);
    }

    fn insert_click(&mut self, index: usize, click: Click) {
        assert!(
            match click.location {
                Location::XPos(_) => self.replay_type == ReplayType::XPos,
                Location::Frame(_) => self.replay_type == ReplayType::Frame,
            },
            "Click location does not match replay type"
        );
        self.clicks.insert(index, click);
    }

    fn change_fps(&mut self, location: Location, fps: f32) {
        if self.current_fps != fps {
            self.current_fps = fps;
            self.clicks.push(Click {
                location,
                click_type: ClickType::FpsChange(fps),
            });
        }
    }

    fn reset(&mut self, location: Location, wipe: bool) {
        assert!(
            match location {
                Location::XPos(_) => self.replay_type == ReplayType::XPos,
                Location::Frame(_) => self.replay_type == ReplayType::Frame,
            },
            "Reset location does not match replay type"
        );
        if wipe {
            self.clicks = self
                .clicks
                .iter()
                .cloned()
                .filter(|c| c.location < location)
                .collect();
            self.current_fps = self.initial_fps;
            if !self.clicks.is_empty() {
                for click in self.clicks.iter().rev() {
                    if let ClickType::FpsChange(fps) = click.click_type {
                        self.current_fps = fps;
                        break;
                    }
                }
            }
        } else if self.clicks.is_empty()
            || match location {
                Location::XPos(x) => x == 0,
                Location::Frame(f) => f == 0,
            }
        {
            self.current_click = 0;
            self.current_fps = self.initial_fps;
        } else {
            while self.current_click > 0 && self.clicks[self.current_click].location >= location {
                self.current_click -= 1;
            }
            self.current_fps = self.initial_fps;
            for click in self
                .clicks
                .iter()
                .rev()
                .skip(self.clicks.len() - self.current_click)
            {
                if let ClickType::FpsChange(fps) = click.click_type {
                    self.current_fps = fps;
                    break;
                }
            }
        }
    }

    fn finalise(&mut self) {
        self.clicks.shrink_to_fit();
    }

    fn for_all_current_clicks(&mut self, location: Location, f: fn(&Click)) {
        let mut click_opt = self.get_current_click(location);
        while let Some(click) = click_opt {
            f(click);
            click_opt = self.get_current_click(location);
        }
    }

    fn get_current_click(&mut self, location: Location) -> Option<&mut Click> {
        if self.current_click >= self.clicks.len() {
            None
        } else if self.clicks[self.current_click].location <= location {
            let click = self.clicks.get_mut(self.current_click).unwrap();
            self.current_click += 1;
            if let ClickType::FpsChange(fps) = click.click_type {
                self.current_fps = fps;
            }
            Some(click)
        } else {
            None
        }
    }

    fn get_current_clicks(&mut self, location: Location) -> Vec<Click> {
        let mut res = Vec::new();

        let mut click_opt = self.get_current_click(location);
        while let Some(click) = click_opt {
            res.push(click.clone());
            click_opt = self.get_current_click(location);
        }

        res
    }

    fn get_last_click(&mut self, player2: bool) -> Option<&Click> {
        for click in self
            .clicks
            .iter()
            .rev()
            .skip(self.clicks.len() - self.current_click)
        {
            match click.click_type {
                ClickType::Player1Down | ClickType::Player1Up if !player2 => return Some(click),
                ClickType::Player2Down | ClickType::Player2Up if player2 => return Some(click),
                _ => {}
            }
        }
        None
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
        self.current_click >= self.clicks.len()
    }
}

#[allow(dead_code)]
impl StandardReplay {
    fn get_click(&mut self, position: usize) -> Option<&Click> {
        self.clicks.get(position)
    }

    pub fn move_click_up(&mut self, position: usize) {
        self.clicks.swap(position, position + 1)
    }

    pub fn move_click_down(&mut self, position: usize) {
        self.clicks.swap(position, position - 1)
    }

    pub fn delete_click(&mut self, position: usize) {
        self.clicks.remove(position);
    }

    pub fn sort(&mut self) {
        self.clicks.sort_by(|a, b| a.location.cmp(&b.location));
    }

    pub fn merge(&mut self, other: &StandardReplay, force_player2: bool) {
        assert_eq!(self.replay_type, other.replay_type);
        assert_eq!(self.initial_fps, other.initial_fps);
        self.clicks
            .extend(other.clicks.iter().filter_map(|c| match c.click_type {
                ClickType::Player1Down => {
                    if force_player2 {
                        Some(Click {
                            location: c.location,
                            click_type: ClickType::Player2Down,
                        })
                    } else {
                        Some(c.clone())
                    }
                }
                ClickType::Player1Up => {
                    if force_player2 {
                        Some(Click {
                            location: c.location,
                            click_type: ClickType::Player2Up,
                        })
                    } else {
                        Some(c.clone())
                    }
                }
                ClickType::None => None,
                _ => Some(c.clone()),
            }));
        self.sort();
        self.finalise();
    }

    pub fn clean(&mut self) {
        let mut last_click_type = None;
        let mut new_clicks = Vec::new();
        for click in &self.clicks {
            match click.click_type {
                ClickType::None => {}
                ClickType::FpsChange(_) => new_clicks.push(click.clone()),
                _ => {
                    if last_click_type.is_none() {
                        last_click_type = Some(click.click_type);
                        new_clicks.push(click.clone());
                    } else if let Some(click_type) = last_click_type {
                        if click_type != click.click_type {
                            new_clicks.push(click.clone());
                            last_click_type = Some(click.click_type);
                        }
                    }
                }
            }
        }
        self.clicks = new_clicks;
        self.finalise();
    }

    pub fn deserialise(data: Vec<u8>) -> Result<Self, String> {
        let res = bincode::deserialize(&data);
        if let Err(res) = res {
            Err(res.to_string())
        } else {
            Ok(res.unwrap())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        let mut replay = StandardReplay::new(60., ReplayType::Frame);
        replay.add_click(Click {
            location: Location::Frame(20),
            click_type: ClickType::Player1Down,
        });
        replay.add_click(Click {
            location: Location::Frame(40),
            click_type: ClickType::Player1Up,
        });
        replay.add_click(Click {
            location: Location::Frame(60),
            click_type: ClickType::Player1Down,
        });
        replay.add_click(Click {
            location: Location::Frame(80),
            click_type: ClickType::Player1Up,
        });
        assert_eq!(replay.clicks.len(), 4);

        println!("Before insert:\n{:#?}", replay);
        replay.insert_click(
            2,
            Click {
                location: Location::Frame(100),
                click_type: ClickType::Player1Down,
            },
        );
        replay.insert_click(
            3,
            Click {
                location: Location::Frame(110),
                click_type: ClickType::Player1Up,
            },
        );
        println!("\nAfter insert:\n{:#?}", replay);
        assert_eq!(replay.clicks.len(), 6);

        replay.sort();
        println!("\nSorted:\n{:#?}", replay);
        assert_eq!(replay.clicks.last().unwrap().location, Location::Frame(110));

        replay.reset(Location::Frame(90), true);
        println!("\nReset from frame 90, recording mode:\n{:#?}", replay);
        assert_eq!(replay.clicks.len(), 4);

        std::fs::write("replay.replay", replay.serialise().unwrap()).unwrap();
        let new_replay =
            StandardReplay::deserialise(std::fs::read("replay.replay").unwrap()).unwrap();

        println!("\nAfter save/load:\n{:#?}", new_replay);
        assert_eq!(replay, new_replay);

        std::fs::remove_file("replay.replay").unwrap();
    }
}
