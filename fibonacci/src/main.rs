use std::io;

fn main() {
    loop {
        let mut input = String::new();

        println!("Which fibonacci number do you want? ");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: u64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You asked for the fibonacci of: {}", input);

        if input > 94 { 
            println!("The maximum fibonacci is 94 for u64 scalar data type.");
            continue;
        }

        fib(input);
    }
}

fn fib(nth: u64) {
    let mut first: u64;
    let mut second: u64 = 0;
    let mut sum: u64 = 1;

    let mut count: u64 = 2;

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
