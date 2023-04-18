
// input is a array of numbers, output prime numbers
fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let input: Vec<i32> = buf
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();   
    let output = prime_numbers(&input);
    for number in output {
        print!("{} ", number);
    }
}

fn prime_numbers(numbers: &[i32]) -> Vec<i32> {
    let mut output = Vec::new();
    for number in numbers {
        if is_prime(*number) {
            output.push(*number);
        }
    }
    output
}

fn is_prime(number: i32) -> bool {
    if number <= 1 {
        return false;
    }
    for i in 2..number {
        if number % i == 0 {
            return false;
        }
    }
    true
}