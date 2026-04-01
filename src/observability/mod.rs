use tracing_subscriber::{fmt, prelude::*, EnvFilter, Registry, Layer};
use tracing_appender::non_blocking::WorkerGuard;
use std::io;

pub struct TracingGuard {
    pub _file_guard: Option<WorkerGuard>,
}

/// Initialize tracing with the given log level, optional log file, and optional JSON output.
pub fn init_tracing(log_level: &str, log_file: Option<&str>, json: bool) -> TracingGuard {
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(log_level))
        .unwrap_or_else(|_| EnvFilter::new("info"));

    let (stdout_layer, file_layer, file_guard) = if json {
        let stdout = fmt::layer().json().with_writer(io::stdout).boxed();
        let (file, guard) = if let Some(file_path) = log_file {
            let file_appender = tracing_appender::rolling::daily(".", file_path);
            let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
            let layer = fmt::layer().json().with_writer(non_blocking).boxed();
            (Some(layer), Some(guard))
        } else {
            (None, None)
        };
        (stdout, file, guard)
    } else {
        let stdout = fmt::layer().with_writer(io::stdout).boxed();
        let (file, guard) = if let Some(file_path) = log_file {
            let file_appender = tracing_appender::rolling::daily(".", file_path);
            let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
            let layer = fmt::layer().with_writer(non_blocking).boxed();
            (Some(layer), Some(guard))
        } else {
            (None, None)
        };
        (stdout, file, guard)
    };

    let registry = Registry::default().with(filter_layer).with(stdout_layer);

    if let Some(layer) = file_layer {
        let _ = registry.with(layer).try_init();
    } else {
        let _ = registry.try_init();
    }

    TracingGuard {
        _file_guard: file_guard,
    }
}
