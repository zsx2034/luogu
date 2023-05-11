fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i64 = buf.trim().parse().unwrap();

    match n {
        0 => {
            println!("0");
            return;
        },
        1 => {
            println!("1");
            return;
        },
        2 => {
            println!("2");
            return;
        },
        _ => {}
    }

    let mut dp1 = vec![1];
    let mut dp2 = vec![2];
    for i in 3..=n {
        dp1.add(&dp2);
        std::mem::swap(&mut dp1, &mut dp2);
    }

    dp2.display();
}

trait BigIntegerAdd {
    fn add(&mut self, other: &Vec<i32>);
    fn display(&self);
}

impl BigIntegerAdd for Vec<i32> {
    fn add(&mut self, other: &Vec<i32>){
        let mut carry = 0;
        let mut i = 0;
        while i < self.len() || i < other.len() {
            let mut sum = carry;
            if i < self.len() {
                sum += self[i];
            }
            if i < other.len() {
                sum += other[i];
            }
            if sum >= 10 {
                carry = 1;
                sum -= 10;
            } else {
                carry = 0;
            }
            if i < self.len() {
                self[i] = sum;
            } else {
                self.push(sum);
            }
            i += 1;
        }
        if carry > 0 {
            self.push(carry);
        }
    }

    fn display(&self) {
        let mut result = String::new();
        for i in (0..self.len()).rev() {
            result.push_str(&self[i].to_string());
        }
        println!("{}", result);
    }
}
