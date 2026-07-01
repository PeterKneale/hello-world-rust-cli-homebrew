fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    println!("{}", hello_world_rust_cli_homebrew::run(&args));
}
