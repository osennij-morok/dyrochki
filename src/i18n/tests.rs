use fluent::fluent_args;

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