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

    let sum = (1..100).fold(0, |acc, x| acc + 2 * x);
    println!("{}", sum);

    let my_result: i32 = itertools::izip!(EvenNumbers::new(), 1..)
        .map(|x| x.0 + x.1)
        .take(10)
        .sum();

    println!("{}", my_result);

    let nums = (1..100).collect::<Vec<i32>>();
    println!("{}", nums[0]);
}
