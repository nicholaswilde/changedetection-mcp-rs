use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub enum Transport {
    Stdio,
    Http,
}

#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub enum LogFormat {
    Text,
    Json,
}

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

    /// Log format (text, json)
    #[arg(long, value_enum, default_value = "text", env = "LOG_FORMAT")]
    pub log_format: LogFormat,

    /// Log file path
    #[arg(long, value_name = "FILE", env = "LOG_FILE")]
    pub log_file: Option<String>,

    /// ChangeDetection.io API Key
    #[arg(short = 'k', long, value_name = "KEY", env = "CHANGEDETECTION_API_KEY")]
    pub api_key: Option<String>,

    /// Transport to use (stdio, http)
    #[arg(short, long, value_enum, default_value = "stdio", env = "MCP_TRANSPORT")]
    pub transport: Transport,

    /// HTTP host
    #[arg(long, default_value = "127.0.0.1", env = "MCP_HOST")]
    pub host: String,

    /// HTTP port
    #[arg(short, long, default_value = "3000", env = "MCP_PORT")]
    pub port: u16,
}

impl Args {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_args_default() {
        std::env::remove_var("CHANGEDETECTION_API_KEY");
        std::env::remove_var("CHANGEDETECTION_CONFIG");
        std::env::remove_var("LOG_LEVEL");
        std::env::remove_var("LOG_FORMAT");
        std::env::remove_var("LOG_FILE");
        std::env::remove_var("MCP_TRANSPORT");
        let args = Args::try_parse_from(["changedetection-mcp-rs"]).unwrap();
        assert_eq!(args.log_level, "info");
        assert_eq!(args.log_format, LogFormat::Text);
        assert_eq!(args.log_file, None);
        assert_eq!(args.config, None);
        assert_eq!(args.api_key, None);
        assert_eq!(args.transport, Transport::Stdio);
    }

    #[test]
    fn test_parse_args_custom() {
        let args = Args::try_parse_from([
            "changedetection-mcp-rs",
            "--log-level",
            "debug",
            "--log-format",
            "json",
            "--log-file",
            "mcp.log",
            "--config",
            "test.toml",
            "--api-key",
            "test-key",
            "--transport",
            "http",
            "--host",
            "0.0.0.0",
            "--port",
            "8080",
        ])
        .unwrap();
        assert_eq!(args.log_level, "debug");
        assert_eq!(args.log_format, LogFormat::Json);
        assert_eq!(args.log_file, Some("mcp.log".to_string()));
        assert_eq!(args.config, Some("test.toml".to_string()));
        assert_eq!(args.api_key, Some("test-key".to_string()));
        assert_eq!(args.transport, Transport::Http);
        assert_eq!(args.host, "0.0.0.0");
        assert_eq!(args.port, 8080);
    }
}
