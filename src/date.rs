use chrono::NaiveDate;
use js_sys::{Array, Date, Function, Object, Reflect};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

pub(crate) fn get_week_status(start_date: &str, current_date: &str) -> String {
    let start = NaiveDate::parse_from_str(start_date, "%Y-%m-%d").expect("Invalid start date");
    let current =
        NaiveDate::parse_from_str(current_date, "%Y-%m-%d").expect("Invalid current date");

    let duration = current.signed_duration_since(start);
    let days = duration.num_days();
    if days < 0 {
        return "Before start date".to_string();
    }

    let week = (days / 7) + 1;
    let parity = if week % 2 == 0 { "even" } else { "odd" };

    format!("Week {}: {}", week, parity)
}

pub(crate) fn latest_old_date(dates: &[String], today: &str) -> Option<String> {
    dates
        .iter()
        .filter(|date| date.as_str() <= today)
        .max()
        .cloned()
        .or_else(|| dates.iter().min().cloned())
}

pub(crate) fn today_in_timezone(timezone: &str) -> Result<String, JsValue> {
    let options = Object::new();
    Reflect::set(
        &options,
        &JsValue::from_str("timeZone"),
        &JsValue::from_str(timezone),
    )?;
    Reflect::set(
        &options,
        &JsValue::from_str("year"),
        &JsValue::from_str("numeric"),
    )?;
    Reflect::set(
        &options,
        &JsValue::from_str("month"),
        &JsValue::from_str("2-digit"),
    )?;
    Reflect::set(
        &options,
        &JsValue::from_str("day"),
        &JsValue::from_str("2-digit"),
    )?;

    let intl = Reflect::get(&js_sys::global(), &JsValue::from_str("Intl"))?;
    let date_time_format =
        Reflect::get(&intl, &JsValue::from_str("DateTimeFormat"))?.dyn_into::<Function>()?;
    let args = Array::new();
    args.push(&JsValue::from_str("en-CA"));
    args.push(&options);
    let formatter = Reflect::construct(&date_time_format, &args)?;
    let format = Reflect::get(&formatter, &JsValue::from_str("format"))?.dyn_into::<Function>()?;
    let formatted = format.call1(&formatter, &Date::new_0().into())?;

    formatted
        .as_string()
        .ok_or_else(|| JsValue::from_str("date format failed"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_same_day_is_week_one_odd() {
        assert_eq!(get_week_status("2026-06-01", "2026-06-01"), "Week 1: odd");
    }

    #[test]
    fn test_seven_days_diff_is_week_two_even() {
        assert_eq!(get_week_status("2026-06-01", "2026-06-08"), "Week 2: even");
    }

    #[test]
    fn test_thirteen_days_diff_is_week_two_even() {
        assert_eq!(get_week_status("2026-06-01", "2026-06-14"), "Week 2: even");
    }

    #[test]
    fn test_fourteen_days_diff_is_week_three_odd() {
        assert_eq!(get_week_status("2026-06-01", "2026-06-15"), "Week 3: odd");
    }

    #[test]
    fn test_before_start_date() {
        assert_eq!(
            get_week_status("2026-07-06", "2026-06-19"),
            "Before start date"
        );
    }

    #[test]
    fn test_latest_old_date() {
        let dates = vec![
            "2026-03-02".to_string(),
            "2026-07-06".to_string(),
            "2026-09-07".to_string(),
        ];
        assert_eq!(
            latest_old_date(&dates, "2026-08-01"),
            Some("2026-07-06".to_string())
        );
    }
}
