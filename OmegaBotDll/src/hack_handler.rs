use std::collections::HashMap;

use crate::hack::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum HackName {
    NoSpike,
    PracticeMusicFix,
    AntiCheatBypass,
    NoRespawnFlash,
    IgnoreEscape,
    DisableDeathEffect,
}

impl From<HackName> for u16 {
    fn from(hack_name: HackName) -> u16 {
        match hack_name {
            HackName::NoSpike => 1,
            HackName::PracticeMusicFix => 2,
            HackName::AntiCheatBypass => 3,
            HackName::NoRespawnFlash => 4,
            HackName::IgnoreEscape => 5,
            HackName::DisableDeathEffect => 6,
        }
    }
}

impl From<u16> for HackName {
    fn from(val: u16) -> Self {
        match val {
            1 => HackName::NoSpike,
            2 => HackName::PracticeMusicFix,
            3 => HackName::AntiCheatBypass,
            4 => HackName::NoRespawnFlash,
            5 => HackName::IgnoreEscape,
            6 => HackName::DisableDeathEffect,
            _ => panic!("Unknown hack: {}", val),
        }
    }
}

#[derive(Default)]
pub struct HackHandler {
    hacks: HashMap<HackName, Hack>,
}

impl HackHandler {
    pub fn add_hack(&mut self, name: HackName, hack: Hack) {
        self.hacks.insert(name, hack);
    }

    pub fn apply(&self, name: HackName) {
        self.hacks.get(&name).unwrap().apply();
    }

    pub fn restore(&self, name: HackName) {
        self.hacks.get(&name).unwrap().restore();
    }
}

#[macro_export]
macro_rules! create_hack (
    ($({ $addr:expr, { $($new_bytes:literal),* }, { $($original_bytes:literal),* } }),+ $(,)?) => {
        $crate::hack::Hack::new(vec![$($crate::patch::Patch::new($addr, vec![$($new_bytes),*], vec![$($original_bytes),*])),+])
    }
);
