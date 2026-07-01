fn main() {
    let name = std::env::args().nth(1);
    println!(
        "{}",
        hello_world_rust_cli_homebrew::greeting(name.as_deref())
    );
}
