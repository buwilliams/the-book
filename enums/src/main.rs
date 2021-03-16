enum Money {
    Dollar(i32),
    Quarter(i32),
    Dime(i32),
    Nickle(i32),
    Penny(i32)
}

fn main() {
    let amount = Some(Money::Dollar(100));
    if let Some(Money::Dollar(100)) = amount {
        println!("We have 100 dollars!");
    }

}
