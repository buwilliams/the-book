fn main() {
    println!("Hello, world!");

    let mut c = Colors::Red;

    let mut p = Person {
        first_name: String::from("Buddy"),
        last_name: String::from("Williams"),
        email: String::from("buddywilliams@gmail.com"),
    };

    let mut p2 = p.clone();
    p2.first_name = String::from("Tom");
    p2.last_name = String::from("Hanks");
    p2.email = String::from("tom@hanks.com");

    let c2 = Colors::Unknown(p2);

    if let Colors::Unknown(Person) = c2 {
        println!("{}", "Person");
    } else {
        println!("{}", "Not a person");
    }

    //println!("{:?}", c2.first_name);
    //println!("{:?}", p2);
}

#[derive(Debug, Clone)]
enum Colors {
    Black,
    White,
    Blue,
    Green,
    Red,
    Unknown(Person),
}

#[derive(Debug, Clone)]
struct Person {
    first_name: String,
    last_name: String,
    email: String,
}