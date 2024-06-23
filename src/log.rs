use std::{
    fmt::Display,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
    str::FromStr,
    sync::RwLock,
};

/// This is the primary program logger. This is the one without a prefix that
/// is accessed and used by the normal logging macros. (info! and the like).
/// The default terminal and file logging level is INFO, unless we are
/// compiling with debug assertions enabled in which case it is DEBUG.
pub const LOGGER: Logger = Logger {
    prefix: None,
    #[cfg(debug_assertions)]
    tlevel: LogLevel::DEBUG,
    #[cfg(not(debug_assertions))]
    tlevel: LogLevel::INFO,

    #[cfg(debug_assertions)]
    flevel: LogLevel::DEBUG,
    #[cfg(not(debug_assertions))]
    flevel: LogLevel::INFO,
};

/// This is Vanessa's internal logger.
pub(crate) const VANESSA_LOGGER: Logger = Logger {
    prefix: Some("Vanessa"),
    #[cfg(debug_assertions)]
    tlevel: LogLevel::DEBUG,
    #[cfg(not(debug_assertions))]
    tlevel: LogLevel::INFO,

    #[cfg(debug_assertions)]
    flevel: LogLevel::DEBUG,
    #[cfg(not(debug_assertions))]
    flevel: LogLevel::INFO,
};

// COLOR WOO
const BRACKET_COLOR: &'static str = "\x1b[38;2;108;111;133m";
const INFO_COLOR: &'static str = "\x1b[38;2;30;102;245m";
const OK_COLOR: &'static str = "\x1b[38;2;64;160;43m";
const WARN_COLOR: &'static str = "\x1b[38;2;223;142;29m";
const ERROR_COLOR: &'static str = "\x1b[38;2;230;69;83m";
const FATAL_COLOR: &'static str = "\x1b[38;2;210;15;57m";
const DEBUG_COLOR: &'static str = "\x1b[38;2;136;57;239m";
const INPUT_COLOR: &'static str = "\x1b[38;2;32;159;181m";
const HYPER_COLOR: &'static str = "\x1b[38;2;234;118;203m";
const CURIO_COLOR: &'static str = "\x1b[38;2;114;135;253m";
const STYLE_RESET: &'static str = "\x1b[0m";

/// if this is used as intended (vanessa::log::init()) at the start of the
/// program, this is safe.
/// otherwise, you're already using it wrong so i dont care
/// skill issue.
static LOG_FILE: RwLock<Option<PathBuf>> = RwLock::new(None);

// In comes the macro spamming!
#[macro_export]
macro_rules! hyper {
    ($($arg:tt)*) => ({
        $crate::log::LOGGER.log($crate::log::LogLevel::HYPER, format!($($arg)*));
    })
}
#[macro_export]
macro_rules! shyper {
    ($logger:expr, $($arg:tt)*) => ({
        $logger.log($crate::log::LogLevel::HYPER, format!($($arg)*));
    })
}
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => ({
        $crate::log::LOGGER.log($crate::log::LogLevel::DEBUG, format!($($arg)*));
    })
}
#[macro_export]
macro_rules! sdebug {
    ($logger:expr, $($arg:tt)*) => ({
        $logger.log($crate::log::LogLevel::DEBUG, format!($($arg)*));
    })
}
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => ({
        $crate::log::LOGGER.log($crate::log::LogLevel::INFO, format!($($arg)*));
    })
}
#[macro_export]
macro_rules! sinfo {
    ($logger:expr, $($arg:tt)*) => ({
        $logger.log($crate::log::LogLevel::INFO, format!($($arg)*));
    })
}
#[macro_export]
macro_rules! curio {
    ($($arg:tt)*) => ({
        $crate::log::LOGGER.log($crate::log::LogLevel::CURIO, format!($($arg)*));
    })
}
#[macro_export]
macro_rules! scurio {
    ($logger:expr, $($arg:tt)*) => ({
        $logger.log($crate::log::LogLevel::CURIO, format!($($arg)*));
    })
}
#[macro_export]
macro_rules! ok {
    ($($arg:tt)*) => ({
        $crate::log::LOGGER.log($crate::log::LogLevel::OK, format!($($arg)*));
    })
}
#[macro_export]
macro_rules! sok {
    ($logger:expr, $($arg:tt)*) => ({
        $logger.log($crate::log::LogLevel::OK, format!($($arg)*));
    })
}
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => ({
        $crate::log::LOGGER.log($crate::log::LogLevel::WARN, format!($($arg)*));
    })
}
#[macro_export]
macro_rules! swarn {
    ($logger:expr, $($arg:tt)*) => ({
        $logger.log($crate::log::LogLevel::WARN, format!($($arg)*));
    })
}
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => ({
        $crate::log::LOGGER.log($crate::log::LogLevel::ERROR, format!($($arg)*));
    })
}
#[macro_export]
macro_rules! serror {
    ($logger:expr, $($arg:tt)*) => ({
        $logger.log($crate::log::LogLevel::ERROR, format!($($arg)*));
    })
}
#[macro_export]
macro_rules! fatal {
    ($($arg:tt)*) => ({
        $crate::log::LOGGER.log($crate::log::LogLevel::FATAL, format!($($arg)*));
    })
}
#[macro_export]
macro_rules! sfatal {
    ($logger:expr, $($arg:tt)*) => ({
        $logger.log($crate::log::LogLevel::FATAL, format!($($arg)*));
    })
}
#[macro_export]
macro_rules! input {
    ($($arg:tt)*) => ({
        $crate::log::LOGGER.log($crate::log::LogLevel::INPUT, format!($($arg)*))
    })
}
#[macro_export]
macro_rules! sinput {
    ($logger:expr, $($arg:tt)*) => ({
        $logger.log($crate::log::LogLevel::INPUT, format!($($arg)*))
    })
}

/**
 * Initialize the logger.
 * This should be called immediately as any logging done before it is called
 * will not be logged to files.
 */
/// Initializes the logging system.
/// This should be called immediately as any logging done before this is called
/// will not be logged into files.
/// Calling this more than once is bad, don't do that.
/// Even if it will realistically have no effect except for printing an error
/// message.
pub fn init() {
    // if we already have a log file the user is being naughty and calling
    // this more than once.
    #[cfg(feature = "file-log")]
    {
        let read = match LOG_FILE.read() {
            Ok(file) => file,
            Err(_) => {
                VANESSA_LOGGER.log(
                    LogLevel::ERROR,
                    format!("cannot acquire log file. can't be initialized."),
                );
                return;
            }
        };
        if read.is_some() {
            VANESSA_LOGGER.log(
                LogLevel::ERROR,
                format!("vanessa::log::init() called more than once. Don't do that."),
            );
            return;
        }
        // if we dont drop this the real inits get caught forever
        // waiting for it to drop
        drop(read);
    }

    #[cfg(feature = "multilog")]
    #[cfg(feature = "file-log")]
    init_multi_log();
    #[cfg(not(feature = "multilog"))]
    #[cfg(feature = "file-log")]
    init_single_log();
}

#[allow(dead_code)]
fn init_single_log() {
    let mut file = match LOG_FILE.write() {
        Ok(file) => file,
        Err(_) => {
            return;
        }
    };
    file.replace(PathBuf::from_str("latest.log").unwrap());
    let lf = file.as_ref().unwrap();
    if std::fs::write(lf, b"").is_err() {
        // if we cant write to it we cant log to it
        file_logging_oops();
        return;
    }
}

#[allow(dead_code)]
fn init_multi_log() {
    // create a logs dir if it doesnt exist
    let logs_dir = Path::new("logs");
    if !logs_dir.exists() {
        if std::fs::create_dir_all(logs_dir).is_err() {
            // fuck
            file_logging_oops();
            return;
        }
    }

    let lf = logs_dir.join(Path::new("latest.log"));
    if lf.exists() {
        let file = match File::open(&lf) {
            Ok(file) => file,
            Err(_) => {
                file_logging_oops();
                return;
            }
        };
        let mut first_line = String::new();
        let mut reader = BufReader::new(file);
        if reader.read_line(&mut first_line).is_err() {
            file_logging_oops();
            return;
        }
        if first_line.starts_with("!Timestamp: ") {
            let timestamp = first_line[12..].replace("\n", "");
            let log_path = Path::new("logs");
            let log_path = log_path.join(Path::new(&format!("{timestamp}.log")));
            match std::fs::copy(&lf, log_path) {
                Ok(_) => {}
                Err(_) => {
                    eprintln!("Failed to copy previous log file. Cannot preserve.");
                }
            };
        } else {
            eprintln!("Previous log file was not timestamped. Cannot preserve.");
        }
    }

    match std::fs::write(
        &lf,
        format!("!Timestamp: {}\n", crate::time::timestamp_now()).as_bytes(),
    ) {
        Ok(_) => {}
        Err(_) => {
            // again, cant write, cant log
            file_logging_oops();
        }
    };
    let mut file = match LOG_FILE.write() {
        Ok(file) => file,
        Err(_) => {
            serror!(VANESSA_LOGGER, "cannot acquire file. cannot initialize.");
            return;
        }
    };
    file.replace(lf);
}

fn file_logging_oops() {
    eprintln!("Failed to initialize file logging. It will not be present.");
}

/// Represents the various log levels.
/// HYPER is for extremely spammy debug messages that you probably don't care
/// about even if you compiled in debug mode.
/// DEBUG is for debugging messages.
/// INFO is for informational stuff.
/// CURIO is for interesting or curious things that aren't necessarily warnings
/// or errors but are a bit more interesting than just info.
/// OK is for things completing successfully.
/// WARN is for warnings.
/// ERROR is for errors.
/// FATAL is for fatal errors.
/// INPUT is special and is used to receive input into your program. This is
/// why Logger::log returns an Option<String>.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LogLevel {
    HYPER = 0,
    DEBUG = 1,
    INFO = 2,
    CURIO = 3,
    OK = 4,
    WARN = 5,
    ERROR = 6,
    FATAL = 7,
    INPUT = 8,
}

impl LogLevel {
    /// Returns the ANSI escape code representing the log level's color.
    pub fn ansi_color(&self) -> &'static str {
        match self {
            LogLevel::HYPER => HYPER_COLOR,
            LogLevel::DEBUG => DEBUG_COLOR,
            LogLevel::INFO => INFO_COLOR,
            LogLevel::CURIO => CURIO_COLOR,
            LogLevel::OK => OK_COLOR,
            LogLevel::WARN => WARN_COLOR,
            LogLevel::ERROR => ERROR_COLOR,
            LogLevel::FATAL => FATAL_COLOR,
            LogLevel::INPUT => INPUT_COLOR,
        }
    }
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            LogLevel::HYPER => "HYPER",
            LogLevel::DEBUG => "DEBUG",
            LogLevel::INFO => "INFO ",
            LogLevel::CURIO => "CURIO",
            LogLevel::OK => "OK   ",
            LogLevel::WARN => "WARN ",
            LogLevel::ERROR => "ERROR",
            LogLevel::FATAL => "FATAL",
            LogLevel::INPUT => "INPUT",
        };
        write!(f, "{text}")
    }
}

/// Loggers can be used to log text to a log file and the terminal. You can
/// access a static version of this with no prefix at vanessa::log::LOGGER.
/// You can write to the default logger with the normal logging macros:
/// hyper!, debug!, info!, curio!, ok!, warn!, error!, fatal!, and input!
/// You can write to other loggers with the specific logging macros:
/// shyper!, sdebug!, sinfo!, scurio! sok!, swarn!, serro!, sfatal!, and
/// sinput!
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(C)]
pub struct Logger<'a> {
    /// Prefix for the logger if any.
    pub prefix: Option<&'a str>,
    /// Minimum level when logging to the terminal
    pub tlevel: LogLevel,
    /// Minimum level when logging to a file
    pub flevel: LogLevel,
}

impl Logger<'_> {
    /// Create a new logger with the specified prefix and minimum terminal
    /// and file levels.
    pub fn new(prefix: &str, tlevel: LogLevel, flevel: LogLevel) -> Logger {
        return Logger {
            prefix: Some(prefix),
            tlevel,
            flevel,
        };
    }

    /// Quickly create a logger with the specified prefix.
    /// Uses the default log level.
    pub fn quick(prefix: &str) -> Logger {
        return Logger {
            prefix: Some(prefix),
            #[cfg(debug_assertions)]
            tlevel: LogLevel::DEBUG,
            #[cfg(not(debug_assertions))]
            tlevel: LogLevel::INFO,
            #[cfg(debug_assertions)]
            flevel: LogLevel::DEBUG,
            #[cfg(not(debug_assertions))]
            flevel: LogLevel::INFO,
        };
    }

    /// Logging function, you'll usually want to use the macros.
    pub fn log(&self, level: LogLevel, text: String) -> Option<String> {
        let timestamp = crate::time::timestamp_now();
        if level == LogLevel::INPUT {
            self.log_term(&timestamp, level, &text);
            print!(" {INPUT_COLOR}");
            std::io::stdout().flush().ok();
            let mut input = String::new();
            match std::io::stdin().read_line(&mut input) {
                Ok(_) => {}
                Err(_) => {
                    VANESSA_LOGGER.log(
                        LogLevel::ERROR,
                        "Failed to read from STDIN for an INPUT-level log.".into(),
                    );
                    return None;
                }
            };
            print!("{STYLE_RESET}");
            std::io::stdout().flush().ok();

            // remove the newline
            input = input.replace("\n", "");
            #[cfg(feature = "file-log")]
            self.log_file(&timestamp, level, &format!("{} {}", &text, &input));
            return Some(input);
        }

        if level >= self.tlevel {
            self.log_term(&timestamp, level, &text);
        }
        #[cfg(feature = "file-log")]
        if level >= self.flevel {
            self.log_file(&timestamp, level, &text);
        }
        return None;
    }

    fn log_term(&self, timestamp: &String, level: LogLevel, text: &String) {
        #[cfg(not(feature = "compact-terminal-log"))]
        let text = format!(
            "{}({}{} {}|{} {}{level}{}){} {text}",
            BRACKET_COLOR,
            level.ansi_color(),
            timestamp,
            BRACKET_COLOR,
            level.ansi_color(),
            match self.prefix {
                Some(prefix) => {
                    format!("{} {}| {}", prefix, BRACKET_COLOR, level.ansi_color())
                }
                None => {
                    "".into()
                }
            },
            BRACKET_COLOR,
            STYLE_RESET
        );
        #[cfg(feature = "compact-terminal-log")]
        let text = format!(
            "{}({}{level}{}){} {text}",
            BRACKET_COLOR,
            level.ansi_color(),
            BRACKET_COLOR,
            STYLE_RESET
        );
        // this is here to fuckoff compile warnings when we compile with
        // compact-terminal-log
        // but since rust I can't #[cfg] this one lmao
        _ = timestamp;

        if level == LogLevel::INPUT {
            print!("{text}");
            std::io::stdout().flush().ok();
        } else {
            println!("{text}");
        }
    }

    fn log_file(&self, timestamp: &String, level: LogLevel, text: &String) {
        // we are like, extremely fail-out happy here
        let file = match LOG_FILE.read() {
            Ok(file) => file,
            Err(_) => {
                return;
            }
        };
        if file.is_none() {
            return;
        }
        let mut opts = OpenOptions::new();
        let opts = opts.write(true).append(true);
        let mut file = match opts.open(file.as_ref().unwrap()) {
            Ok(file) => file,
            Err(_) => {
                return;
            }
        };
        match file.seek(SeekFrom::End(0)) {
            Ok(_) => {}
            Err(_) => {
                return;
            }
        };
        file.write_all(
            format!(
                "({} | {}{level}) {text}\n",
                timestamp,
                match self.prefix {
                    Some(prefix) => {
                        format!("{} | ", prefix)
                    }
                    None => {
                        "".into()
                    }
                }
            )
            .as_bytes(),
        )
        .ok();
    }
}
