use crate::dom::{document, element, window};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{Event, MediaQueryList};

pub(crate) fn init_theme_toggle() -> Result<(), JsValue> {
    let media = window()?
        .match_media("(prefers-color-scheme: dark)")?
        .ok_or_else(|| JsValue::from_str("theme media query unavailable"))?;
    let selected_theme = Rc::new(RefCell::new(system_theme(&media)));
    let toggle = element(&document()?, "theme-toggle")?;

    apply_theme(&selected_theme.borrow())?;

    {
        let selected_theme = Rc::clone(&selected_theme);
        let media_for_listener = media.clone();
        let closure = Closure::<dyn FnMut(Event)>::new(move |_event: Event| {
            *selected_theme.borrow_mut() = system_theme(&media_for_listener);
            let _ = apply_theme(&selected_theme.borrow());
        });
        media.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let selected_theme = Rc::clone(&selected_theme);
        let closure = Closure::<dyn FnMut(Event)>::new(move |_event: Event| {
            let next_theme = if selected_theme.borrow().as_str() == "dark" {
                "light".to_string()
            } else {
                "dark".to_string()
            };
            *selected_theme.borrow_mut() = next_theme;
            let _ = apply_theme(&selected_theme.borrow());
        });
        toggle.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}

fn system_theme(media: &MediaQueryList) -> String {
    if media.matches() {
        "dark".to_string()
    } else {
        "light".to_string()
    }
}

fn apply_theme(theme: &str) -> Result<(), JsValue> {
    let document = document()?;
    let root = document
        .document_element()
        .ok_or_else(|| JsValue::from_str("documentElement not found"))?;
    root.set_attribute("data-theme", theme)?;

    let toggle = element(&document, "theme-toggle")?;
    let next_theme = if theme == "dark" { "light" } else { "dark" };
    toggle.set_text_content(Some(if theme == "dark" { "Dark" } else { "Light" }));
    toggle.set_attribute(
        "aria-pressed",
        if theme == "dark" { "true" } else { "false" },
    )?;
    toggle.set_attribute("aria-label", &format!("Switch to {next_theme} theme"))?;

    Ok(())
}
