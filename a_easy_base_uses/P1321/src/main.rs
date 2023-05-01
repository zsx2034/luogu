fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut chars = buf.chars().collect::<Vec<char>>();

    let template_boy = vec![
        Vec::new(), 
        vec![
            vec!['b'], 
            vec!['o'], 
            vec!['y'],
        ],
        vec![
            vec!['b', 'o'],
            vec!['o', 'y'],
        ],
        vec![
            vec!['b', 'o', 'y'],
        ],
    ];

    let template_girl = vec![
        Vec::new(), 
        vec![
            vec!['g'],
            vec!['i'],
            vec!['r'],
            vec!['l'],
        ],
        vec![
            vec!['g', 'i'],
            vec!['i', 'r'],
            vec!['r', 'l'],
        ],
        vec![
            vec!['g', 'i', 'r'],
            vec!['i', 'r', 'l'],
        ],
        vec![
            vec!['g', 'i', 'r', 'l'],
        ],
    ];

    let mut count_boy = 0;
    let mut count_girl = 0;
    let len_chars = chars.len();

    // compare with template_boy and count it if match
    for step in (1..4).rev() {
        for i in 0..len_chars - step {
            let tmp = &mut chars[i..i + step];
            for template in template_boy[step].iter() {
                if template == tmp {
                    count_boy += 1;
                    for ele in &mut *tmp {
                        *ele = '.';
                    }
                }
            }
        }
    }
    // compare with template_boy and count it if match
    for step in (1..5).rev() {
        for i in 0..len_chars - step {
            let tmp = &mut chars[i..i + step];
            for template in template_girl[step].iter() {
                if template == tmp {
                    count_girl += 1;
                    for ele in &mut *tmp {
                        *ele = '.';
                    }
                }
            }
        }
    }
    println!("{}", count_boy);
    println!("{}", count_girl);
}
