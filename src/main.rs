use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    /*
     *
     * min / mean / max per station
     *
     */
    let mut stations: HashMap<String, Vec<f32>> = HashMap::new();
    let f = File::open("./data/measurements.txt").unwrap();
    let reader = BufReader::new(f);
    for line in reader.lines() {
        if let Some((station, temp)) = line.unwrap().split_once(";") {
            let temp = temp.parse::<f32>().unwrap();
            stations
                .entry(station.to_string())
                .or_insert(Vec::new())
                .push(temp);
        };
    }
    for (k, v) in stations {
        let min = v.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let max = v.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let mean: f32 = v.iter().sum();
        println!("{k}={min}/{mean}/{max},",);
    }
}
