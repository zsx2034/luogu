fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let n = iter.next().unwrap().parse::<usize>().unwrap();
    let m = iter.next().unwrap().parse::<usize>().unwrap();

    let mut max = 0.0;
    for _ in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let mut scores = buf.split_whitespace().map(|x| x.parse::<f32>().unwrap()).collect::<Vec<_>>();
        let avg = scores.solve();
        if avg > max {
            max = avg;
        }
    } 
    println!("{:.2}", max);
}

trait Solution {
    fn solve(&self) -> f32;
}

impl Solution for Vec<f32> {
    fn solve(&self) -> f32 {
        let mut sum = 0.0;
        let mut min = 20.0;
        let mut max = 0.0;
        for i in 0..self.len() {
            sum += self[i];
            if self[i] < min {
                min = self[i];
            }
            if self[i] > max {
                max = self[i];
            }
        }
        (sum - min - max) / (self.len() - 2) as f32
    }
}

