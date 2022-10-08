use std::{
    sync::{Arc, Mutex}, 
    fs, 
    str::FromStr
};

use fluent::{FluentArgs, FluentResource, FluentBundle, FluentMessage, fluent_args};
use unic_langid::LanguageIdentifier;

#[cfg(test)]
mod tests;

static LOCALE: &str = "ru_RU";
static PANIC_MSG: &str = "Malformed locales directory";

pub fn holes_found_msg(holes_count: usize) -> String {
    let args = fluent_args!["holesCount" => holes_count];
    t("holes-found", Some(&args))
        .expect(PANIC_MSG)
}

pub fn text_is_required_msg() -> String {
    t("text-is-required", None)
        .expect(PANIC_MSG)
}

pub fn arg_is_required_msg() -> String {
    t("arg-is-required", None)
        .expect(PANIC_MSG)
}

fn t(message_key: &str, args: Option<&FluentArgs>) -> Option<String> {
    let bundle: FluentBundle<Arc<FluentResource>> = create_bundle();
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

fn load_resource() -> Option<FluentResource> {
    static RESOURCE_FILE_NAME: &str = "general.ftl";
    let resource_file_path: String = format!("locales/{}/{}", LOCALE, RESOURCE_FILE_NAME);
    let resource_str: String = match fs::read_to_string(&resource_file_path) {
        Ok(resource_str) => resource_str,
        Err(_) => return None
    };
    return FluentResource::try_new(resource_str).ok();
}

fn load_cached_resource() -> Arc<FluentResource> {
    static CACHED_RESOURCE: Mutex<Option<Arc<FluentResource>>> = Mutex::new(None);
    let mut resource_lock = CACHED_RESOURCE.lock().unwrap();
    if let Some(resource) = &*resource_lock {
        return Arc::clone(resource);
    }
    let resource: FluentResource = match load_resource() {
        Some(resource) => resource,
        None => panic!("Resource file is required!")
    };
    return Arc::clone(resource_lock.insert(Arc::new(resource)));
}

fn create_bundle() -> FluentBundle<Arc<FluentResource>> {
    let mut bundle: FluentBundle<Arc<FluentResource>> = FluentBundle::new(vec![load_lang_id()]);
    bundle.add_resource(load_cached_resource()).unwrap();
    return bundle;
}

fn load_lang_id() -> LanguageIdentifier {
    match LanguageIdentifier::from_str(LOCALE).ok() {
        Some(lang_id) => lang_id,
        None => panic!("Incorrect language identifier")
    }
}