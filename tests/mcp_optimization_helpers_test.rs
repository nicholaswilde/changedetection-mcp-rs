#[cfg(test)]
mod tests {
    use serde_json::json;

    // We'll assume these functions will be implemented in a new module
    // for now we'll just define the interface we want.
    // Since we're in TDD Red phase, we expect this NOT to compile until we create the module.

    #[test]
    fn test_field_selection() {
        let data = json!({
            "id": "123",
            "name": "Test",
            "metadata": {
                "created_at": "2023-01-01",
                "owner": "admin"
            },
            "tags": ["a", "b"]
        });

        // Case 1: Select specific top-level fields
        let fields = vec!["id".to_string(), "name".to_string()];
        let filtered = changedetection_mcp_rs::mcp::helpers::filter_fields(&data, &fields);
        assert_eq!(filtered, json!({"id": "123", "name": "Test"}));

        // Case 2: Select nested fields (optional, but good for token optimization)
        // For now, let's keep it simple: top-level fields only.
        let fields = vec!["id".to_string(), "metadata".to_string()];
        let filtered = changedetection_mcp_rs::mcp::helpers::filter_fields(&data, &fields);
        assert_eq!(
            filtered,
            json!({"id": "123", "metadata": {"created_at": "2023-01-01", "owner": "admin"}})
        );
    }

    #[test]
    fn test_pagination_vec() {
        let items = vec![1, 2, 3, 4, 5];

        // Page 1, size 2
        let (paged, total) = changedetection_mcp_rs::mcp::helpers::paginate_vec(&items, 1, 2);
        assert_eq!(paged, vec![&1, &2]);
        assert_eq!(total, 5);

        // Page 2, size 2
        let (paged, total) = changedetection_mcp_rs::mcp::helpers::paginate_vec(&items, 2, 2);
        assert_eq!(paged, vec![&3, &4]);
        assert_eq!(total, 5);

        // Page 3, size 2
        let (paged, total) = changedetection_mcp_rs::mcp::helpers::paginate_vec(&items, 3, 2);
        assert_eq!(paged, vec![&5]);
        assert_eq!(total, 5);

        // Out of bounds
        let (paged, total) = changedetection_mcp_rs::mcp::helpers::paginate_vec(&items, 4, 2);
        assert!(paged.is_empty());
        assert_eq!(total, 5);
    }

    #[test]
    fn test_pagination_map() {
        use std::collections::BTreeMap;
        let mut items = BTreeMap::new();
        items.insert("a", 1);
        items.insert("b", 2);
        items.insert("c", 3);
        items.insert("d", 4);
        items.insert("e", 5);

        // Page 1, size 2
        let (paged, total) = changedetection_mcp_rs::mcp::helpers::paginate_map(&items, 1, 2);
        assert_eq!(paged.len(), 2);
        assert_eq!(total, 5);
        // BTreeMap is sorted
        assert_eq!(paged[0], (&"a", &1));
        assert_eq!(paged[1], (&"b", &2));
    }
}
