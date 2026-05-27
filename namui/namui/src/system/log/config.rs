use std::path::PathBuf;
use tracing::Level;

pub struct LogConfig {
    pub(crate) level: Level,
    pub(crate) filter: Option<String>,
    pub(crate) file_output: Option<PathBuf>,
    pub(crate) app_name: Option<String>,
    pub(crate) in_game_console: bool,
    pub(crate) ring_buffer_capacity: usize,
    pub(crate) ansi: Option<bool>,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: Level::INFO,
            filter: None,
            file_output: None,
            app_name: None,
            in_game_console: false,
            ring_buffer_capacity: 1024,
            ansi: None,
        }
    }
}

impl LogConfig {
    pub fn builder() -> LogConfigBuilder {
        LogConfigBuilder::default()
    }
}

#[derive(Default)]
pub struct LogConfigBuilder {
    inner: LogConfig,
}

impl LogConfigBuilder {
    pub fn level(mut self, level: Level) -> Self {
        self.inner.level = level;
        self
    }

    pub fn filter(mut self, filter: impl Into<String>) -> Self {
        self.inner.filter = Some(filter.into());
        self
    }

    pub fn file_output(mut self, path: impl Into<PathBuf>) -> Self {
        self.inner.file_output = Some(path.into());
        self
    }

    pub fn app_name(mut self, name: impl Into<String>) -> Self {
        self.inner.app_name = Some(name.into());
        self
    }

    pub fn in_game_console(mut self, enabled: bool) -> Self {
        self.inner.in_game_console = enabled;
        self
    }

    pub fn ring_buffer_capacity(mut self, capacity: usize) -> Self {
        self.inner.ring_buffer_capacity = capacity;
        self
    }

    pub fn ansi(mut self, enabled: bool) -> Self {
        self.inner.ansi = Some(enabled);
        self
    }

    pub fn build(self) -> LogConfig {
        self.inner
    }
}
