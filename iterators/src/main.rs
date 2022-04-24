struct EvenNumbers {
    value: i32,
}

impl EvenNumbers {
    fn new() -> Self {
        Self { value: 2 }
    }
}

impl Iterator for EvenNumbers {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let old_value = Some(self.value);
        self.value += 2;
        old_value
    }
}

fn main() {
    let result: Vec<i32> = EvenNumbers::new()
        .skip(10)
        .map(|x| 3 * x)
        .take(10)
        .collect();
    println!("{:#?}", result);
}
