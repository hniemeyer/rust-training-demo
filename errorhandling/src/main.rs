use eyre::Result;

fn lookup_data(key: &str) -> Result<i32> {
    if key.len() > 8 {
        Err(eyre::eyre!(
            "Only keys with less than 8 characters allowed."
        ))
    } else {
        Ok(key.len() as i32)
    }
}

fn add_unsafe_data(first_key: &str, second_key: &str) -> Result<i32> {
    let first_data = lookup_data(first_key)?;
    let second_data = lookup_data(second_key)?;
    Ok(first_data + second_data)
}

fn main() {
    let my_data = lookup_data("hallo").expect("Receiving the key did not work.");
    println!("{}", my_data);
    let my_result = add_unsafe_data("hallo", "xxxxx").expect("addition did not work");
    println!("{}", my_result);
}
