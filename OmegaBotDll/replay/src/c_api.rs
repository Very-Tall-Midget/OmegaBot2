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

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CStandardReplay {
    pub initial_fps: f32,
    pub current_fps: f32,
    pub replay_type: CReplayType,
    pub current_click: usize,
    pub total_clicks: usize,
    pub clicks: *mut CClick,
}

impl From<StandardReplay> for CStandardReplay {
    fn from(replay: StandardReplay) -> Self {
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

impl From<CStandardReplay> for StandardReplay {
    fn from(replay: CStandardReplay) -> Self {
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
        let mut $replay: StandardReplay = $self.clone().into();
        $code;
        *$self = $replay.into();
    };
}

impl CStandardReplay {
    #[no_mangle]
    pub extern "C" fn print(&self) {
        println!("{:?}", self);
    }

    #[no_mangle]
    pub extern "C" fn add_click(&mut self, click: &CClick) {
        c_replay!(self as replay {
            replay.add_click(click.clone().into());
        });
    }
}
