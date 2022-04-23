fn main() {
    let my_number = 256;
    match my_number {
        x if x < 0 => println!("negative number!"),
        3 | 5 | 7 | 11 => println!("Small prime number!"),
        100..=200 => println!("large number"),
        _ => println!("not a special number!"),
    };

    let new_number: i32 = match my_number {
        0..=1000 => 1,
        _ => -1,
    };

    println!("{}", new_number);
}
