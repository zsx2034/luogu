fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let row: usize = iter.next().unwrap().parse().unwrap();
    let col: usize = iter.next().unwrap().parse().unwrap();

    let mut v = vec![vec![0; col]; row];
    for row in 0..row {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        for (col, ch) in buf.chars().enumerate() {
            if ch == '.' {
                v[row][col] = -1;
            }
        }
    }

    let mut ans = 1;
    for i in 0..row {
        for j in 0..col {
            if v[i][j] == 0 {
                search(row, col, &mut v, ans, i, j);
                ans += 1;
            }
        }
    }

    println!("{}", ans - 1);
}

fn display(v: &Vec<Vec<i32>>) {
    for row in v {
        for col in row {
            print!("{} ", col);
        }
        println!();
    }
    println!();
}

fn search(row: usize, col: usize, map: &mut Vec<Vec<i32>>, tag: i32, i: usize, j: usize) {
    let mut q = std::collections::VecDeque::new();
    q.push_back((i, j));
    while !q.is_empty() {
        let (current_row, current_col) = q.pop_front().unwrap();
        map[current_row][current_col] = tag;
        for direct in 1..=8 {
            let next = next_point(row, col, current_row, current_col, direct);
            if let Some((next_row, next_col)) = next {
                if map[next_row][next_col] == 0 {
                    map[next_row][next_col] = tag;
                    q.push_back((next_row, next_col));
                }
            }
        }
    }
}

fn next_point(
    row: usize,
    col: usize,
    current_row: usize,
    current_col: usize,
    direct: usize,
) -> Option<(usize, usize)> {
    match direct {
        // up
        1 => {
            if current_row > 0 {
                Some((current_row - 1, current_col))
            } else {
                None
            }
        }
        // up right
        2 => {
            if current_row > 0 && current_col + 1 < col {
                Some((current_row - 1, current_col + 1))
            } else {
                None
            }
        }
        // right
        3 => {
            if current_col + 1 < col {
                Some((current_row, current_col + 1))
            } else {
                None
            }
        }
        // down right
        4 => {
            if current_row + 1 < row && current_col + 1 < col {
                Some((current_row + 1, current_col + 1))
            } else {
                None
            }
        }
        // down
        5 => {
            if current_row + 1 < row {
                Some((current_row + 1, current_col))
            } else {
                None
            }
        }
        // down left
        6 => {
            if current_row + 1 < row && current_col > 0 {
                Some((current_row + 1, current_col - 1))
            } else {
                None
            }
        }
        // left
        7 => {
            if current_col > 0 {
                Some((current_row, current_col - 1))
            } else {
                None
            }
        }
        // up left
        8 => {
            if current_row > 0 && current_col > 0 {
                Some((current_row - 1, current_col - 1))
            } else {
                None
            }
        }
        _ => None,
    }
}
