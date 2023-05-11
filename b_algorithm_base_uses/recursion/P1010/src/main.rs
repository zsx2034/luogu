use std::fmt::format;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut num = buf.trim().parse::<u32>().unwrap();

    let mut res = dp(num);

    if res.ends_with("+") {
        res.pop();
    }

    println!("{}", res); 
}

fn dp(mut num: u32) -> String {
    if num > 7 {
        let mut res = "".to_string();
        while num > 7 {
            let tmp = (num as f64).log2().floor() as u32; 
            num -= 2u32.pow(tmp);
            if num > 0 {
                res.push_str(&format!("2({})", &dp(tmp)));
                res.push_str("+");
            } else {
                res.push_str(&format!("2({})", &dp(tmp)));
            }
        }
        if num > 0 {
            res.push_str(&dp(num));
        }
        return res;
    } else {
        match num {
            0 => return "0".to_string(),
            1 => return "2(0)".to_string(),
            2 => return "2".to_string(),
            3 => return "2+2(0)".to_string(),
            4 => return "2(2)".to_string(),
            5 => return "2(2)+2(0)".to_string(),
            6 => return "2(2)+2".to_string(),
            7 => return "2(2)+2+2(0)".to_string(),
            _ => return "".to_string(),
        }
    }
}
