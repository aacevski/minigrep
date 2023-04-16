use minigrep::Config;
use std::env;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    if minigrep::STDOUT_COMMANDS.contains(&config.query.as_str()) {
        minigrep::run_command(config).unwrap_or_else(|err| {
            eprintln!("Problem running command: {}", err);
            std::process::exit(1);
        });
    } else {
        minigrep::run(config).unwrap_or_else(|err| {
            eprintln!("Problem running command: {}", err);
            std::process::exit(1);
        });
    }
}
