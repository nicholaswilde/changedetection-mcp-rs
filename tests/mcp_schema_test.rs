use schemars::{schema_for, JsonSchema};
use serde::Deserialize;
use serde_json::Value;

#[derive(JsonSchema, Deserialize, Debug)]
pub struct ListWatchesArgs {
    /// Optional tag to filter watches
    pub tag: Option<String>,
}

#[derive(JsonSchema, Deserialize, Debug)]
pub struct GetWatchDetailsArgs {
    /// The UUID of the watch
    pub uuid: String,
}

#[derive(JsonSchema, Deserialize, Debug)]
pub struct CreateWatchArgs {
    /// The URL to watch
    pub url: String,
    /// Optional tag to assign to the watch
    pub tag: Option<String>,
}

#[derive(JsonSchema, Deserialize, Debug)]
pub struct DeleteWatchArgs {
    /// The UUID of the watch to delete
    pub uuid: String,
}

#[derive(JsonSchema, Deserialize, Debug)]
pub struct TriggerCheckArgs {
    /// The UUID of the watch to trigger a check for
    pub uuid: String,
}

pub fn get_schema<T: JsonSchema>() -> Value {
    let schema = schema_for!(T);
    serde_json::to_value(&schema).expect("Failed to serialize schema")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_generation() {
        let schema = get_schema::<ListWatchesArgs>();
        assert_eq!(schema["type"], "object");
        assert!(schema["properties"].get("tag").is_some());
    }

    #[test]
    fn test_get_watch_details_schema() {
        let schema = get_schema::<GetWatchDetailsArgs>();
        assert_eq!(schema["type"], "object");
        assert!(schema["required"]
            .as_array()
            .unwrap()
            .contains(&Value::String("uuid".to_string())));
    }
}
