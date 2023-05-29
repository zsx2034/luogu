use std::collections::VecDeque;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    let mut map: Vec<Vec<i32>> = vec![vec![0; n]; n];

    for i in 0..n {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let mut iter = buf.trim().split_whitespace();
        for j in 0..n {
            map[i][j] = iter.next().unwrap().parse().unwrap();
        }
    }

    let mut tag = 5;
    for i in 0..n {
        for j in 0..n {
            if map[i][j] == 0 {
                let mut res = true;
                bfs(&mut map, i, j, tag, &mut res);
                if res {
                    display(&map, tag);
                    return;
                }
                tag += 1;
            }
        }
    }

    // println!();
    // for i in 0..n {
    //     for j in 0..n {
    //         print!("{} ", map[i][j]);
    //     }
    //     println!();
    // }
    
}

fn display(map: &Vec<Vec<i32>>, tag: i32) {
    for i in 0..map.len() {
        for j in 0..map.len() {
            if map[i][j] == tag {
                print!("2 ");
            } else if map[i][j] == 1 {
                print!("1 ");
            } else {
                print!("0 ");
            }
        }
        println!();
    }
}

fn bfs(map: &mut Vec<Vec<i32>>, x: usize, y: usize, tag: i32, res: &mut bool) {
    let mut q = VecDeque::new();
    q.push_back((x, y));
    while let Some((x, y)) = q.pop_front() {
        map[x][y] = tag;

        for i in 0..=3 {
            if let Some((nx, ny)) = next_direct(x, y, map.len(), i) {
                if map[nx][ny] == 0 {
                    map[nx][ny] = tag;
                    q.push_back((nx, ny));
                } 
            } else {
                *res = false;
            }
        }
    }
}

fn next_direct(x: usize, y: usize, n: usize, direct: usize) -> Option<(usize, usize)> {
    match direct {
        // up
        0 => {
            if x == 0 {
                None
            } else {
                Some((x - 1, y))
            }
        }
        // down
        1 => {
            if x == n - 1 {
                None
            } else {
                Some((x + 1, y))
            }
        }
        // left
        2 => {
            if y == 0 {
                None
            } else {
                Some((x, y - 1))
            }
        }
        // right
        3 => {
            if y == n - 1 {
                None
            } else {
                Some((x, y + 1))
            }
        }
        _ => None,
    }
}
