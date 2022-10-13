pub struct MutexCount {
    lock: usize,
    active: bool,
}

impl Default for MutexCount {
    fn default() -> Self {
        Self {
            lock: 0,
            active: true,
        }
    }
}

impl MutexCount {
    pub fn wait(&self) -> bool {
        !self.active || self.lock == 0 // only allow thread if not active or no locks

        // while self.active && self.lock > 0 {}
        // true
    }

    pub fn lock(&mut self) {
        if self.active {
            self.lock += 1;
        }
    }

    pub fn unlock(&mut self) {
        if self.active && self.lock != 0 {
            self.lock -= 1;
        }
    }

    pub fn clear(&mut self) {
        self.lock = 0;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }
}
