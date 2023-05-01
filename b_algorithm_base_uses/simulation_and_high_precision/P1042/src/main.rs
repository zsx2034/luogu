fn main() {
    let mut buf = String::new();

    let mut method_11 = Method11::new();
    let mut method_21 = Method21::new();
    'end: loop {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        for ch in buf.trim().chars() {
            if ch == 'E' {
                method_11.add_score(ch);
                method_21.add_score(ch);
                break 'end;
            } else {
                method_11.add_score(ch);
                method_21.add_score(ch);
            }
        }
    }
    method_11.display();
    println!();
    method_21.display();
}

trait AddScore {
    fn add_score(&mut self, score: char);
}

// 11
struct Method11 {
    score: Vec<Score>,
    times: usize,
}

impl Method11 {
    fn new() -> Self {
        Self {
            score: vec![Score::new(11)],
            times: 0,
        }
    }

    fn display(&self) {
        for m in &self.score {
            m.display();
        }
    }
}

impl AddScore for Method11 {
    fn add_score(&mut self, score: char) {
        if self.score[self.times].add_score(score) {
        } else {
            self.times += 1;
            self.score.push(Score::new(11));
            self.score[self.times].add_score(score);
        }
    }
}

// 21
struct Method21 {
    score: Vec<Score>,
    times: usize,
}

impl Method21 {
    fn new() -> Self {
        Self {
            score: vec![Score::new(21)],
            times: 0,
        }
    }

    fn display(&self) {
        for m in &self.score {
            m.display();
        }
    }
}

impl AddScore for Method21 {
    fn add_score(&mut self, score: char) {
        if self.score[self.times].add_score(score) {
        } else {
            self.times += 1;
            self.score.push(Score::new(21));
            self.score[self.times].add_score(score);
        }
    }
}

struct Score {
    a: i32,
    b: i32,
    base: i32,
}

impl Score {
    fn new(base: i32) -> Self {
        Self { a: 0, b: 0, base }
    }

    fn add_score(&mut self, score: char) -> bool {
        if (self.a >= self.base || self.b >= self.base) &&(self.a - self.b).abs() >= 2 {
            return false;
        }
        match score {
            'W' => self.a += 1,
            'L' => self.b += 1,
            _ => {}
        }
        return true;
    }

    fn display(&self) {
        println!("{}:{}", self.a, self.b);
    }
}
