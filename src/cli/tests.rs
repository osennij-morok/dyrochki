use crate::cli::{CLICommand, run};

#[test]
fn test_cli() {
    let parsed_command: CLICommand = run();
    dbg!(&parsed_command);
}