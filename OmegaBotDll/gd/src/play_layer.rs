use super::*;
use crate::utils::*;

#[derive(Debug, Clone, Copy)]
pub struct PlayLayer {
    address: usize,
}

impl PlayLayer {
    pub fn create(level: usize) -> Self {
        Self::from_address(unsafe {
            std::mem::transmute::<_, extern "fastcall" fn(usize) -> usize>(get_base() + 0x1FB6D0)(
                level,
            )
        })
    }

    pub fn from_address(address: usize) -> Self {
        PlayLayer { address }
    }

    pub fn is_dead(&self) -> bool {
        unsafe {
            *read_value(self.address + 0x39C)
                && self
                    .player1()
                    .if_not_null(|p| *p.get_position_float().0 != 0.0)
                    .unwrap_or(true)
        }
    }

    pub fn is_paused(&self) -> *mut bool {
        read_value(self.address + 0x42B)
    }

    pub fn time(&self) -> *mut f64 {
        read_value(self.address + 0x450)
    }

    pub fn set_player1(&self, player1: PlayerObject) {
        unsafe {
            *read_value::<usize>(self.address + 0x224) = player1.into();
        }
    }

    pub fn player1(&self) -> PlayerObject {
        PlayerObject::from_address(unsafe { read_ptr(self.address + 0x224) })
    }

    pub fn player2(&self) -> PlayerObject {
        PlayerObject::from_address(unsafe { read_ptr(self.address + 0x228) })
    }

    pub fn level_settings(&self) -> LevelSettings {
        LevelSettings::from_address(unsafe { read_ptr(self.address + 0x22C) })
    }

    pub fn is_practice_mode(&self) -> bool {
        unsafe { *read_value(self.address + 0x495) }
    }
}

impl_is_null!(PlayLayer);

impl From<PlayLayer> for usize {
    fn from(pl: PlayLayer) -> Self {
        pl.address
    }
}

impl From<usize> for PlayLayer {
    fn from(address: usize) -> Self {
        PlayLayer::from_address(address)
    }
}
