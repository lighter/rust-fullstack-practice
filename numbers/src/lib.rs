pub fn say_hello() {
    println!("Hello, world!");
}

pub fn print() {
    let numbers = [1, 2, 3, 4, 5];
    for n in numbers.iter() {
        println!("{}", n)
    }
}

pub fn print_v2(limit: u8) {
    let numbers = generate_sequence(limit);
    let numbers2 = generate_sequence_short_version(limit);
    output_sequence(&numbers);
    output_sequence(&numbers2);
}

fn output_sequence(numbers: &[u8]) {
    for n in numbers { 
        println!("{}", n);
    }
}

fn generate_sequence(limit: u8) -> Vec<u8> {
    let mut numbers = Vec::new();
    for n in 1..=limit {
        numbers.push(n);
    }
    numbers
}

fn generate_sequence_short_version(limit: u8) -> Vec<u8> {
    (1..=limit).collect()
}