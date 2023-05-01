fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    for _ in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let score = buf.trim().split_whitespace().collect::<Score>();
        score.judge();
    }
}

struct Score {
    id: String,
    a: u32,
    b: u32,
} 

impl Score {
    fn new(id: String, a: u32, b: u32) -> Score {
        Score {
            id: id,
            a: a,
            b: b,
        }
    }

    fn judge(&self) {
        if self.a + self.b > 140 && self.a * 7 + self.b * 3 >= 800 {
            println!("Excellent");
        } else {
            println!("Not excellent");
        }
    }
}

impl<'a> FromIterator<&'a str> for Score {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let id = iter.next().unwrap().to_string();
        let a = iter.next().unwrap().parse().unwrap();
        let b = iter.next().unwrap().parse().unwrap();
        Score::new(id, a, b)
    }
}