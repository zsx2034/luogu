use std::collections::HashMap;

fn main() {
    let template = HashMap::from([
        ('A', 1),
        ('B', 2),
        ('C', 3),
        ('D', 4),
        ('E', 5),
        ('F', 6),
        ('G', 7),
        ('H', 8),
        ('I', 9),
        ('J', 10),
        ('K', 11),
        ('L', 12),
        ('M', 13),
        ('N', 14),
        ('O', 15),
        ('P', 16),
        ('Q', 17),
        ('R', 18),
        ('S', 19),
        ('T', 20),
        ('U', 21),
        ('V', 22),
        ('W', 23),
        ('X', 24),
        ('Y', 25),
        ('Z', 26),
    ]);

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let accumulator = |acc: i32, ch: char| -> i32 {
        let mut acc = acc;
        if let Some(&val) = template.get(&ch) {
            acc = (acc * val) % 47;
        }
        acc
    };

    let mut a = 1;
    buf.trim().chars().for_each(|ch| {
        a = accumulator(a, ch);
    });

    let mut b = 1;
    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().chars().for_each(|ch|{
        b = accumulator(b, ch);
    });

    if a == b {
        println!("GO");
    } else {
        println!("STAY");
    }
}
