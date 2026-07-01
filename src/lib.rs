pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn greeting(name: Option<&str>) -> String {
    format!("Hello, {}!", name.unwrap_or("world"))
}

pub fn version() -> String {
    format!("{NAME} {VERSION}")
}

pub fn help() -> String {
    format!(
        "{NAME} {VERSION}
A hello world CLI distributed via a custom Homebrew tap.

USAGE:
    {NAME} [NAME]

ARGS:
    <NAME>    Name to greet (defaults to \"world\")

OPTIONS:
    -h, --help       Print help information
    -v, --version    Print version information"
    )
}

/// Interpret the CLI arguments (excluding the program name) and return the text
/// to print. `-h`/`--help` and `-v`/`--version` take precedence over a name.
pub fn run(args: &[String]) -> String {
    for arg in args {
        match arg.as_str() {
            "-h" | "--help" => return help(),
            "-v" | "--version" => return version(),
            _ => {}
        }
    }
    let name = args.iter().find(|a| !a.starts_with('-'));
    greeting(name.map(String::as_str))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_greeting() {
        assert_eq!(greeting(None), "Hello, world!");
    }

    #[test]
    fn named_greeting() {
        assert_eq!(greeting(Some("Peter")), "Hello, Peter!");
    }

    #[test]
    fn run_no_args_greets_world() {
        assert_eq!(run(&[]), "Hello, world!");
    }

    #[test]
    fn run_positional_is_greeted() {
        assert_eq!(run(&["Peter".to_string()]), "Hello, Peter!");
    }

    #[test]
    fn run_help_flags() {
        assert_eq!(run(&["--help".to_string()]), help());
        assert_eq!(run(&["-h".to_string()]), help());
        assert!(help().contains("USAGE"));
    }

    #[test]
    fn run_version_flags() {
        assert_eq!(run(&["--version".to_string()]), version());
        assert_eq!(run(&["-v".to_string()]), version());
        assert!(version().contains(VERSION));
    }

    #[test]
    fn flags_take_precedence_over_name() {
        assert_eq!(
            run(&["Peter".to_string(), "--version".to_string()]),
            version()
        );
    }
}
