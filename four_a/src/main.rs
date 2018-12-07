use std::collections::HashMap;
use std::io;

struct Entry {
    start: i32,
    end: i32,
    guard_id: i32
}

fn main() {
    let mut entries = Vec::new();
    loop {
        let mut buf = String::new();
        match io::stdin().read_line(&mut buf) {
            Ok(_) => {
                if buf == "\n" {
                    break;
                }
                entries.push(buf)
            }
            Err(error) => println!("error: {}", error),
        }
    }

    let mut dates = HashMap::new();
    for entry in &entries {
        let parts: Vec<&str> = entry.split("] ").collect();
        let timedate: Vec<&str> = parts[0].split(" ").collect();
        let date = timedate[0].trim_matches('[');
        let minutes = timedate[1].split(":").nth(1).unwrap().parse::<i32>().unwrap();
        match parts[2] {
            "wakes up" => {
                let entry = dates.entry(date).or_insert(Entry {start: 0, end: 0, guard_id: 0});
                entry.end = minutes - 1;
            }
            "falls asleep" => {
                let entry = dates.entry(date).or_insert(Entry {start: 0, end: 0, guard_id: 0});
                entry.start = minutes;
            }
            _ => {
                let entry = dates.entry(date).or_insert(Entry {start: 0, end: 0, guard_id: 0});
                entry.guard_id = parts[2][7..10].parse::<i32>().unwrap();
            }
        }
        
    //     let mut offsets: Vec<&str> = parts[0].split(',').collect();
    //     let offset_x: i32 = offsets[0].parse().unwrap();
    //     let offset_y: i32 = offsets[1].parse().unwrap();
    //     let mut dimensions: Vec<&str> = parts[1].split('x').collect();
    //     let range_x_end: i32 = dimensions[0].parse().unwrap();
    //     for dimension_x in 1..(range_x_end + 1) {
    //         let range_y_end: i32 = dimensions[1].trim_end().parse().unwrap();
    //         for dimension_y in 1..(range_y_end + 1) {
    //             let piece = fabric
    //                 .entry((dimension_x + offset_x, dimension_y + offset_y))
    //                 .or_insert(0);
    //             *piece += 1;
    //         }
    //     }
    // }
    // let mut pieces_overlapping = 0;
    // for (_, v) in &fabric {
    //     if v >= &2 {
    //         pieces_overlapping += 1;
    //     }
    // }

    // println!("fabric: {:?}", fabric.keys().count());
    // println!("pieces_overlapping: {:?}", pieces_overlapping);
}
