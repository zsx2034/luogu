use std::io;

fn main() {
    // 定义三个变量并初始化为0
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;

    // 创建一个字符串变量用于存储输入
    let mut code = String::new();

    // 从标准输入读取一行并存储到code变量中
    io::stdin()
        .read_line(&mut code)
        .expect("Failed to read line");

    // 按照分号分割代码为多个赋值语句
    let statements = code.split(';');

    // 对每个赋值语句进行处理
    for statement in statements {
        // 如果语句为空，跳过
        if statement.is_empty() {
            continue;
        }
        // 按照:=分割语句为左右两部分
        let parts = statement.split(":=").collect::<Vec<&str>>();
        // 如果分割后的长度不是2，说明语句格式不正确，报错并退出
        if parts.len() != 2 {
            println!("Invalid statement: {}", statement);
            break;
        }
        // 获取左右两部分的内容
        let left = parts[0];
        let right = parts[1];
        // 根据左边的变量名，将右边的值赋给对应的变量
        match left {
            "a" => {
                // 如果右边是一个一位数字，将其转换为整数并赋值
                if right.chars().all(|c| c.is_digit(10)) {
                    a = right.parse::<i32>().unwrap();
                } else {
                    // 否则，根据右边的变量名，将其值赋给左边的变量
                    match right {
                        "a" => a = a,
                        "b" => a = b,
                        "c" => a = c,
                        // 如果右边不是一个合法的变量名，报错并退出
                        _ => {
                            println!("Invalid variable: {}", right);
                            break;
                        }
                    }
                }
            }
            "b" => {
                if right.chars().all(|c| c.is_digit(10)) {
                    b = right.parse::<i32>().unwrap();
                } else {
                    match right {
                        "a" => b = a,
                        "b" => b = b,
                        "c" => b = c,
                        _ => {
                            println!("Invalid variable: {}", right);
                            break;
                        }
                    }
                }
            }
            "c" => {
                if right.chars().all(|c| c.is_digit(10)) {
                    c = right.parse::<i32>().unwrap();
                } else {
                    match right {
                        "a" => c = a,
                        "b" => c = b,
                        "c" => c = c,
                        _ => {
                            println!("Invalid variable: {}", right);
                            break;
                        }
                    }
                }
            }
            // 如果左边不是一个合法的变量名，报错并退出
            _ => {
                println!("Invalid variable: {}", left);
                break;
            }
        }
    }

    // 输出a,b,c的值
    println!("a={}, b={}, c={}", a, b, c);
}
