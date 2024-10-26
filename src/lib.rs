
pub mod config;
pub mod options;

use config::Config;

#[derive(Debug, Clone)]
pub struct ZoneInfoCompiler {
    config: Config,
}

impl ZoneInfoCompiler {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn compile(file: &[u8]) {
        todo!();
    }
}
