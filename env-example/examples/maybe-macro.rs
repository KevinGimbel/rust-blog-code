fn main() {
    let maybe_macro = env!("MY_VAR", "Bitte MY_VAR setzen, z.B. mit export MY_VAR=21");
    println!("maybe_macro = {}", maybe_macro);
}
