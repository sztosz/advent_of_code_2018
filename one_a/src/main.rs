use std::io;

fn main() {
    let mut frequency = 0;
    loop {
        let mut buf = String::new();
        match io::stdin().read_line(&mut buf) {
            Ok(_) => match buf.trim_end().parse::<i32>() {
                Ok(int) => {
                    frequency += int;
                }
                Err(error) => {
                    println!("error: {}", error);
                    break;
                }
            },
            Err(error) => println!("error: {}", error),
        }
    }
    println!("frequency: {}", frequency)
}
