fn main() {
    use online::check;
    println!("Online? {}", check(None).is_ok());
}
