fn main() {
    let buddy = Person {
        first_name: String::from("Buddy"),
        last_name: String::from("Williams"),
        email: String::from("buddywilliams@gmail.com"),
    };

    let infinite = Book {
        title: String::from("Infinite"),
        author: String::from("Jeremy Robinson"),
    };

    println!("{}", buddy.summarize());
    println!("{}", infinite.summarize());
}

trait Summary {
    fn summarize(&self) -> String;
}

struct Person {
    first_name: String,
    last_name: String,
    email: String,
}

impl Summary for Person {
    fn summarize(&self) -> String {
        format!(
            "Person: {} {} at {}",
            self.first_name,
            self.last_name,
            self.email,
        )
    }
}

struct Book {
    title: String,
    author: String,
}

impl Summary for Book {
    fn summarize(&self) -> String {
        format!(
            "Book: {} by {}",
            self.title,
            self.author
        )
    }
}