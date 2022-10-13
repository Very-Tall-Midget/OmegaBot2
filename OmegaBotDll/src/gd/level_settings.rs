use crate::utils::*;

pub struct LevelSettings {
    address: usize,
}

impl LevelSettings {
    pub fn from_address(address: usize) -> Self {
        LevelSettings { address }
    }

    pub fn is_2player(&self) -> bool {
        unsafe { *read_value(self.address + 0xFA) }
    }
}

impl_is_null!(LevelSettings);

impl From<usize> for LevelSettings {
    fn from(address: usize) -> Self {
        LevelSettings::from_address(address)
    }
}
