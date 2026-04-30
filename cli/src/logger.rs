use std::path::PathBuf;
use std::process::exit;

use chrono::Local;
use fern::colors::{Color, ColoredLevelConfig};
use log::debug;

pub fn setup_logger(log_path: Option<PathBuf>) {
    // Create dispatch
    let dispatch = fern::Dispatch::new();

    // Set colors
    let colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .warn(Color::Yellow)
        .error(Color::Red);

    // Set dispatch formatting
    let dispatch = dispatch.format(move |out, message, record| {
            out.finish(format_args!(
                "[{} {}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                colors.color(record.level()),
                message
            ))
        })
        .chain(std::io::stdout());

    // Set to log level filter
    let mut dispatch = dispatch.level(log::LevelFilter::Info);

    if let Some(log_path) = log_path {
        let log_file = format!(
            "{}/{}.log",
            log_path.display(),
            Local::now().format("%Y-%m-%d_%H-%M-%S")
        );

        dispatch = dispatch.chain(fern::log_file(&log_file).unwrap_or_else(|err| {
            eprintln!("An unexpected Error occurred while trying to add a log file at {log_file:?} to the logger, Err: {err}");
            exit(1)
        }));
    }

    dispatch.apply().unwrap_or_else(|err| {
        eprintln!("An unexpected Error occurred while trying to setup the logger, Err: {err}");
        exit(1);
    });

    debug!("Logger initialized!");
}
