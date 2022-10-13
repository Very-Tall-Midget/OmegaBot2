use crate::utils;

pub struct Patch {
    pub address: usize,
    pub new_bytes: Vec<u8>,
    pub original_bytes: Vec<u8>,
}

impl Patch {
    pub fn new(address: usize, new_bytes: Vec<u8>, original_bytes: Vec<u8>) -> Self {
        assert_eq!(new_bytes.len(), original_bytes.len());
        Self {
            address,
            new_bytes,
            original_bytes,
        }
    }

    pub fn apply(&self) {
        utils::patch(self.address, self.new_bytes.clone());
    }

    pub fn restore(&self) {
        utils::patch(self.address, self.original_bytes.clone());
    }
}
