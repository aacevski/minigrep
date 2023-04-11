use minigrep::Config;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
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
