use chrono::{DateTime, Utc};
use serde_json::Value;

pub fn opt_string(v: &Value, key: &str) -> Option<String> {
    v.get(key).and_then(|x| x.as_str().map(str::to_owned))
}

pub fn opt_bool(v: &Value, key: &str) -> Option<bool> {
    v.get(key).and_then(|x| x.as_bool())
}

pub fn bool_or(v: &Value, key: &str, default: bool) -> bool {
    v.get(key).and_then(|x| x.as_bool()).unwrap_or(default)
}

pub fn i64_or(v: &Value, key: &str, default: i64) -> i64 {
    v.get(key).and_then(|x| x.as_i64()).unwrap_or(default)
}

pub fn string_or(v: &Value, key: &str) -> String {
    v.get(key)
        .map(|x| match x {
            Value::String(s) => s.clone(),
            _ => x.to_string(),
        })
        .unwrap_or_default()
}
pub fn opt_vec_string(v: &serde_json::Value, key: &str) -> Option<Vec<String>> {
    match v.get(key)? {
        Value::Array(arr) => Some(
            arr.iter()
                .filter_map(|item| item.as_str().map(|s| s.to_string()))
                .collect(),
        ),
        _ => None,
    }
}

pub fn opt_i64(v: &Value, key: &str) -> Option<i64> {
    v.get(key).and_then(|x| x.as_i64())
}

pub fn opt_f64(v: &Value, key: &str) -> Option<f64> {
    v.get(key).and_then(|x| x.as_f64())
}

pub fn opt_f32(v: &serde_json::Value, key: &str) -> Option<f32> {
    v.get(key)?.as_f64().map(|n| n as f32)
}

pub fn opt_datetime(v: &serde_json::Value, key: &str) -> Option<DateTime<Utc>> {
    v.get(key)?
        .as_str()
        .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&Utc))
}

pub fn opt_vec_i64(v: &Value, key: &str) -> Option<Vec<i64>> {
    let arr = v.get(key)?.as_array()?;
    // Collect only integer-like entries; if any element is non-integer, return None
    let mut out = Vec::with_capacity(arr.len());
    for el in arr {
        if let Some(i) = el.as_i64() {
            out.push(i);
        } else if let Some(u) = el.as_u64() {
            out.push(u as i64);
        } else if let Some(s) = el.as_str() {
            // allow stringified integers
            if let Ok(i) = s.parse::<i64>() {
                out.push(i);
            } else {
                return None;
            }
        } else {
            return None;
        }
    }
    Some(out)
}

pub fn opt_value(v: &Value, key: &str) -> Option<Value> {
    v.get(key).cloned()
}

// If you want to accept either camelCase or snake_case for a key:
pub fn opt_string_any(v: &Value, keys: &[&str]) -> Option<String> {
    keys.iter().find_map(|k| opt_string(v, k))
}
