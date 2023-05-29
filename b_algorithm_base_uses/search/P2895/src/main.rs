use std::collections::VecDeque;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    let mut is_safe = vec![vec![usize::MAX; 401]; 401];
    for _ in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let mut iter = buf.split_whitespace();
        let a: usize = iter.next().unwrap().parse().unwrap();
        let b: usize = iter.next().unwrap().parse().unwrap();
        let c: usize = iter.next().unwrap().parse().unwrap();
        change_safe_state(&mut is_safe, a, b, c)
    }

    let mut walked = vec![vec![false; 401]; 401];
    walked[0][0] = true;
    
    if is_safe[0][0] == usize::MAX {
        println!("0");
        return;
    }
    
    let mut t: usize = 0;
    let mut q1: VecDeque<(usize, usize)> = VecDeque::from(vec![(0, 0)]);
    let mut q2: VecDeque<(usize, usize)> = VecDeque::new();
    while !q1.is_empty() {
        while let Some((a, b)) = q1.pop_front() {
            // println!("{} {} {}", a, b, t);
            for d in 1..=4 {
                if let Some((i, j)) = move_to(a, b, d) {
                    if is_safe[i][j] > t + 1 && !walked[i][j] {
                        if (i == 400 && j == 400) || is_safe[i][j] == usize::MAX {
                            println!("{}", t + 1);
                            return;
                        }
                        walked[i][j] = true;
                        q2.push_back((i, j));
                    }
                }
            }
        }
        swap(&mut q1, &mut q2);
        t += 1;
    }

    println!("-1");
}

fn swap<T>(a: &mut T, b: &mut T) {
    unsafe {
        std::ptr::swap(a, b);
    }
}

fn change_safe_state(isSafe: &mut Vec<Vec<usize>>, a: usize, b: usize, c: usize) {
    isSafe[a][b] = std::cmp::min(isSafe[a][b], c);

    if a > 0 {
        isSafe[a - 1][b] = std::cmp::min(isSafe[a - 1][b], c);
    }
    if a < 400 {
        isSafe[a + 1][b] = std::cmp::min(isSafe[a + 1][b], c);
    }
    if b > 0 {
        isSafe[a][b - 1] = std::cmp::min(isSafe[a][b - 1], c);
    }
    if b < 400 {
        isSafe[a][b + 1] = std::cmp::min(isSafe[a][b + 1], c);
    }
}

fn move_to(i: usize, j: usize, direct: usize) -> Option<(usize, usize)> {
    match direct {
        // up
        1 => {
            if i > 0 {
                return Some((i - 1, j));
            }
        }
        // right
        2 => {
            if j < 400 {
                return Some((i, j + 1));
            }
        }
        // down
        3 => {
            if i < 400 {
                return Some((i + 1, j));
            }
        }
        // left
        4 => {
            if j > 0 {
                return Some((i, j - 1));
            }
        }
        _ => {}
    }
    None
}
