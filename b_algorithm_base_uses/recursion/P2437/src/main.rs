fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut iter = buf.split_whitespace();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let n: usize = iter.next().unwrap().parse().unwrap();

    let n = n - (m - 1);
    let mut v1 = vec![1];
    let mut v2 = vec![1];
    for i in 3..=n {
        v1.add(&v2);
        std::mem::swap(&mut v1, &mut v2);
    }

    let mut s: String = v2.iter().rev().map(|x| x.to_string()).collect();
    println!("{}", s);
}

trait Add {
    fn add(&mut self, other: &Self);
}

impl Add for Vec<usize> {
    fn add(&mut self, other: &Self) {
        let mut carry = 0;
        let mut i = 0;
        while i < self.len() {
            let sum = self[i] + other[i] + carry;
            self[i] = sum % 10;
            carry = sum / 10;
            i += 1;
        }

        while i < other.len() {
            let sum = other[i] + carry;
            self.push(sum % 10);
            carry = sum / 10;
            i += 1;
        }

        if carry > 0 {
            self.push(carry);
        }
    }
}
