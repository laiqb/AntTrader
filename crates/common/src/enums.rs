use log::Level;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString, FromRepr};

#[repr(C)]
#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Display,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    FromRepr,
    EnumIter,
    EnumString,
    Serialize,
    Deserialize,
)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(
    feature = "python",
    pyo3::pyclass(eq, eq_int, module = "ant_trader.core.ant_pyo3.common.enums")
)]
pub enum ComponentState{
    #[default]
    PreInitialized = 0,

    Ready = 1,

    Starting = 2,

    Running = 3,

    Stopping = 4,

    Stopped = 5,

    Resuming = 6,

    Resetting = 7,

    Disposing = 8,

    Disposed = 9,

    Degrading = 10,

    Degraded = 11,

    Faulting = 12,

    Faulted = 13,
}

impl ComponentState{
    pub fn variant_name(&self) -> String{
        let s = self.to_string();
        format!("{}{}", s[0..1].to_uppercase(), s[1..].to_lowercase())
    }
}

#[repr(C)]
#[derive(
    Copy,
    Clone,
    Debug,
    Display,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    FromRepr,
    EnumIter,
    EnumString,
    Serialize,
    Deserialize,
)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(
    feature = "python",
    pyo3::pyclass(eq, eq_int, module = "ant_trader.core.ant_pyo3.common.enums")
)]
pub enum ComponentTrigger{
    Initialize = 1,
    Start = 2,
    StartCompleted = 3,
    Stop = 4,
    StopCompleted = 5,
    Resume = 6,
    ResumeCompleted = 7,
    Reset = 8,
    ResetCompleted = 9,
    Dispose = 10,
    DisposeCompleted = 11,
    Degrade = 12,
    DegradeCompleted = 13,
    Fault = 14,
    FaultCompleted = 15,
}

#[repr(C)]
#[derive(
    Copy,
    Clone,
    Debug,
    Display,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    FromRepr,
    EnumIter,
    EnumString,
    Serialize,
    Deserialize,
)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(
    feature = "python",
    pyo3::pyclass(eq, eq_int, module = "ant_trader.core.ant_pyo3.common.enums")
)]
pub enum Environment{
    Backtest,
    Sandbox,
    Live,
}

#[repr(C)]
#[derive(
    Copy,
    Clone,
    Debug,
    Display,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    FromRepr,
    EnumIter,
    EnumString,
    Serialize,
    Deserialize,
)]
#[strum(ascii_case_insensitive)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(
    feature = "python",
    pyo3::pyclass(eq, eq_int, module = "ant_trader.core.ant_pyo3.common.enums")
)]
pub enum LogLevel{
    /// The **OFF** log level. A level lower than all other log levels (off).
    #[strum(serialize = "OFF")]
    #[serde(rename = "OFF")]
    Off = 0,
    /// The **TRACE** log level. Only available in Rust for debug/development builds.
    #[strum(serialize = "TRACE")]
    #[serde(rename = "TRACE")]
    Trace = 1,
    /// The **DEBUG** log level.
    #[strum(serialize = "DEBUG")]
    #[serde(rename = "DEBUG")]
    Debug = 2,
    /// The **INFO** log level.
    #[strum(serialize = "INFO")]
    #[serde(rename = "INFO")]
    Info = 3,
    /// The **WARNING** log level.
    #[strum(serialize = "WARN", serialize = "WARNING")]
    #[serde(rename = "WARNING")]
    Warning = 4,
    /// The **ERROR** log level.
    #[strum(serialize = "ERROR")]
    #[serde(rename = "ERROR")]
    Error = 5,
}

#[repr(C)]
#[derive(
    Copy,
    Clone,
    Debug,
    Display,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    FromRepr,
    EnumIter,
    EnumString,
    Serialize,
    Deserialize,
)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(
    feature = "python",
    pyo3::pyclass(eq, eq_int, module = "ant_trader.core.ant_pyo3.common.enums")
)]
pub enum LogColor {
    /// The default/normal log color.
    #[strum(serialize = "NORMAL")]
    Normal = 0,
    /// The green log color, typically used with [`LogLevel::Info`] log levels and associated with success events.
    #[strum(serialize = "GREEN")]
    Green = 1,
    /// The blue log color, typically used with [`LogLevel::Info`] log levels and associated with user actions.
    #[strum(serialize = "BLUE")]
    Blue = 2,
    /// The magenta log color, typically used with [`LogLevel::Info`] log levels.
    #[strum(serialize = "MAGENTA")]
    Magenta = 3,
    /// The cyan log color, typically used with [`LogLevel::Info`] log levels.
    #[strum(serialize = "CYAN")]
    Cyan = 4,
    /// The yellow log color, typically used with [`LogLevel::Warning`] log levels.
    #[strum(serialize = "YELLOW")]
    Yellow = 5,
    /// The red log color, typically used with [`LogLevel::Error`] level.
    #[strum(serialize = "RED")]
    Red = 6,
}

impl LogColor {
    #[must_use]
    pub const fn as_ansi(&self) -> &str {
        match *self {
            Self::Normal => "",
            Self::Green => "\x1b[92m",
            Self::Blue => "\x1b[94m",
            Self::Magenta => "\x1b[35m",
            Self::Cyan => "\x1b[36m",
            Self::Yellow => "\x1b[1;33m",
            Self::Red => "\x1b[1;31m",
        }
    }
}

impl From<u8> for LogColor {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Green,
            2 => Self::Blue,
            3 => Self::Magenta,
            4 => Self::Cyan,
            5 => Self::Yellow,
            6 => Self::Red,
            _ => Self::Normal,
        }
    }
}

impl From<Level> for LogColor {
    fn from(value: Level) -> Self {
        match value {
            Level::Error => Self::Red,
            Level::Warn => Self::Yellow,
            Level::Info => Self::Normal,
            Level::Debug => Self::Normal,
            Level::Trace => Self::Normal,
        }
    }
}

/// An ANSI log line format specifier.
/// This is used for formatting log messages with ANSI escape codes.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, FromRepr, EnumString, Display)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(
    feature = "python",
    pyo3::pyclass(eq, eq_int, module = "ant_trader.core.ant_pyo3.common.enums")
)]
pub enum LogFormat {
    /// Header log format. This ANSI escape code is used for magenta text color,
    /// often used for headers or titles in the log output.
    #[strum(serialize = "\x1b[95m")]
    Header,

    /// Endc log format. This ANSI escape code is used to reset all format attributes
    /// to their defaults. It should be used after applying other formats.
    #[strum(serialize = "\x1b[0m")]
    Endc,

    /// Bold log format. This ANSI escape code is used to make the text bold in the log output.
    #[strum(serialize = "\x1b[1m")]
    Bold,

    /// Underline log format. This ANSI escape code is used to underline the text in the log output.
    #[strum(serialize = "\x1b[4m")]
    Underline,
}

/// The serialization encoding.
#[repr(C)]
#[derive(
    Copy,
    Clone,
    Debug,
    Display,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    FromRepr,
    EnumIter,
    EnumString,
    Serialize,
    Deserialize,
)]
#[strum(ascii_case_insensitive)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(
    feature = "python",
    pyo3::pyclass(eq, eq_int, module = "ant_trader.core.ant_pyo3.common.enums")
)]
pub enum SerializationEncoding {
    /// The MessagePack encoding.
    #[serde(rename = "msgpack")]
    MsgPack = 0,
    /// The JavaScript Object Notation (JSON) encoding.
    #[serde(rename = "json")]
    Json = 1,
}