use std::sync::{LazyLock, Mutex};

static LOG_LIST: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(Vec::new()));

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum LogColor {
    Black = 0,
    Red = 1,
    Green = 2,
    Yellow = 3,
    Blue = 4,
    Purple = 5,
    Cyan = 6,
    White = 7
}

#[derive(Copy, Clone)]
pub enum LogType {
    Info,
    Warning,
    Error
}

fn match_ansi_color(color: LogColor) -> &'static str {
    match color {
        LogColor::Black => "\x1b[0;30m",
        LogColor::Red => "\x1b[0;31m",
        LogColor::Green => "\x1b[0;32m",
        LogColor::Yellow => "\x1b[0;33m",
        LogColor::Blue => "\x1b[0;34m",
        LogColor::Purple => "\x1b[0;35m",
        LogColor::Cyan => "\x1b[0;36m",
        LogColor::White => "\x1b[0;37m",
    }
}

fn match_log_type(log_type: LogType) -> &'static str {
    match log_type {
        LogType::Info => "[INFO]",
        LogType::Warning => "[WARNING]",
        LogType::Error => "[ERROR]"
    }
}

fn match_log_type_color(log_type: LogType) -> &'static str {
    match log_type {
        LogType::Info => "\x1b[0;32m",
        LogType::Warning => "\x1b[0;33m",
        LogType::Error => "\x1b[0;31m"
    }
}

fn reset_log_color() {
    print!("{}", "\x1b[0m");
}

fn emit_log_type(log_type: LogType) {
    print!("{}", match_log_type_color(log_type));
    print!("{}", match_log_type(log_type));
    reset_log_color();
}

pub fn log(message: impl std::fmt::Display, color: LogColor, log_type: LogType) {
    let mut logs = LOG_LIST.lock().unwrap();
    let log = match_log_type(log_type).to_string() + " " + &message.to_string();
    logs.push(log);

    emit_log_type(log_type);
    print!("{}", match_ansi_color(color));
    println!(" {}", message);
    reset_log_color();
}

#[macro_export]
macro_rules! gemulog {
    ($color:expr, $log_type:expr, $($arg:tt)*) => {
        logger::log(
            $color,
            $log_type,
            format_args!($($arg)*),
        );
    };
}

#[macro_export]
macro_rules! gemuinfo {
    ($($arg:tt)*) => {
        $crate::logger::log(
            format_args!($($arg)*),
            $crate::logger::LogColor::Green,
            $crate::logger::LogType::Info
        )
    };

    ($color:expr, $($arg:tt)*) => {
        $crate::logger::log(
            format_args!($($arg)*),
            $color,
            $crate::logger::LogType::Info,
        );
    };
}

#[macro_export]
macro_rules! gemuwarning {
    ($($arg:tt)*) => {
        $crate::logger::log(
            format_args!($($arg)*),
            $crate::logger::LogColor::Yellow,
            $crate::logger::LogType::Warning,
        );
    };

    ($color:expr, $($arg:tt)*) => {
        $crate::logger::log(
            format_args!($($arg)*),
            $color,
            $crate::logger::LogType::Warning,
        );
    };
}

#[macro_export]
macro_rules! gemuerror {
    ($($arg:tt)*) => {
        $crate::logger::log(
            format_args!($($arg)*),
            $crate::logger::LogColor::Red,
            $crate::logger::LogType::Error,
        );
    };

    ($color:expr, $($arg:tt)*) => {
        $crate::logger::log(
            format_args!($($arg)*),
            $color,
            $crate::logger::LogType::Error,
        );
    };
}