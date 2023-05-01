fn main() {
    let mut buf = String::new();
    let mut map: Vec<Vec<char>> = Vec::with_capacity(10);
    for _ in 0..10 {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        map.push(buf.trim().chars().collect());
    }

    let mut c = Role::new();
    let mut f = Role::new();
    for i in 0..10 {
        for j in 0..10 {
            if map[i][j] == 'C' {
                map[i][j] = '.';
                c.set_position(j, i);
            }

            if map[i][j] == 'F' {
                map[i][j] = '.';
                f.set_position(j, i);
            }
        }
    }

    let mut time = 0;
    let mut status = vec![false; 1000000];
    loop {
        if c.same(&f) {
            break;
        }
        let h = c.hash() * 1000 + f.hash();
        if status[h] {
            time = 0;
            break;
        }
        status[h] = true;
        c.step(&map);
        f.step(&map);
        time += 1;
    }

    println!("{}", time);
}

struct Role {
    x: usize,
    y: usize,
    direction: usize,
}

impl Role {
    fn new() -> Self {
        Role {
            x: 0,
            y: 0,
            direction: 0,
        }
    }

    fn set_position(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }

    fn rotate(&mut self) {
        self.direction = (self.direction + 1) % 4;
    }

    fn step(&mut self, map: &[Vec<char>]) {
        match self.direction {
            0 => {
                if self.y > 0 && map[self.y - 1][self.x] == '.' {
                    self.y -= 1;
                } else {
                    self.rotate();
                }
            }
            1 => {
                if self.x < 9 && map[self.y][self.x + 1] == '.' {
                    self.x += 1;
                } else {
                    self.rotate();
                }
            }
            2 => {
                if self.y < 9 && map[self.y + 1][self.x] == '.' {
                    self.y += 1;
                } else {
                    self.rotate();
                }
            }
            3 => {
                if self.x > 0 && map[self.y][self.x - 1] == '.' {
                    self.x -= 1;
                } else {
                    self.rotate();
                }
            }
            _ => {}
        }
    }

    fn same(&self, other: &Role) -> bool {
        self.x == other.x && self.y == other.y
    }

    fn hash(&self) -> usize {
        self.x + self.y * 10 + self.direction * 100
    }

    fn display(&self) {
        println!("x: {}, y: {}, direction: {}", self.x, self.y, match self.direction {
            0 => "北",
            1 => "东",
            2 => "南",
            3 => "西",
            _ => "Error"
        })
    }
}