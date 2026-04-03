mod common;

use common::MockApp;

#[tokio::test]
async fn test_list_processors() {
    let app = MockApp::new().await;
    let mock_spec = r#"
components:
  schemas:
    Watch:
      properties:
        processor:
          enum:
          - restock_diff
          - text_json_diff
          - visual_diff
"#;

    app.mock_get_text("/api/v1/full-spec", 200, mock_spec).await;

    let processors = app.client.list_processors().await.unwrap();
    assert_eq!(processors.len(), 3);
    assert!(processors.contains(&"restock_diff".to_string()));
    assert!(processors.contains(&"text_json_diff".to_string()));
    assert!(processors.contains(&"visual_diff".to_string()));
}
