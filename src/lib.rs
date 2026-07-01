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
/// to print, or an error message for an unrecognised flag. `-h`/`--help` and
/// `-v`/`--version` take precedence over a name.
pub fn run(args: &[String]) -> Result<String, String> {
    let mut name: Option<&str> = None;
    for arg in args {
        match arg.as_str() {
            "-h" | "--help" => return Ok(help()),
            "-v" | "--version" => return Ok(version()),
            flag if flag.starts_with('-') => {
                return Err(format!(
                    "error: unrecognised flag '{flag}'\n\nTry '--help' for usage."
                ));
            }
            positional => {
                if name.is_none() {
                    name = Some(positional);
                }
            }
        }
    }
    Ok(greeting(name))
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
        assert_eq!(run(&[]).unwrap(), "Hello, world!");
    }

    #[test]
    fn run_positional_is_greeted() {
        assert_eq!(run(&["Peter".to_string()]).unwrap(), "Hello, Peter!");
    }

    #[test]
    fn run_help_flags() {
        assert_eq!(run(&["--help".to_string()]).unwrap(), help());
        assert_eq!(run(&["-h".to_string()]).unwrap(), help());
        assert!(help().contains("USAGE"));
    }

    #[test]
    fn run_version_flags() {
        assert_eq!(run(&["--version".to_string()]).unwrap(), version());
        assert_eq!(run(&["-v".to_string()]).unwrap(), version());
        assert!(version().contains(VERSION));
    }

    #[test]
    fn flags_take_precedence_over_name() {
        assert_eq!(
            run(&["Peter".to_string(), "--version".to_string()]).unwrap(),
            version()
        );
    }

    #[test]
    fn run_unknown_flag_errors() {
        let err = run(&["--nope".to_string()]).unwrap_err();
        assert!(err.contains("unrecognised flag '--nope'"));
    }
}
