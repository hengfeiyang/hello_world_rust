// Copyright 2023 Zinc Labs Inc.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use serde_json::value::Map;
use serde_json::value::Value;

const KEY_SEPARATOR: &str = "_";
const FORMAT_KEY_ENABLED: bool = true;

/// Flattens the provided JSON object (`current`).
///
/// It will return an error if flattening the object would make two keys to be the same,
/// overwriting a value. It will alre return an error if the JSON value passed it's not an object.
///
/// # Errors
/// Will return `Err` if `to_flatten` it's not an object, or if flattening the object would
/// result in two or more keys colliding.
pub fn flatten(to_flatten: Value) -> Result<Value, anyhow::Error> {
    let mut flat = Map::<String, Value>::new();
    flatten_value(&to_flatten, "".to_owned(), 0, &mut flat).map(|_x| Value::Object(flat))
}

/// Flattens the passed JSON value (`current`), whose path is `parent_key` and its 0-based
/// depth is `depth`.  The result is stored in the JSON object `flattened`.
fn flatten_value(
    current: &Value,
    parent_key: String,
    depth: u32,
    flattened: &mut Map<String, Value>,
) -> Result<(), anyhow::Error> {
    if depth == 0 {
        match current {
            Value::Object(map) => {
                if map.is_empty() {
                    return Ok(()); // If the top level input object is empty there is nothing to do
                }
            }
            _ => return Err(anyhow::anyhow!("flatten value must be an object")),
        }
    }

    if let Some(current) = current.as_object() {
        flatten_object(current, &parent_key, depth, flattened)?;
    } else if let Some(current) = current.as_array() {
        flatten_array(current, &parent_key, depth, flattened)?;
    } else {
        flattened.insert(parent_key, current.clone());
    }
    Ok(())
}

/// Flattens the passed object (`current`), whose path is `parent_key` and its 0-based depth
/// is `depth`.  The result is stored in the JSON object `flattened`.
fn flatten_object(
    current: &Map<String, Value>,
    parent_key: &str,
    depth: u32,
    flattened: &mut Map<String, Value>,
) -> Result<(), anyhow::Error> {
    for (k, v) in current.iter() {
        let k = if FORMAT_KEY_ENABLED {
            format_key(k)
        } else {
            k.to_string()
        };
        let parent_key = if depth > 0 {
            format!("{}{}{}", parent_key, KEY_SEPARATOR, k)
        } else {
            k
        };
        flatten_value(v, parent_key, depth + 1, flattened)?;
    }
    Ok(())
}

/// Flattens the passed array (`current`), whose path is `parent_key` and its 0-based depth
/// is `depth`.  The result is stored in the JSON object `flattened`.
fn flatten_array(
    current: &[Value],
    parent_key: &str,
    depth: u32,
    flattened: &mut Map<String, Value>,
) -> Result<(), anyhow::Error> {
    if current.is_empty() {
        return Ok(());
    }
    // for (i, obj) in current.iter().enumerate() {
    //     let parent_key = format!("{}{}{}", parent_key, KEY_SEPARATOR, i);
    //     flatten_value(obj, parent_key, depth + 1, flattened)?;
    // }
    let v = Value::String(Value::Array(current.to_vec()).to_string());
    flatten_value(&v, parent_key.to_string(), depth, flattened)?;
    Ok(())
}

/// We need every character in the key to be lowercase alphanumeric or underscore
pub fn format_key(key: &str) -> String {
    if key
        .chars()
        .all(|c| c.is_lowercase() || c.is_numeric() || c == '_')
    {
        return key.to_string();
    }
    key.chars()
        .map(|c| {
            if c.is_lowercase() || c.is_numeric() {
                c
            } else if c.is_uppercase() {
                c.to_lowercase().next().unwrap()
            } else {
                '_'
            }
        })
        .collect::<String>()
}
 