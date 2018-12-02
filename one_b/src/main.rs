use std::collections::HashSet;
use std::io;

fn main() {
    let mut changes = Vec::new();
    loop {
        let mut buf = String::new();
        match io::stdin().read_line(&mut buf) {
            Ok(_) => match buf.trim_end().parse::<i32>() {
                Ok(int) => changes.push(int),
                Err(error) => {
                    println!("error: {}", error);
                    break;
                }
            },
            Err(error) => println!("error: {}", error),
        }
    }
    let mut frequency = 0;
    let mut frequencies = HashSet::new();
    frequencies.insert(0);
    let mut changes_iter = changes.into_iter().cycle();
    loop {
        match changes_iter.next() {
            Some(change) => {
                frequency += change;
                match frequencies.insert(frequency) {
                    false => {
                        println!("answear: {}", frequency);
                        break;
                    }
                    true => (),
                }
            }
            None => println!("error: Changes itearator is empty, although it should cycle"),
        }
    }
}
