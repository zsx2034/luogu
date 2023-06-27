use std::collections::VecDeque;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let buf: Vec<char> = buf.trim().chars().collect();

    let mut st: VecDeque<i32> = VecDeque::new();
    let mut i: usize = 0;
    let mut isNum = false;
    while i < buf.len() {
        match buf[i] {
            '0'..='9' => {
                if !isNum {
                    isNum = true;
                    st.push_back((buf[i] as u8 - b'0') as i32);
                } else {
                    let mut num = st.pop_back().unwrap();
                    num *= 10;
                    num += (buf[i] as u8 - b'0') as i32;
                    st.push_back(num);
                }
            },
            '+' => {
                isNum = false;
                if st.len() >= 2 {
                    let num1 = st.pop_back().unwrap();
                    let num2 = st.pop_back().unwrap();
                    st.push_back(num1 + num2);
                } else if st.len() == 1 {
                    let num1 = st.pop_back().unwrap();
                    st.push_back(num1);
                } 
            },
            '-' => {
                isNum = false;
                if st.len() >= 2 {
                    let num1 = st.pop_back().unwrap();
                    let num2 = st.pop_back().unwrap();
                    st.push_back(num2 - num1);
                } else if st.len() == 1 {
                    let num1 = st.pop_back().unwrap();
                    st.push_back(-num1);
                } 
            },
            '*' => {
                isNum = false;
                if st.len() >= 2 {
                    let num1 = st.pop_back().unwrap();
                    let num2 = st.pop_back().unwrap();
                    st.push_back(num1 * num2);
                } else if st.len() == 1 {
                    let num1 = st.pop_back().unwrap();
                    st.push_back(num1);
                } 
            },
            '/' => {
                isNum = false;
                if st.len() >= 2 {
                    let num1 = st.pop_back().unwrap();
                    let num2 = st.pop_back().unwrap();
                    st.push_back(num2 / num1);
                } else if st.len() == 1 {
                    let num1 = st.pop_back().unwrap();
                    st.push_back(num1);
                } 
            },
            _ => {
                isNum = false;
            }
        }
        i += 1;
    }
    println!("{}", st.pop_back().unwrap());
}
