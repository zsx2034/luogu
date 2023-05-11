fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n: u32 = buf.trim().parse().unwrap();

    match n {
        1 => {
            println!("1");
            return;
        }
        2 => {
            println!("2");
            return;
        }
        _ => (),
    }

    let res = solve(n);
    let mut flag = true;
    for i in res.iter().rev() {
        if *i == 0 && flag {
            continue;
        }
        flag = false;
        print!("{}", i);
    }
}

fn solve(n: u32) -> Vec<u32> {
    let mut res = vec![1];

    for i in n + 2..=2 * n {
        res.mul(i);
    }

    for i in 1..=n {
        res.div(i);
    }
    res
}

trait BigInt {
    fn mul(&mut self, other: u32);
    fn div(&mut self, other: u32);
}

impl BigInt for Vec<u32> {
    fn mul(&mut self, other: u32) {
        for i in 0..self.len() {
            self[i] *= other;
        }

        for i in 0..self.len() - 1 {
            if self[i] >= 10 {
                self[i + 1] += self[i] / 10;
                self[i] %= 10;
            }
        }

        while self[self.len() - 1] >= 10 {
            let tmp: u32 = self[self.len() - 1] / 10;
            let i = self.len() - 1;
            self[i] %= 10;
            self.push(tmp);
        }
    }

    fn div(&mut self, other: u32) {
        let mut carry: u32 = 0;
        for i in (0..self.len()).rev() {
            let tmp: u32 = self[i] + carry * 10;
            self[i] = tmp / other;
            carry = tmp % other;
        }

        // while self.len() > 1 && self[self.len() - 1] == 0 {
        //     self.pop();
        // }
    }
}

#[cfg(test)]
mod test {
    use crate::BigInt;
    #[test]
    fn test_div() {
        let mut v: Vec<u32> = vec![4, 2, 3, 4, 2];
        v.div(12_u32);
        assert_eq!(v, vec![7, 2, 0, 2]);
    }
}
