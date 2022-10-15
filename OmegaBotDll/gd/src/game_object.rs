use crate::utils::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameObject {
    address: usize,
}

impl GameObject {
    pub fn from_address(address: usize) -> Self {
        GameObject { address }
    }

    pub fn activated(&self) -> *mut bool {
        read_value(self.address + 0x2CA)
    }

    pub fn activated_p2(&self) -> *mut bool {
        read_value(self.address + 0x2CB)
    }
}

impl_is_null!(GameObject);

impl From<usize> for GameObject {
    fn from(address: usize) -> Self {
        GameObject::from_address(address)
    }
}
