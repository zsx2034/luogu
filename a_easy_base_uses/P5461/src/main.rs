fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n = buf.trim().parse::<u32>().unwrap();
    let len = 2_usize.pow(n);
    let mut map = vec![vec![1; len]; len];

    release(&mut map, 0, 0, 2_usize.pow(n));

    for i in 0..len {
        for j in 0..len {
            if j != len - 1 {
                print!("{} ", map[i][j]);
            } else {
                print!("{}", map[i][j]);
            }
        }
        println!();
    }
}

fn release(map: &mut Vec<Vec<u32>>, x: usize, y: usize, len: usize) {
    if len == 1 {
        // map[y][x] = 0;
        return;
    }

    let len = len / 2;
    for i in 0..len {
        for j in 0..len {
            map[y + i][x + j] = 0;
        }
    }

    // right top
    release(map, x + len, y, len);
    // left bottom
    release(map, x, y + len, len);
    // right bottom
    release(map, x + len, y + len, len);
}
