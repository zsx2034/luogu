
/// ```
///     /\
///    /__\
///   /\  /\
///  /__\/__\
/// ```
/// 一个三角形占用的空间是 宽4，高2
fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut n = buf.trim().parse::<i32>().unwrap();

    let width = 2_usize.pow(n as u32 - 1) * 4;
    let height = 2_usize.pow(n as u32 - 1) * 2;

    let mut map: Vec<Vec<char>> = vec![vec![' '; width];height];

    draw_all(&mut map, 0, height - 1, n as usize);

    for row in map.iter() {
        for col in row.iter() {
            print!("{}", col);
        }
        println!();
    }
}

fn draw_all(map: &mut Vec<Vec<char>>, col: usize, row: usize, n: usize) {
    if n == 1 {
        draw(map, col, row);
    } else {
        let width = 2_usize.pow(n as u32 - 1) * 4;
        let height = 2_usize.pow(n as u32 - 1) * 2;

        draw_all(map, col, row, n-1);
        draw_all(map, col+width/2, row, n-1);
        draw_all(map, col+width/4, row-height/2, n-1);
    }
}


fn draw(map: &mut Vec<Vec<char>>, col: usize, row: usize) {
    map[row][col] = '/';
    map[row][col+1] = '_';
    map[row][col+2] = '_';
    map[row][col+3] = '\\';
    map[row-1][col+1] = '/';
    map[row-1][col+2] = '\\';
}
