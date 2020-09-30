fn main() {
    let version = env!("CARGO_PKG_VERSION");
    let name = env!("CARGO_PKG_NAME");
    let env_macro = env!("MY_VAR");
    let env_fn = std::env::var("MY_VAR").unwrap();

    println!("name: {}", name);
    println!("version: {}", version);
    println!("env_macro = {}", env_macro);
    println!("env_fn = {}", env_fn);
}
