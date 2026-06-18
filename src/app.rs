use crate::config::load_config;
use crate::date::{latest_old_date, today_in_timezone};
use crate::dom::{document, element};
use crate::status::render_status;
use crate::theme::init_theme_toggle;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{Element, Event, HtmlInputElement, HtmlSelectElement};

pub(crate) async fn run_app() -> Result<(), JsValue> {
    init_theme_toggle()?;

    let config = load_config().await?;
    let today = today_in_timezone(&config.timezone)?;
    let base_date = latest_old_date(&config.base_dates, &today).unwrap_or_else(|| today.clone());

    let base_date = Rc::new(RefCell::new(base_date));
    let current_date = Rc::new(RefCell::new(today));
    let document = document()?;
    let start_value = element(&document, "start-val")?;
    let current_value = element(&document, "current-val")?;

    render_date_value(
        &start_value,
        &base_date.borrow(),
        &render_base_date_select(&config.base_dates, &base_date.borrow()),
    );
    render_date_value(
        &current_value,
        &current_date.borrow(),
        &render_date_picker(&current_date.borrow(), "Select current date"),
    );
    render_status(&base_date.borrow(), &current_date.borrow())?;

    attach_base_date_listener(start_value, Rc::clone(&base_date), Rc::clone(&current_date))?;
    attach_current_date_listener(current_value, base_date, current_date)?;

    Ok(())
}

fn render_base_date_select(dates: &[String], selected_date: &str) -> String {
    let mut dates = dates.to_vec();
    dates.sort();

    let options = dates
        .iter()
        .map(|date| {
            if date == selected_date {
                format!("<option value=\"{date}\" selected>{date}</option>")
            } else {
                format!("<option value=\"{date}\">{date}</option>")
            }
        })
        .collect::<String>();

    format!(
        "<select class=\"date-control\" aria-label=\"Select semester start date\">{options}</select>"
    )
}

fn render_date_picker(selected_date: &str, label: &str) -> String {
    format!(
        "<input class=\"date-control\" type=\"date\" value=\"{selected_date}\" aria-label=\"{label}\">"
    )
}

fn render_date_value(element: &Element, date: &str, control: &str) {
    element.set_inner_html(&format!(
        "<span class=\"date-current\">{date}</span>{control}"
    ));
}

fn attach_base_date_listener(
    start_value: Element,
    base_date: Rc<RefCell<String>>,
    current_date: Rc<RefCell<String>>,
) -> Result<(), JsValue> {
    let select = start_value
        .query_selector(".date-control")?
        .ok_or_else(|| JsValue::from_str("base date select not found"))?
        .dyn_into::<HtmlSelectElement>()?;
    let current_label = start_value
        .query_selector(".date-current")?
        .ok_or_else(|| JsValue::from_str("base date label not found"))?;

    let closure = Closure::<dyn FnMut(Event)>::new(move |event: Event| {
        let Some(target) = event.target() else {
            return;
        };
        let Ok(select) = target.dyn_into::<HtmlSelectElement>() else {
            return;
        };
        let selected_date = select.value();
        *base_date.borrow_mut() = selected_date.clone();
        current_label.set_text_content(Some(&selected_date));
        let _ = render_status(&base_date.borrow(), &current_date.borrow());
    });
    select.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())?;
    closure.forget();

    Ok(())
}

fn attach_current_date_listener(
    current_value: Element,
    base_date: Rc<RefCell<String>>,
    current_date: Rc<RefCell<String>>,
) -> Result<(), JsValue> {
    let input = current_value
        .query_selector(".date-control")?
        .ok_or_else(|| JsValue::from_str("current date input not found"))?
        .dyn_into::<HtmlInputElement>()?;
    let current_label = current_value
        .query_selector(".date-current")?
        .ok_or_else(|| JsValue::from_str("current date label not found"))?;

    let closure = Closure::<dyn FnMut(Event)>::new(move |event: Event| {
        let Some(target) = event.target() else {
            return;
        };
        let Ok(input) = target.dyn_into::<HtmlInputElement>() else {
            return;
        };
        let selected_date = input.value();
        *current_date.borrow_mut() = selected_date.clone();
        current_label.set_text_content(Some(&selected_date));
        let _ = render_status(&base_date.borrow(), &current_date.borrow());
    });
    input.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())?;
    closure.forget();

    Ok(())
}
