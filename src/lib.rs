pub fn greeting(name: Option<&str>) -> String {
    format!("Hello, {}!", name.unwrap_or("world"))
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
}
