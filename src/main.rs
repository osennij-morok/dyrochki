use cli::CLICommand;

mod holes_counter;
mod i18n;
mod cli;
mod web_server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let cli_command: CLICommand = cli::run();
    match cli_command {
        CLICommand::Count(text) => holes_counter::count_to_stdout(&text),
        CLICommand::Server { 
            host, 
            port, 
            secured 
        } => return web_server::run(&host, port, secured).await
    }
    Ok(())
}