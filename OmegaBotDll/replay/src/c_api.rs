use crate::*;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub enum CReplayType {
    XPos,
    Frame,
}

impl From<ReplayType> for CReplayType {
    fn from(replay_type: ReplayType) -> Self {
        match replay_type {
            ReplayType::XPos => CReplayType::XPos,
            ReplayType::Frame => CReplayType::Frame,
        }
    }
}

impl From<CReplayType> for ReplayType {
    fn from(replay_type: CReplayType) -> Self {
        match replay_type {
            CReplayType::XPos => ReplayType::XPos,
            CReplayType::Frame => ReplayType::Frame,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CLocation {
    pub location_type: CReplayType,
    pub location: u32,
}

impl From<Location> for CLocation {
    fn from(location: Location) -> Self {
        match location {
            Location::XPos(x) => Self {
                location_type: CReplayType::XPos,
                location: x,
            },
            Location::Frame(f) => Self {
                location_type: CReplayType::Frame,
                location: f,
            },
        }
    }
}

impl From<CLocation> for Location {
    fn from(location: CLocation) -> Self {
        match location.location_type {
            CReplayType::XPos => Location::XPos(location.location),
            CReplayType::Frame => Location::Frame(location.location),
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub enum CClickType {
    None,
    FpsChange,
    Player1Down,
    Player1Up,
    Player2Down,
    Player2Up,
}

impl From<ClickType> for CClickType {
    fn from(click_type: ClickType) -> Self {
        match click_type {
            ClickType::None => CClickType::None,
            ClickType::FpsChange(_) => panic!("Cannot convert ClickType::FPSChange"),
            ClickType::Player1Down => CClickType::Player1Down,
            ClickType::Player1Up => CClickType::Player1Up,
            ClickType::Player2Down => CClickType::Player2Down,
            ClickType::Player2Up => CClickType::Player2Up,
        }
    }
}

impl From<CClickType> for ClickType {
    fn from(click_type: CClickType) -> Self {
        match click_type {
            CClickType::None => ClickType::None,
            CClickType::FpsChange => panic!("Cannot convert ClickType::FPSChange"),
            CClickType::Player1Down => ClickType::Player1Down,
            CClickType::Player1Up => ClickType::Player1Up,
            CClickType::Player2Down => ClickType::Player2Down,
            CClickType::Player2Up => ClickType::Player2Up,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CClick {
    pub location: CLocation,
    pub click_type: CClickType,
    pub fps: f32,
}

impl From<Click> for CClick {
    fn from(click: Click) -> Self {
        match click.click_type {
            ClickType::FpsChange(fps) => Self {
                location: click.location.into(),
                click_type: CClickType::FpsChange,
                fps,
            },
            _ => Self {
                location: click.location.into(),
                click_type: click.click_type.into(),
                fps: 0.0,
            },
        }
    }
}

impl From<CClick> for Click {
    fn from(click: CClick) -> Self {
        match click.click_type {
            CClickType::FpsChange => Click {
                location: click.location.into(),
                click_type: ClickType::FpsChange(click.fps),
            },
            _ => Click {
                location: click.location.into(),
                click_type: click.click_type.into(),
            },
        }
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct CReplay {
    pub initial_fps: f32,
    pub current_fps: f32,
    pub replay_type: CReplayType,
    pub current_click: usize,
    pub total_clicks: usize,
    pub clicks: *mut CClick,
}

impl From<Replay> for CReplay {
    fn from(replay: Replay) -> Self {
        let clicks: Vec<CClick> = replay
            .clicks
            .clone()
            .into_iter()
            .map(|c| c.into())
            .collect::<Vec<_>>();
        let clicks = clicks.into_boxed_slice();
        Self {
            initial_fps: replay.initial_fps,
            current_fps: replay.current_fps,
            replay_type: replay.replay_type.into(),
            current_click: replay.current_click,
            total_clicks: replay.clicks.len(),
            clicks: Box::into_raw(clicks) as *mut CClick,
        }
    }
}

impl From<CReplay> for Replay {
    fn from(replay: CReplay) -> Self {
        let clicks: Vec<Click> = unsafe {
            std::slice::from_raw_parts(replay.clicks, replay.total_clicks)
                .iter()
                .cloned()
                .map(|c| c.into())
                .collect::<Vec<_>>()
        };
        Self {
            initial_fps: replay.initial_fps,
            current_fps: replay.current_fps,
            replay_type: replay.replay_type.into(),
            current_click: replay.current_click,
            clicks,
        }
    }
}

// Yes this very very inefficient, but it doesn't matter as it will only be used by the macro editor
macro_rules! c_replay {
    ($self:ident as $replay:ident $code:block) => {
        #[allow(unused_mut)]
        let mut $replay: Replay = $self.clone().into();
        $code;
        *$self = $replay.into();
    };
}

impl CReplay {
    #[no_mangle]
    pub extern "C" fn print(&self) {
        println!("{:?}", self);
    }

    #[no_mangle]
    pub extern "C" fn create(fps: f32, replay_type: CReplayType) -> Self {
        Replay {
            initial_fps: fps,
            current_fps: fps,
            replay_type: replay_type.into(),
            current_click: 0,
            clicks: Vec::new(),
        }
        .into()
    }

    #[no_mangle]
    pub extern "C" fn free_clicks(&mut self) {
        unsafe {
            Box::from_raw(std::slice::from_raw_parts_mut(
                self.clicks,
                self.total_clicks,
            ));
        }
    }

    #[no_mangle]
    pub extern "C" fn add_click(&mut self, click: CClick) {
        c_replay!(self as replay {
            replay.clicks.push(click.clone().into());
        });
    }

    #[no_mangle]
    pub extern "C" fn insert_click(&mut self, index: usize, click: CClick) {
        c_replay!(self as replay {
            replay.clicks.insert(index, click.clone().into());
        });
    }

    #[no_mangle]
    pub extern "C" fn change_fps(&mut self, location: CLocation, fps: f32) {
        c_replay!(self as replay {
            replay.clicks.push(Click {
                location: location.into(),
                click_type: ClickType::FpsChange(fps),
            });
        });
    }

    #[no_mangle]
    pub extern "C" fn reset(&mut self, location: CLocation, wipe: bool) {
        c_replay!(self as replay {
            replay.reset(location.into(), wipe);
        });
    }

    #[no_mangle]
    pub extern "C" fn finalise(&mut self) {
        c_replay!(self as replay {
            replay.finalise();
        });
    }

    #[no_mangle]
    pub extern "C" fn for_all_current_clicks(
        &mut self,
        location: CLocation,
        f: extern "C" fn(&CClick),
    ) {
        c_replay!(self as replay {
            replay.for_all_current_clicks(location.into(), |c| f(&c.clone().into()));
        });
    }

    #[no_mangle]
    pub extern "C" fn get_current_click(
        &mut self,
        location: CLocation,
        found_click: &mut bool,
    ) -> CClick {
        let out;
        c_replay!(self as replay {
            let click = replay.get_current_click(location.into());
            *found_click = click.is_some();
            out = click.cloned().unwrap_or(Click { location: Location::XPos(0), click_type: ClickType::None }).clone().into();
        });
        out
    }

    #[no_mangle]
    pub extern "C" fn get_current_clicks(
        &mut self,
        location: CLocation,
        number_of_clicks: &mut usize,
    ) -> *mut CClick {
        let vec;
        c_replay!(self as replay {
            vec = replay.get_current_clicks(location.into());
        });
        *number_of_clicks = vec.len();
        Box::into_raw(vec.into_boxed_slice()) as *mut CClick
    }

    #[no_mangle]
    pub extern "C" fn get_last_click(&mut self, player2: bool, found_click: &mut bool) -> CClick {
        let out;
        c_replay!(self as replay {
            let click = replay.get_last_click(player2);
            *found_click = click.is_some();
            out = click.cloned().unwrap_or(Click {location: Location::XPos(0), click_type: ClickType::None}).clone().into();
        });
        out
    }

    #[no_mangle]
    pub extern "C" fn save(&mut self, filename: *const u16, filename_length: usize) -> bool {
        let mut success = false;
        c_replay!(self as replay {
            let res = replay.serialise();
            if let Ok(res) = res {
                success = std::fs::write(String::from_utf16(unsafe { std::slice::from_raw_parts(filename, filename_length) }).unwrap(), res).is_ok();
            }
        });
        success
    }

    #[no_mangle]
    pub extern "C" fn load(
        filename: *const u16,
        filename_length: usize,
        success: &mut bool,
    ) -> Self {
        let filename =
            String::from_utf16(unsafe { std::slice::from_raw_parts(filename, filename_length) })
                .unwrap();
        let mut data = std::fs::read(filename).unwrap();
        if *data.get(0).unwrap_or(&1) != 0 {
            *success = false;
        } else {
            data.remove(0);
            let res: Result<Replay, _> = bincode::deserialize(&data);
            *success = res.is_ok();
            if let Ok(replay) = res {
                return replay.into();
            }
        }

        Replay::new(0.0, ReplayType::XPos).into()
    }

    #[no_mangle]
    pub extern "C" fn is_done(&self) -> bool {
        self.current_click >= self.total_clicks
    }
}
