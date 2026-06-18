use crate::date::get_week_status;
use crate::dom::{document, element};
use wasm_bindgen::prelude::*;

pub(crate) fn render_status(base_date: &str, current_date: &str) -> Result<(), JsValue> {
    let status = get_week_status(base_date, current_date);
    let result = element(&document()?, "res")?;

    let Some((week, parity_eng)) = status.split_once(": ") else {
        result.set_text_content(Some(&status));
        return Ok(());
    };

    let parity = match parity_eng {
        "odd" => "单周",
        "even" => "双周",
        _ => parity_eng,
    };
    result.set_inner_html(&format!(
        "{week} <span class=\"badge badge-{parity_eng}\">{parity}</span>"
    ));

    Ok(())
}
