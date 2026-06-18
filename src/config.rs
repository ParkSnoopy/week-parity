use crate::dom::window;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::Response;

#[derive(Clone, Debug)]
pub(crate) struct Config {
    pub(crate) base_dates: Vec<String>,
    pub(crate) timezone: String,
}

pub(crate) async fn load_config() -> Result<Config, JsValue> {
    load_config_toml().await
}

async fn load_config_toml() -> Result<Config, JsValue> {
    let response_value = JsFuture::from(window()?.fetch_with_str("./config.toml")).await?;
    let response: Response = response_value.dyn_into()?;
    if !response.ok() {
        return Err(JsValue::from_str("config.toml fetch failed"));
    }

    let text_value = JsFuture::from(response.text()?).await?;
    let text = text_value
        .as_string()
        .ok_or_else(|| JsValue::from_str("config.toml text failed"))?;

    parse_config_toml(&text).map_err(JsValue::from_str)
}

fn parse_config_toml(toml: &str) -> Result<Config, &'static str> {
    let raw_dates = extract_between(toml, "base_date", '[', ']')
        .ok_or("config.toml missing base_date array")?;
    let base_dates: Vec<String> = raw_dates
        .split(',')
        .filter_map(|part| part.trim().trim_matches('"').split('#').next())
        .map(str::trim)
        .filter(|date| !date.is_empty())
        .map(ToOwned::to_owned)
        .collect();
    if base_dates.is_empty() {
        return Err("config.toml base_date array is empty");
    }

    let timezone = extract_quoted_value(toml, "timezone").ok_or("config.toml missing timezone")?;

    Ok(Config {
        base_dates,
        timezone,
    })
}

fn extract_between(source: &str, key: &str, start: char, end: char) -> Option<String> {
    let value_start = source.find(key)?;
    let tail = &source[value_start..];
    let start_index = tail.find(start)? + 1;
    let end_index = tail[start_index..].find(end)? + start_index;
    Some(tail[start_index..end_index].to_string())
}

fn extract_quoted_value(source: &str, key: &str) -> Option<String> {
    let value_start = source.find(key)?;
    let tail = &source[value_start..];
    let quote_start = tail.find('"')? + 1;
    let quote_end = tail[quote_start..].find('"')? + quote_start;
    Some(tail[quote_start..quote_end].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_config_toml() {
        let config = parse_config_toml(
            "base_date = [\"2026-03-02\", \"2026-07-06\"]\ntimezone = \"Asia/Shanghai\"",
        )
        .expect("valid config");
        assert_eq!(config.base_dates, vec!["2026-03-02", "2026-07-06"]);
        assert_eq!(config.timezone, "Asia/Shanghai");
    }

    #[test]
    fn test_parse_config_toml_requires_base_date() {
        assert_eq!(
            parse_config_toml("timezone = \"Asia/Shanghai\"").unwrap_err(),
            "config.toml missing base_date array"
        );
    }

    #[test]
    fn test_parse_config_toml_requires_timezone() {
        assert_eq!(
            parse_config_toml("base_date = [\"2026-03-02\"]").unwrap_err(),
            "config.toml missing timezone"
        );
    }
}
