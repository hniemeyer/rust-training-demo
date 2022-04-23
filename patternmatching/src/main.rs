struct MyData {
    x: i32,
    y: i32,
    z: i32,
}

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

    let data = MyData { x: 1, y: 2, z: 3 };

    match data {
        MyData { x, y, z } if x + y + z == 12 => println!("The sum is 12"),
        MyData { x, y, z: _ } if x > y => println!("x is bigger than y"),
        MyData { x: _, y: _, z } if z == 3 => println!("z is equal to 3"),
        _ => println!("Something else"),
    }
}
