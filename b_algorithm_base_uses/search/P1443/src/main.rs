use std::collections::VecDeque;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let row: usize = iter.next().unwrap().parse().unwrap();
    let col: usize = iter.next().unwrap().parse().unwrap();

    let mut y: usize = iter.next().unwrap().parse().unwrap();
    let mut x: usize = iter.next().unwrap().parse().unwrap();

    x -= 1;
    y -= 1;

    let mut map = vec![vec![-1; col]; row];
    let mut point = VecDeque::from(vec![(x, y)]);
    map[y][x] = 0;
    while let Some((x, y)) = point.pop_front() {
        for i in 1..=8 {
            if let Some((nx, ny)) = move_to(row, col, x, y, i) {
                if map[ny][nx] == -1 {
                    map[ny][nx] = map[y][x] + 1;
                    point.push_back((nx, ny));
                }
            }
        }
    }

    for i in 0..row {
        for j in 0..col {
            print!("{:<5} ", map[i][j]);
        }
        println!();
    }

}

fn move_to(row: usize, col: usize, x: usize, y: usize, direct: usize) -> Option<(usize, usize)> {
    match direct {
        // up left
        1 => {
            if x >= 1 && y >= 2 {
                Some((x - 1, y - 2))
            } else {
                None
            }
        }
        // up right
        2 => {
            if x < col - 1 && y >= 2 {
                Some((x + 1, y - 2))
            } else {
                None
            }
        }
        // right up
        3 => {
            if x < col - 2 && y >= 1 {
                Some((x + 2, y - 1))
            } else {
                None
            }
        }
        // right down
        4 => {
            if x < col - 2 && y < row - 1 {
                Some((x + 2, y + 1))
            } else {
                None
            }
        }
        // down right
        5 => {
            if x < col - 1 && y < row - 2 {
                Some((x + 1, y + 2))
            } else {
                None
            }
        }
        // down left
        6 => {
            if x >= 1 && y < row - 2 {
                Some((x - 1, y + 2))
            } else {
                None
            }
        }
        // left down
        7 => {
            if x >= 2 && y < row - 1 {
                Some((x - 2, y + 1))
            } else {
                None
            }
        }
        // left up
        8 => {
            if x >= 2 && y >= 1 {
                Some((x - 2, y - 1))
            } else {
                None
            }
        }
        _ => {
            panic!("Invalid direction");
        }
    }
}
