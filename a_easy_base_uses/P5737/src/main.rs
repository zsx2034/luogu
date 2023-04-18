
// output the leap year between a and b(contains a and b)
fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut year_range = buf.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let mut year = year_range[0];
    let mut res = Vec::new();
    while year <= year_range[1] {
        if year % 4 == 0 && year % 100 != 0 || year % 400 == 0 {
            res.push(year);
        }
        year += 1;
    }

    println!("{}", res.len());
    for i in 0..res.len() {
        print!("{} ", res[i]);
    }
}
