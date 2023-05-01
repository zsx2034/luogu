fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();
    let mut points: Vec<Point> = Vec::with_capacity(n);
    for _ in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        points.push(buf.trim().split_whitespace().collect());
    }

    points.sort_by(|a, b| a.z.partial_cmp(&b.z).unwrap());

    let mut distance = 0.0;
    for i in 0..n - 1 {
        distance += points[i].distance(&points[i + 1]);
    }

    println!("{:.3}", distance);
    // println!("{:.3}", (distance + 0.0005) * 1000.0 as i64 as f64 / 1000.0);
}

struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Point { x, y, z }
    }

    fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

impl<'a> FromIterator<&'a str> for Point {
    fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        let x: f64 = iter.next().unwrap().parse().unwrap();
        let y: f64 = iter.next().unwrap().parse().unwrap();
        let z: f64 = iter.next().unwrap().parse().unwrap();
        Point::new(x, y, z)
    }
}
