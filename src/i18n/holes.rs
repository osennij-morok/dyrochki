use fluent::fluent_args;

use super::{PANIC_MSG, t};

pub fn holes_found_msg(holes_count: usize) -> String {
    let args = fluent_args!["holesCount" => holes_count];
    t("holes-found", Some(&args))
        .expect(PANIC_MSG)
}

pub fn input_text_is_empty_msg() -> String {
    t("input-text-is-empty", None)
        .expect(PANIC_MSG)
}

pub fn text_label_msg() -> String {
    t("text-label", None)
        .expect(PANIC_MSG)
}

pub fn count_holes_btn_msg() -> String {
    t("count-holes-btn", None)
        .expect(PANIC_MSG)
}

pub fn index_title_msg() -> String {
    t("index-title", None)
        .expect(PANIC_MSG)
}

pub fn uncounted_chars_msg(uncounted_chars: &[char]) -> String {
    if uncounted_chars.is_empty() {
        return String::new();
    }
    let uncounted_chars_str: String = uncounted_chars.iter()
        .map(|ch| ch.to_string())
        .collect::<Vec<String>>()
        .join(", ");
    let args = fluent_args![
        "uncountedCharsCount" => uncounted_chars.len(),
        "uncountedCharsStr" => uncounted_chars_str
    ];
    t("uncounted-chars", Some(&args))
        .unwrap_or(String::new())
}

#[allow(dead_code)]
pub fn text_is_required_msg() -> String {
    t("text-is-required", None)
        .expect(PANIC_MSG)
}

#[allow(dead_code)]
pub fn arg_is_required_msg() -> String {
    t("arg-is-required", None)
        .expect(PANIC_MSG)
}