#[derive(Debug)]
struct User {
    first_name: String,
    last_name: String,
    email: String,
}

impl User {
    fn name(&self) -> String {
        let mut s = String::from(&self.first_name);
        s.push_str(" ");
        s.push_str(&self.last_name);
        s
    }
}

fn main() {
    let u = User {
        first_name: String::from("Buddy"),
        last_name: String::from("Williams"),
        email: String::from("buddywilliams@gmail.com"),
    };

    println!("Hello, {}!", u.first_name);

    println!("struct: {:?}", u);

    println!("Hello, {}! Welcome be reaching out to you at {}", u.name(), u.email);
}
