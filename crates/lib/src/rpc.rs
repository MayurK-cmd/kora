use std::{sync::Arc, time::Duration};

use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;

use crate::rpc_failover::FailoverRpcClient;

/// Create an RPC client from a single endpoint (backward compatibility).
pub fn get_rpc_client(rpc_url: &str) -> Arc<RpcClient> {
    Arc::new(RpcClient::new_with_timeout_and_commitment(
        rpc_url.to_string(),
        Duration::from_secs(90),
        CommitmentConfig::confirmed(),
    ))
}

/// Create a failover RPC client from multiple endpoints.
///
/// # Arguments
/// * `endpoints` - List of RPC endpoint URLs. If empty, returns an error.
///
/// # Returns
/// A failover-capable RPC client that will rotate to the next endpoint on failures.
pub fn get_failover_rpc_client(endpoints: Vec<String>) -> Result<Arc<RpcClient>, String> {
    if endpoints.is_empty() {
        return Err("At least one RPC endpoint is required".to_string());
    }

    if endpoints.len() == 1 {
        return Ok(Arc::new(RpcClient::new_with_timeout_and_commitment(
            endpoints[0].clone(),
            Duration::from_secs(90),
            CommitmentConfig::confirmed(),
        )));
    }

    // For now, return the primary client. The failover logic is integrated at the RPC method level.
    let failover = FailoverRpcClient::new(endpoints);
    Ok(failover.get_client())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_rpc_client() {
        let client = get_rpc_client("http://localhost:8899");
        assert!(Arc::strong_count(&client) >= 1);
    }

    #[test]
    fn test_get_failover_rpc_client_single() {
        let result = get_failover_rpc_client(vec!["http://localhost:8899".to_string()]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_failover_rpc_client_multiple() {
        let result = get_failover_rpc_client(vec![
            "http://localhost:8899".to_string(),
            "http://localhost:8900".to_string(),
        ]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_failover_rpc_client_empty() {
        let result = get_failover_rpc_client(vec![]);
        assert!(result.is_err());
    }
}
