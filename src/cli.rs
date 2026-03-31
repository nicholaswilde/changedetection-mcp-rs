use clap::Parser;

/// A Model Context Protocol (MCP) server for ChangeDetection.io
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to the configuration file
    #[arg(short, long, value_name = "FILE", env = "CHANGEDETECTION_CONFIG")]
    pub config: Option<String>,

    /// Log level (trace, debug, info, warn, error)
    #[arg(short, long, value_name = "LEVEL", default_value = "info", env = "LOG_LEVEL")]
    pub log_level: String,

    /// ChangeDetection.io API Key
    #[arg(short, long, value_name = "KEY", env = "CHANGEDETECTION_API_KEY")]
    pub api_key: Option<String>,
}

impl Args {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
