fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut buf: Vec<char> = buf.trim().chars().collect();

    let mut op = if buf[buf.len() - 1] == '%' { 4 } else { 1 };
    let mut pos = 0;
    for (p, ch) in buf.iter().enumerate() {
        if ch == &'.' {
            pos = p;
            op = 2;
            break;
        } else if ch == &'/' {
            pos = p;
            op = 3;
            break;
        }
    }

    match op {
        1 => {
            let num = buf.iter().rev().collect::<String>().parse::<i128>().unwrap();
            println!("{}", num);
        },
        2 => {
            let integer = buf.iter().take(pos).rev().collect::<String>().parse::<i128>().unwrap();
            let mut zore_count = 0;
            for ch in buf.iter().skip(pos + 1) {
                if ch == &'0' {
                    zore_count += 1;
                } else {
                    break;
                }
            }
            let float = buf.iter().skip(pos + 1 + zore_count).rev().collect::<String>();
            let float = if float.is_empty() { "0".to_string() } else { float };
            println!("{}.{}", integer, float);
        },
        3 => {
            let integer = buf.iter().take(pos).rev().collect::<String>().parse::<i128>().unwrap();
            let float = buf.iter().skip(pos + 1).rev().collect::<String>().parse::<i128>().unwrap();
            print!("{}/{}", integer, float)
        },
        4 => {
            buf.pop();
            let num = buf.iter().rev().collect::<String>().parse::<i128>().unwrap();
            println!("{}%", num);
        },
        _ => {}
    }
}