fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut iter = buf.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let t: f64 = iter.next().unwrap().parse().unwrap();

    let mut items = Vec::with_capacity(n);
    for _ in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        
        let item = buf.split_whitespace().collect::<Item>();
        items.push(item);
    }

    items.sort_by(|a, b| b.portion.partial_cmp(&a.portion).unwrap());

    let mut total_weight = 0.0;
    let mut total_value = 0.0;
    for item in items {
        if total_weight + item.weight <= t {
            total_weight += item.weight;
            total_value += item.value;
        } else {
            let remain = t - total_weight;
            total_value += remain * item.portion;
            break;
        }
    }
    println!("{:.2}", total_value);
}


struct Item {
    weight: f64,
    value: f64,
    portion: f64,
}

impl Item {
    fn new(weight: f64, value: f64) -> Self {
        Self {
            weight,
            value,
            portion: value as f64 / weight as f64,
        }
    }
}

impl<'a> FromIterator<&'a str> for Item {
    fn from_iter<I: IntoIterator<Item=&'a str>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        let weight: f64 = iter.next().unwrap().parse().unwrap();
        let value: f64 = iter.next().unwrap().parse().unwrap();

        Self::new(weight, value)
    }
}