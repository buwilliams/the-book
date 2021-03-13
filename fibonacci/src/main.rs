use std::io;

fn main() {
    let mut input = String::new();

    println!("Which fibonacci number do you want? ");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: i64 = input
        .trim()
        .parse()
        .expect("Not a number");

    println!("You asked for the fibonacci of: {}", input);

    fib(input);
}

fn fib(nth: i64) {
    let mut first: i64;
    let mut second: i64 = 0;
    let mut sum: i64 = 1;

    let mut count: i64 = 2;

    match nth {
        1 => {
            println!("Fib ({}): {}", 1, second);
        },
        _ => {
            println!("Fib ({}): {}", 1, second);
            println!("Fib ({}): {}", 2, sum);
        },
    }

    while count < nth {
        first = second;
        second = sum;
        sum = first + second;
        count = count + 1;
        println!("Fib ({}): {}", count, sum);
    }
}
