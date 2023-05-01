fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("msg");
    let n = buf.trim().parse::<f64>().unwrap();

    buf.clear();
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("msg");
    let scores:Vec<f64> = buf.split_whitespace().map(|num | num.trim().parse().unwrap()).collect();

    let mut final_score = 0.0_f64;
    let mut max = 0.0;
    let mut min = f64::MAX;
    for score in scores {
        if score > max { max = score; }
        if score < min { min = score; }
        final_score += score;
    } 

    final_score = ((final_score - min - max) / (n - 2.0) * 100.0).round() / 100.0;
    println!("{:.2}", final_score); 
}
