fn main() {
    let maybe_fn = std::env::var("MY_VAR").unwrap_or("22".to_string());
    println!("maybe_fn = {}", maybe_fn);
}
