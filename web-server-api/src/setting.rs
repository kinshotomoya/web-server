use config::{Config, ConfigBuilder, ConfigError, File, FileFormat, FileSourceFile};
use serde::{Deserialize, Serialize};
use tracing::Level;

#[derive(Deserialize, Serialize, Debug)]
pub struct Settings {
    pub log: Log,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Log {
    pub level: String
}

impl Log {
    pub fn find_trace_log_level(&self) -> Level {
        // impl From<&str> for LogLevelでintoメソッドを実装している
        match self.level.as_str().into() {
            LogLevel::Error => Level::ERROR,
            LogLevel::Warning => Level::WARN,
            LogLevel::Info => Level::INFO,
            LogLevel::Debug => Level::DEBUG,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub enum LogLevel {
    Error,
    Warning,
    Info,
    Debug,
}

impl From<LogLevel> for &str {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Error => "error",
            LogLevel::Warning => "warning",
            LogLevel::Info => "info",
            LogLevel::Debug => "debug",
        }
    }
}

impl From<&str> for LogLevel{
    fn from(log_level: &str) -> Self {
        match log_level {
            "error" => LogLevel::Info,
            "warning" => LogLevel::Warning,
            "info" => LogLevel::Info,
            "debug" => LogLevel::Debug,
            _ => LogLevel::Error
        }
    }
}

// 参考: https://docs.rs/config/latest/config/builder/struct.ConfigBuilder.html
impl Settings {
    pub fn new(env: String) -> Result<Self, ConfigError>{
        let mut builder = Config::builder();
        let config = builder.add_source(File::new(&format!("src/config/{}.toml", env), FileFormat::Toml)).build();
        // メソッドの戻り方がSettingsになっているので、deserializeを指定しなくても良い
        config?.try_deserialize()
    }
}
