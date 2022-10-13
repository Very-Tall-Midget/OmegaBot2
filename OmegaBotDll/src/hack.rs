use crate::patch::*;

pub struct Hack {
    pub patches: Vec<Patch>,
}

impl Hack {
    pub fn new(patches: Vec<Patch>) -> Self {
        Self { patches }
    }

    pub fn apply(&self) {
        for patch in &self.patches {
            patch.apply();
        }
    }

    pub fn restore(&self) {
        for patch in &self.patches {
            patch.restore();
        }
    }
}
