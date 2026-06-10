use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_week_status_wasm(start_date: &str, current_date: &str) -> String {
    get_week_status(start_date, current_date)
}

use chrono::{NaiveDate};

pub fn get_week_status(start_date: &str, current_date: &str) -> String {
    let start = NaiveDate::parse_from_str(start_date, "%Y-%m-%d").expect("Invalid start date");
    let current = NaiveDate::parse_from_str(current_date, "%Y-%m-%d").expect("Invalid current date");
    
    let duration = current.signed_duration_since(start);
    let days = duration.num_days();
    
    if days < 0 {
        return "Before start date".to_string();
    }
    
    let week = (days / 7) + 1;
    let parity = if week % 2 == 0 { "even" } else { "odd" };
    
    format!("Week {}: {}", week, parity)
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
        assert_eq!(get_week_status("2026-06-01", "2026-06-14"), "Week 2: even"); // Wait, 13 days is 6/1 + 13 = 6/14. 6/1 to 6/8 (1), 6/8 to 6/15 (2).
        // Let's re-verify:
        // day 0-6: week 1
        // day 7-13: week 2
        // day 14-20: week 3
    }
    
    #[test]
    fn test_fourteen_days_diff_is_week_three_odd() {
        assert_eq!(get_week_status("2026-06-01", "2026-06-15"), "Week 3: odd");
    }
}
