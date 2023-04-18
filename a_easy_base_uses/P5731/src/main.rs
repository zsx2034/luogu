fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<usize>().unwrap();

    let tmp = n * n;
    let mut direction = 0;
    let mut al = 0;
    let mut ar = n - 1;
    let mut bl = 0;
    let mut br = n - 1;
    let mut x = 0;
    let mut y = 0;
    let mut res = vec![vec![0; n]; n];
    for i in 1..tmp+1 {
        res[y][x] = i;
        match direction {
            0 => {
                if x < ar {
                    x += 1;
                } else {
                    direction = 1;
                    bl += 1;
                    y += 1;
                }
            }
            1 => {
                if y < br {
                    y += 1;
                } else {
                    direction = 2;
                    ar -= 1;
                    x -= 1;
                }
            }
            2 => {
                if x > al {
                    x -= 1;
                } else {
                    direction = 3;
                    br -= 1;
                    y -= 1;
                }
            },
            3 => {
                if y > bl {
                    y -= 1;
                } else {
                    direction = 0;
                    al += 1;
                    x += 1;
                }
            },
            _ => break,
        }
    }

    for line in res {
        for item in line {
            print!("{:3}", item);
        }
        println!();
    }
}
