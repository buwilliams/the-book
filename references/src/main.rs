fn main() {
    let mut s: String = String::from("Hello, world");

    append(&mut s);
    println!("{}", s);

    print_length_ref(&s);
    print_length_ref(&s);

    let s = print_length_shadow(s);
    print_length_shadow(s);
}

fn append(s: &mut String) {
    s.push('!');
}

fn print_length_ref(s: &String) {
    println!("{}", s.len());
}

fn print_length_shadow(s: String) -> String {
    println!("{}", s.len());
    s
}