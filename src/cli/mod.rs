use clap::{command, Command, ArgMatches, arg, value_parser};

#[cfg(test)]
mod tests;

const DEFAULT_HOST: &str = "localhost";
const DEFAULT_PORT: u16 = 8080;
const DEFAULT_SECURED: bool = false;

#[derive(Debug)]
pub enum CLICommand {
    Server {
        host: String, 
        port: u16,
        secured: bool
    },
    Count(String),
}

pub fn run() -> CLICommand {
    let matches: ArgMatches = parse_args();
    if let Some(submatch) = matches.subcommand_matches("server") {
        let port: u16 = match submatch.get_one::<u16>("port") {
            Some(port) => port.clone(),
            None => default_port()
        };
        let host: String = match submatch.get_one::<String>("host") {
            Some(host) => host.clone(),
            None => DEFAULT_HOST.to_owned()
        };
        let secured: bool = match submatch.get_one::<bool>("secured") {
            Some(secured) => *secured,
            None => DEFAULT_SECURED
        };
        println!("На порту {} можно посчитать дырочки....", port);
        return CLICommand::Server { host, port, secured };
    }
    if let Some(submatch) = matches.subcommand_matches("count") {
        let text: String = match submatch.get_one::<String>("text") {
            Some(text) => text.clone(),
            None => unreachable!()
        };
        return CLICommand::Count(text);
    }
    unreachable!();
}

fn parse_args() -> ArgMatches {
    command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("server")
                .about("Runs http server")
                .arg(
                    arg!(-p --port [PORT] "Port to serve content on")
                        .value_parser(value_parser!(u16))
                )
                .arg(
                    arg!(--host [HOST] "Host to serve content on")
                )
                .arg(
                    arg!(-s --secured "Use https")
                )
        )
        .subcommand(
            Command::new("count")
                .arg_required_else_help(true)
                .about("Counts holes amount in specified text")
                .arg(
                    arg!([text] "Text to count holes in")
                )
        )
        .get_matches()
}

fn default_port() -> u16 {
    port_from_env().unwrap_or(DEFAULT_PORT)
}

fn port_from_env() -> Option<u16> {
    std::env::var("PORT")
        .ok()
        .map_or(None, |port_str| u16::from_str_radix(&port_str, 10).ok())
}