pub fn flatten(json: serde_json::Value) -> Result<serde_json::Value, anyhow::Error> {
    Ok(flatten_json_object::Flattener::new().flatten(&json)?)
}
