fn main() {
    let mut template = std::collections::HashMap::from([
        ("zero", 0),
        ("one", 1),
        ("two", 4),
        ("three", 9),
        ("four", 16),
        ("five", 25),
        ("six", 36),
        ("seven", 49),
        ("eight", 64),
        ("nine", 81),
        ("ten", 0),
        ("eleven", 21),
        ("twelve", 44),
        ("thirteen", 69),
        ("fourteen", 96),
        ("fifteen", 25),
        ("sixteen", 56),
        ("seventeen", 89),
        ("eighteen", 24),
        ("nineteen", 61),
        ("twenty", 0),
        ("a", 1),
        ("both", 4),
        ("another", 1),
        ("first", 1),
        ("second", 4),
        ("third", 9),
    ]);

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let mut buffer = buffer.trim().to_string();
    if &buffer[buffer.len() - 1..] == "." {
        buffer.pop();
    }

    let mut numbers = Vec::new();
    let mut words = buffer.split_whitespace();
    for word in words {
        if let Some(value) = template.get(word) {
            numbers.push(value);
        }
    }

    numbers.sort();

    let mut flag = true;
    for ele in numbers {
        if ele != &0 {
            if flag {
                print!("{}", ele);
                flag = false;
            } else {
                print!("{:02}", ele);
            }
        }
    }

    if flag {
        println!("0");
    }
}
