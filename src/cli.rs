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
    #[arg(short = 'k', long, value_name = "KEY", env = "CHANGEDETECTION_API_KEY")]
    pub api_key: Option<String>,
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
        let args = Args::try_parse_from(["changedetection-mcp-rs"]).unwrap();
        assert_eq!(args.log_level, "info");
        assert_eq!(args.config, None);
        assert_eq!(args.api_key, None);
    }

    #[test]
    fn test_parse_args_custom() {
        let args = Args::try_parse_from([
            "changedetection-mcp-rs",
            "--log-level",
            "debug",
            "--config",
            "test.toml",
            "--api-key",
            "test-key",
        ])
        .unwrap();
        assert_eq!(args.log_level, "debug");
        assert_eq!(args.config, Some("test.toml".to_string()));
        assert_eq!(args.api_key, Some("test-key".to_string()));
    }
}
