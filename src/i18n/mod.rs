use std::{
    fs, 
    str::FromStr
};

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

fn load_resource() -> Option<FluentResource> {
    const RESOURCE_FILE_NAME: &str = "general.ftl";
    let resource_file_path: String = format!("locales/{}/{}", LOCALE, RESOURCE_FILE_NAME);
    let resource_str: String = match fs::read_to_string(&resource_file_path) {
        Ok(resource_str) => resource_str,
        Err(_) => return None
    };
    return FluentResource::try_new(resource_str).ok();
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