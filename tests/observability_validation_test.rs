use changedetection_mcp_rs::api::Client;
use changedetection_mcp_rs::mcp::McpServer;
use mcp_sdk_rs::server::ServerHandler;
use std::sync::Arc;
use tracing_subscriber::{fmt, prelude::*, Layer, Registry};
use std::sync::Mutex;

struct MockWriter {
    logs: Arc<Mutex<Vec<String>>>,
}

impl std::io::Write for MockWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let s = String::from_utf8_lossy(buf).to_string();
        self.logs.lock().unwrap().push(s);
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

// Implement Clone to make it compatible with move closure
impl Clone for MockWriter {
    fn clone(&self) -> Self {
        Self {
            logs: self.logs.clone(),
        }
    }
}

#[tokio::test]
async fn test_log_correlation_id() {
    let logs = Arc::new(Mutex::new(Vec::new()));
    let mock_writer = MockWriter { logs: logs.clone() };

    let layer = fmt::layer()
        .json()
        .with_writer(move || mock_writer.clone())
        .boxed();
    
    let subscriber = Registry::default().with(layer);
    
    let _guard = tracing::subscriber::set_default(subscriber);
    
    let client = Client::new("http://localhost:5000".to_string(), "test-key".to_string());
    let server = McpServer::new(client);
    
    // This should trigger the instrumented handle_method
    let _ = server.handle_method("tools/list", None).await;
    
    // Explicitly drop guard and subscriber to ensure logs are flushed
    drop(_guard);

    let captured_logs = logs.lock().unwrap();
    assert!(!captured_logs.is_empty(), "No logs captured");
    
    let mut found_correlation = false;
    for log in captured_logs.iter() {
        if log.contains("mcp_request") && log.contains("request_id") {
            found_correlation = true;
            // Verify it's valid JSON
            let v: serde_json::Value = serde_json::from_str(log).unwrap();
            assert!(v.get("span").is_some(), "Span not found in log: {}", log);
            assert!(v["span"].get("request_id").is_some(), "request_id not found in span: {}", log);
            assert!(v["span"].get("method").is_some(), "method not found in span: {}", log);
            assert_eq!(v["span"]["method"], "tools/list");
        }
    }
    assert!(found_correlation, "Correlation ID not found in logs: {:?}", captured_logs);
}
