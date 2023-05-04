fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut iter = buf.split_whitespace();
    let row: usize = iter.next().unwrap().parse().unwrap();
    let col: usize = iter.next().unwrap().parse().unwrap();

    let mut w = vec![col; row];
    let mut b = vec![col; row];
    let mut r = vec![col; row];
    for i in 0..row {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let mut chars = buf.trim().chars();
        for _ in 0..col {
            let c: char = chars.next().unwrap();
            match c {
                'W' => w[i] -= 1,
                'B' => b[i] -= 1,
                'R' => r[i] -= 1,
                _ => (),
            }
        }
    }

    let mut min = std::usize::MAX;
    let mut sum = 0;
    for i in 1..row - 1 {
        for j in 1..row - i {
            assert!(i + j < row);
            sum = 0;
            for l in 0..i {
                sum += w[l];
            }
            for l in i..i + j {
                sum += b[l];
            }
            for l in i + j..row {
                sum += r[l];
            }

            if sum < min {
                min = sum;
            }
        }
    }

    println!("{}", min);
}

// fn dp(i: usize, depth: usize, d: &mut Dp_state) {
//     if depth == 3 && i == d.row {
//         if d.sum < d.cost {
//             d.cost = d.sum;
//         }
//         return;
//     }
//     for j in d.row - 1 - i.. {
//         match i {
//             0 => {
//                 d.sum += d.w[j];
//                 dp(j + 1, depth + 1, d);
//                 d.sum -= d.w[j];
//             }
//             1 => {
//                 d.sum += d.b[j];
//                 dp(j + 1, depth + 1, d);
//                 d.sum -= d.b[j];
//             }
//             2 => {
//                 d.sum += d.r[j];
//                 dp(j + 1, depth + 1, d);
//                 d.sum -= d.r[j];
//             }
//             _ => (),
//         }
//     }
// }

// struct Dp_state {
//     w: Vec<usize>,
//     b: Vec<usize>,
//     r: Vec<usize>,
//     row: usize,
//     col: usize,
//     sum: usize,
//     cost: usize,
// }
