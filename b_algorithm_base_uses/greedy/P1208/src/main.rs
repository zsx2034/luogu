fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let mut total: u64 = iter.next().unwrap().parse().unwrap();
    let n: usize = iter.next().unwrap().parse().unwrap();

    let mut farmers: Vec<Farmer> = Vec::with_capacity(n);
    for _ in 0..n {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let mut iter = buf.split_whitespace();
        let price: u64 = iter.next().unwrap().parse().unwrap();
        let amount: u64 = iter.next().unwrap().parse().unwrap();
        farmers.push(Farmer::new(price, amount));
    }

    farmers.sort_by(|a, b| a.price.cmp(&b.price));

    let mut min_cost = 0;
    for farmer in farmers {
        if total == 0 {
            break;
        }
        let amount = if total > farmer.amount {
            farmer.amount
        } else {
            total
        };
        min_cost += amount * farmer.price;
        total -= amount;
    }

    println!("{}", min_cost);
}

struct Farmer {
    price: u64,
    amount: u64
}

impl Farmer {
    fn new(price: u64, amount: u64) -> Self {
        Self {
            price,
            amount
        }
    }
}
