
// calculate the distance of two points
fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x1 - x2).powf(2.0) + (y1 - y2).powf(2.0)).sqrt()
}

fn main() {
    let mut buf = String::new();
    let mut points = Vec::new();
    for _ in 0..3 {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let mut point = buf.split_whitespace().map(|x| x.parse::<f64>().unwrap()).collect::<Vec<f64>>();
        points.push(point);
    }

    let mut total = 0.0;
    for i in 0..3 {
        let j = (i + 1) % 3;
        total += distance(points[i][0], points[i][1], points[j][0], points[j][1]);
    }

    println!("{:.02}", total);
}
