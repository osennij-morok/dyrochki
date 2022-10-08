use std::{fs, str::FromStr, sync::{Arc, Mutex}, process::exit, borrow::Borrow, fmt::Debug};

use fluent::{FluentArgs, FluentResource, FluentBundle, FluentMessage, fluent_args};
use unic_langid::LanguageIdentifier;

use super::t;

#[test]
fn test_t() {
    let args = fluent_args!["holesCount" => 4];
    let result = t("holes-found", Some(&args));
    match result {
        Some(result) => println!("{}", result),
        None => println!("Some error")
    }
}