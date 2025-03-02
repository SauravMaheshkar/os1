use alloc::{borrow::Cow, string::String};

use hashbrown::HashMap;

use crate::{
    interrupts::guard::InterruptLock, util::circular_buffer::CircularBuffer,
};

#[allow(unused_macros)]
macro_rules! log {
    ($verbosity: expr, $s: expr) => {
        if $crate::logger::LOGGER.get_verbosity(module_path!()) <= $verbosity {
            let log = $crate::logger::Log {
                file: file!(),
                line: line!(),
                verbosity: $verbosity,
                message: $s.into()
            };
            $crate::logger::LOGGER.push_log(log);
        }
    };
    ($verbosity: expr, $s: expr $(, $args: expr)*) => {
        if $crate::logger::LOGGER.get_verbosity(module_path!()) <= $verbosity {
            let log = $crate::logger::Log {
                file: file!(),
                line: line!(),
                verbosity: $verbosity,
                message: alloc::format!($s $(, $args)*).into()
            };
            $crate::logger::LOGGER.push_log(log);
        }
    };
}

#[allow(unused_macros)]
macro_rules! debug {
    ($s: expr) => {
        log!($crate::logger::Verbosity::Debug, $s)
    };
    ($s: expr $(, $args: expr)*) => {
        log!($crate::logger::Verbosity::Debug, $s $(, $args)*)
    };
}

#[allow(unused_macros)]
macro_rules! info {
    ($s: expr) => {
        log!($crate::logger::Verbosity::Info, $s)
    };
    ($s: expr $(, $args: expr)*) => {
        log!($crate::logger::Verbosity::Info, $s $(, $args)*)
    };
}

#[allow(unused_macros)]
macro_rules! warn {
    ($s: expr) => {
        log!($crate::logger::Verbosity::Warning, $s)
    };
    ($s: expr $(, $args: expr)*) => {
        log!($crate::logger::Verbosity::Warning, $s $(, $args)*)
    };
}

#[allow(unused_macros)]
macro_rules! error {
    ($s: expr) => {
        log!($crate::logger::Verbosity::Error, $s)
    };
    ($s: expr $(, $args: expr)*) => {
        log!($crate::logger::Verbosity::Error, $s $(, $args)*)
    };
}

pub static LOGGER: Logger = Logger::new();

pub struct Log {
    pub file: &'static str,
    pub line: u32,
    pub verbosity: Verbosity,
    pub message: Cow<'static, str>,
}

impl core::fmt::Display for Log {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!(
            "[{}] {}:{} {}",
            self.verbosity, self.file, self.line, self.message
        ))?;

        Ok(())
    }
}

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
#[allow(unused)]
pub enum Verbosity {
    Debug,
    Info,
    Warning,
    Error,
}

impl core::fmt::Display for Verbosity {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Debug => f.write_str("DEBUG"),
            Self::Info => f.write_str("INFO"),
            Self::Warning => f.write_str("WARNING"),
            Self::Error => f.write_str("ERROR"),
        }?;
        Ok(())
    }
}

pub struct Logger {
    verbose: InterruptLock<Option<HashMap<String, Verbosity>>>,
    logs: InterruptLock<CircularBuffer<Log, 1024>>,
}

impl Logger {
    pub const fn new() -> Self {
        Logger {
            verbose: InterruptLock::new(None),
            logs: InterruptLock::new(CircularBuffer::new()),
        }
    }

    pub fn get_verbosity(&self, module: &str) -> Verbosity {
        *self
            .verbose
            .lock()
            .as_ref()
            .expect("Logger not initialized")
            .get(module)
            .unwrap_or(&Verbosity::Info)
    }

    pub fn push_log(&self, log: Log) {
        if self.logs.lock().push_back(log).is_err() {
            panic!("Dropped log");
        }
    }

    pub fn service(&self) {
        while let Some(v) = self.logs.lock().pop_front() {
            println_serial!("{}", v);
        }
    }
}

pub fn init(verbose: HashMap<String, Verbosity>) {
    *LOGGER.verbose.lock() = Some(verbose);
}

pub fn service() {
    LOGGER.service()
}
