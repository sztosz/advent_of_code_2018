use std::collections::HashMap;
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
    let mut twices = 0;
    let mut thrices = 0;

    for id in ids {
        let mut counts: HashMap<char, i32> = HashMap::new();
        for x in id.chars() {
            let val = counts.entry(x).or_insert(0);
            *val += 1;
        }
        for (_, v) in &counts {
            match v {
                2 => {
                    twices += 1;
                    break;
                }
                _ => (),
            }
        }
        for (_, v) in &counts {
            match v {
                3 => {
                    thrices += 1;
                    break;
                }
                _ => (),
            }
        }
    }

    println!("Result: {}", twices * thrices)
}
