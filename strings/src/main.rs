fn main() {
    let mut s: String = String::new();
    s.push_str("Hello");
    let mut w = String::new();
    w.push_str("World!");
    let hello = format!("{} {}", s, w);
    println!("{}", hello);

    println!("{}", format!("{} {}!", String::from("Hello"), String::from("world")));

    let a = format!("{} {}!", String::from("Hello"), String::from("world"));
    println!("{}", &a[0..1]);
}
