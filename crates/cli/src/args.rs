use clap::Parser;

/// Global arguments used by all subcommands
#[derive(Debug, Parser)]
#[command(name = "kora")]
pub struct GlobalArgs {
    /// Solana RPC endpoint URLs (comma-separated, or use RPC_URLS environment variable)
    /// Falls back to RPC_URL environment variable if RPC_URLS is not set.
    /// At least one endpoint is required.
    #[arg(long, env = "RPC_URLS")]
    pub rpc_urls: Option<String>,

    /// Solana RPC endpoint URL (deprecated: use --rpc-urls instead)
    #[arg(long, env = "RPC_URL")]
    pub rpc_url: Option<String>,

    /// Path to Kora configuration file (TOML format)
    #[arg(long, default_value = "kora.toml")]
    pub config: String,
}

impl GlobalArgs {
    /// Get the list of RPC endpoints, prioritizing RPC_URLS over RPC_URL.
    /// Returns an error if neither is set or if the list is empty.
    pub fn get_rpc_endpoints(&self) -> Result<Vec<String>, String> {
        let endpoints_str = self
            .rpc_urls
            .as_ref()
            .or(self.rpc_url.as_ref())
            .ok_or_else(|| {
                "No RPC endpoint configured. Set either --rpc-urls or --rpc-url, or set \
                 RPC_URLS/RPC_URL environment variables."
                    .to_string()
            })?;

        let endpoints: Vec<String> =
            endpoints_str.split(',').map(|s| s.trim().to_string()).collect();

        if endpoints.is_empty() || endpoints.iter().all(|s| s.is_empty()) {
            return Err("At least one RPC endpoint is required".to_string());
        }

        Ok(endpoints.into_iter().filter(|s| !s.is_empty()).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_rpc_endpoints_from_urls() {
        let args = GlobalArgs {
            rpc_urls: Some("http://localhost:8899,http://localhost:8900".to_string()),
            rpc_url: None,
            config: "kora.toml".to_string(),
        };
        let endpoints = args.get_rpc_endpoints().unwrap();
        assert_eq!(endpoints.len(), 2);
        assert_eq!(endpoints[0], "http://localhost:8899");
        assert_eq!(endpoints[1], "http://localhost:8900");
    }

    #[test]
    fn test_get_rpc_endpoints_fallback_to_url() {
        let args = GlobalArgs {
            rpc_urls: None,
            rpc_url: Some("http://localhost:8899".to_string()),
            config: "kora.toml".to_string(),
        };
        let endpoints = args.get_rpc_endpoints().unwrap();
        assert_eq!(endpoints.len(), 1);
        assert_eq!(endpoints[0], "http://localhost:8899");
    }

    #[test]
    fn test_get_rpc_endpoints_urls_takes_priority() {
        let args = GlobalArgs {
            rpc_urls: Some("http://localhost:8899,http://localhost:8900".to_string()),
            rpc_url: Some("http://localhost:9999".to_string()),
            config: "kora.toml".to_string(),
        };
        let endpoints = args.get_rpc_endpoints().unwrap();
        assert_eq!(endpoints.len(), 2);
        assert_eq!(endpoints[0], "http://localhost:8899");
    }

    #[test]
    fn test_get_rpc_endpoints_trims_whitespace() {
        let args = GlobalArgs {
            rpc_urls: Some("  http://localhost:8899  ,  http://localhost:8900  ".to_string()),
            rpc_url: None,
            config: "kora.toml".to_string(),
        };
        let endpoints = args.get_rpc_endpoints().unwrap();
        assert_eq!(endpoints.len(), 2);
        assert_eq!(endpoints[0], "http://localhost:8899");
        assert_eq!(endpoints[1], "http://localhost:8900");
    }

    #[test]
    fn test_get_rpc_endpoints_error_when_none() {
        let args = GlobalArgs {
            rpc_urls: None,
            rpc_url: None,
            config: "kora.toml".to_string(),
        };
        let result = args.get_rpc_endpoints();
        assert!(result.is_err());
    }
}
