use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    /*
     *
     * min / mean / max per station, and total count
     *
     */
    let mut stations: HashMap<String, (f32, f32, f32, i32)> = HashMap::new();
    let f = File::open("./data/measurements.txt").unwrap();
    let reader = BufReader::new(f);
    for line in reader.lines() {
        if let Some((station, temp)) = line.unwrap().split_once(";") {
            let temp = temp.parse::<f32>().unwrap();
            if let Some(stats) = stations.get_mut(&station.to_string()) {
                stats.3 += 1;
                if temp < stats.0 {
                    stats.0 = temp
                }
                stats.1 = stats.1 + temp / stats.3 as f32;
                if temp > stats.2 {
                    stats.2 = temp
                }
            } else {
                stations.insert(station.to_string(), (f32::MAX, temp, f32::MIN, 1));
            }
        };
    }
    for (k, v) in stations {
        println!("{k}={}/{}/{},", v.0, v.1, v.2);
    }
}
