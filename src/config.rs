

use std::path::PathBuf;

use bitflags::bitflags;

use crate::options::BloatOption;

bitflags! {
    #[derive(Debug, Default, Clone, Copy)]
    struct ZicFlags: u8 {
        const VERBOSE = 0b0000_0001;
        const LOCALTIME = 0b0000_0010;
        const LEAP_SECONDS = 0b0000_0100;
        const POSIX_RULES = 0b0000_1000;
    }
}

#[derive(Debug, Clone, Default)]
pub struct Config {
    bloat: BloatOption,
    flags: ZicFlags,
    directory: PathBuf,
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

}

// ==== Config Builder API ====

impl Config {
    pub fn set_bloat(mut self, option: BloatOption) -> Self {
        self.bloat = option;
        self
    }

    pub fn set_verbose(mut self, value: bool) -> Self {
        self.flags.set(ZicFlags::VERBOSE, value);
        self
    }

    pub fn set_localtime_flag(mut self, value: bool) -> Self {
        self.flags.set(ZicFlags::LOCALTIME, value);
        self
    }

    pub fn set_posixrules_flag(mut self, value: bool) -> Self {
        self.flags.set(ZicFlags::POSIX_RULES, value);
        self
    }

    pub fn set_leap_seconds_flag(mut self, value: bool) -> Self {
        self.flags.set(ZicFlags::LEAP_SECONDS, value);
        self
    }

    pub fn directory(mut self, path: PathBuf) -> Self {
        self.directory = path;
        self
    }
}

