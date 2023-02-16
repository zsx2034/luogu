
// 线段树
fn main() {
    
}

// 模拟方式
fn solution1() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("ms");
    let mut input = buf.split_whitespace();      
    let n:i64 = input.next().unwrap().parse().unwrap();
    let tree = vec![true;n+1];
    let area_number: i64 = input.next().unwrap().parse().unwrap();
    for _ in 0..area_number {
        buf.clear();
        std::io::stdin().read_line(&mut buf).expect("ms");
        input = buf.split_whitespace();
        let a = input.next().unwrap().parse::<i64>().unwrap();
        let b = input.next().unwrap().parse::<i64>().unwrap();
        for i in a..b+1 {
            tree[i] = false;
        }
    }

    let mut sum = 0;
    for _ in 0..area_number {
        if tree[i] {
            sum += 1;
        }
    }
    println!("{}", sum);
}
