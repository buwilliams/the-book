fn main() {
    let mut s: String = String::new();
    s.push_str("Hello");
    let mut w = String::new();
    w.push_str("World!");
    let hello = format!("{} {}", s, w);
    println!("{}", hello);
}
