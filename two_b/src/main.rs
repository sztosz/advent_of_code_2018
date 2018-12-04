use std::io;

fn main() {
    let mut ids = Vec::new();
    loop {
        let mut buf = String::new();
        match io::stdin().read_line(&mut buf) {
            Ok(_) => {
                if buf == "\n" {
                    break;
                }
                ids.push(buf)
            }
            Err(error) => println!("error: {}", error),
        }
    }

    let mut matches = Vec::new();

    for id in &ids {
        for comparable in &ids {
            let mut number_of_differences = 0;
            for (index, char_) in id.chars().enumerate() {
                match comparable.chars().nth(index) {
                    Some(c) => {
                        if char_ != c {
                            number_of_differences += 1;
                        }
                    }
                    None => (),
                }
            }
            if number_of_differences == 1 {
                matches.push(comparable)
            }
        }
    }

    let first = &matches[0];
    let second = &matches[1];
    let mut result = String::new();

    for (index, char_) in first.chars().enumerate() {
        match second.chars().nth(index) {
            Some(c) => {
                if char_ == c {
                    result.push(c)
                }
            }
            None => (),
        }
    }
    println!("result: {}", result)
}
