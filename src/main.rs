fn main() {
    let input = include_str!("../input");
    let data = include_str!("../data");

    let mut codes = Vec::<(Vec<u8>, String)>::new();

    let mut levels = Vec::<u8>::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let mut indent = 0;
        for c in line.chars() {
            if c != ' ' {
                break;
            }
            indent += 1;
        }
        if indent % 4 != 0 {
            panic!("indent is not a multiple of 4: {}", line);
        }
        let depth = indent / 4;
        let line = line.trim();

        if depth > levels.len() {
            panic!("depth is greater than levels: {}", line);
        }
        while depth < levels.len() {
            levels.pop();
        }

        match line.find(':') {
            Some(i) => {
                let (digits, name) = line.split_at(i);
                let mut chars = name.chars();
                chars.next();
                let name = chars.as_str().trim();

                if name.starts_with("N/A") {
                    continue;
                }
                for digit in digits.trim().split(" ") {
                    if digit.len() != 1 {
                        panic!("not a digit: {}", line);
                    }
                    let digit = digit.chars().next().unwrap();
                    let Some(digit) = digit.to_digit(10) else {
                        panic!("not a digit: {}", line);
                    };

                    let mut code = levels.clone();
                    code.push(digit as u8);
                    codes.push((code, name.to_string()));
                }
            }
            None => {
                if line.len() != 1 {
                    panic!("not a digit: {}", line);
                }
                let digit = line.chars().next().unwrap();
                let Some(digit) = digit.to_digit(10) else {
                    panic!("not a digit: {}", line);
                };
                levels.push(digit as u8);
            }
        }
    }

    codes.push((vec![], "N/A".to_string()));

    // for (code, name) in &codes {
    //     for digit in code {
    //         print!("{}", digit);
    //     }
    //     println!("{}  ->  {}", ".".repeat(5 - code.len()), name);
    // }

    for line in data.lines() {
        let position = line.find(' ').unwrap();
        let (code, name) = line.split_at(position);
        let name = name.trim();
        let code: Vec<u8> = code
            .chars()
            .take_while(|ch| ch != &'.')
            .map(|ch| {
                // println!("{ch} - {line}");
                ch.to_digit(10).expect("not a digit!!!") as u8
            })
            .collect();

        'code2: for (code2, name2) in &codes {
            for (i, digit2) in code2.iter().enumerate() {
                if code[i] != *digit2 {
                    continue 'code2;
                };
            }

            print!("{}\n{:40}", name, name2);
            print!(" (");
            for digit in &code {
                print!("{}", digit);
            }
            print!(" : ");
            for digit in code2 {
                print!("{}", digit);
            }
            println!(")");
            println!();
            break;
        }
    }
}
