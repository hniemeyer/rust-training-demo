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

fn main() {
    let my_data = lookup_data("hallo").expect("Receiving the key did not work.");
    println!("{}", my_data);
}
