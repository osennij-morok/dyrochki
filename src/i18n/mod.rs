use std::str::FromStr;

use fluent::{FluentArgs, FluentResource, FluentBundle, FluentMessage};
use lazy_static::lazy_static;
use unic_langid::LanguageIdentifier;

#[cfg(test)]
mod tests;
pub mod holes;

const LOCALE: &str = "ru_RU";
const PANIC_MSG: &str = "Malformed locales directory";

fn t(message_key: &str, args: Option<&FluentArgs>) -> Option<String> {
    let bundle: FluentBundle<&'static FluentResource> = create_bundle();
    let msg: FluentMessage = match bundle.get_message(message_key) {
        Some(msg) => msg,
        None => return None
    };
    let pattern = match msg.value() {
        Some(pattern) => pattern,
        None => return None
    };
    let mut errors = vec![];
    let result: String = bundle.format_pattern(pattern, args, &mut errors).to_string();
    if !errors.is_empty() {
        return None;
    }
    return Some(result);
}

lazy_static! {
    static ref RESOURCE: FluentResource = load_resource()
        .expect("Resource file is required!");
}

macro_rules! LOCALE { () => { "ru_RU" } }
macro_rules! RESOURCE_FILE_NAME { () => { "general.ftl" } }
macro_rules! RESOURCE_FILE_PATH {
    () => { concat!("../../locales/", LOCALE!(), "/", RESOURCE_FILE_NAME!()) }
}

fn load_resource() -> Option<FluentResource> {
    const RESOURCE_STR: &str = include_str!(RESOURCE_FILE_PATH!());
    return FluentResource::try_new(RESOURCE_STR.to_owned()).ok();
}

fn create_bundle() -> FluentBundle<&'static FluentResource> {
    let mut bundle: FluentBundle<&'static FluentResource> = FluentBundle::new(vec![load_lang_id()]);
    bundle.add_resource(&RESOURCE).unwrap();
    return bundle;
}

fn load_lang_id() -> LanguageIdentifier {
    match LanguageIdentifier::from_str(LOCALE).ok() {
        Some(lang_id) => lang_id,
        None => panic!("Incorrect language identifier")
    }
}