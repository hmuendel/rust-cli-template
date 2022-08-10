//! The logging module sets up the logging system as specified in the
//! configuration.
//!
//! It uses the [tracing](https://crates.io/crates/tracing)
//! crate to make it easy to trace the execution of the program through
//! different threads or asynchronous execution.
//!
//! The verbosity can be controlled via the verbosity config option.
use crate::Cfg;
use once_cell::sync::OnceCell;
use tracing::{debug, info, trace, warn};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_log::log::LevelFilter;

static GUARD: OnceCell<WorkerGuard> = OnceCell::new();

pub fn setup_logging(cfg: &Cfg) {
    if let Err(err) = tracing_log::LogTracer::init() {
        println!("Failed to initialize log tracer: {}", err);
    }
    // create appender for standard error
    let (non_blocking, guard) = tracing_appender::non_blocking(std::io::stderr());
    GUARD.set(guard).expect("Failed to set appender guard");

    let tracing_level = match cfg.verbosity.log_level_filter() {
        LevelFilter::Off => None,
        LevelFilter::Error => Some(tracing::Level::ERROR),
        LevelFilter::Warn => Some(tracing::Level::WARN),
        LevelFilter::Info => Some(tracing::Level::INFO),
        LevelFilter::Debug => Some(tracing::Level::DEBUG),
        LevelFilter::Trace => Some(tracing::Level::TRACE),
    };
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_writer(non_blocking)
        .with_max_level(tracing_level)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Setting the default tracing subscriber failed");
    match cfg.verbosity.log_level_filter() {
        LevelFilter::Off => {}
        LevelFilter::Error => {}
        LevelFilter::Warn => warn!("Verbosity level set to warn"),
        LevelFilter::Info => info!("Verbosity level set to info"),
        LevelFilter::Debug => debug!("Verbosity level set to debug"),
        LevelFilter::Trace => trace!("Verbosity level set to trace"),
    }
    trace!("Logging initialized");
}
