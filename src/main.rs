use memmap2::Mmap;
use std::collections::HashMap;
use std::fs::File;

fn main() {
    // (min / mean / max / total count) per station
    let mut stations: HashMap<&str, (f32, f32, f32, i32)> = HashMap::new();
    let f = File::open("./data/measurements.txt").unwrap();

    // mmap file
    let mmap = unsafe { Mmap::map(&f).unwrap() };
    mmap.advise(memmap2::Advice::Sequential).unwrap();
    let iterator = mmap.split(|sub| *sub == b'\n');
    for slice in iterator {
        let mut iter = slice.split(|w| *w == b';');
        let station_bytes = iter.next();
        let temp_bytes = iter.next();
        if let Some(station) = station_bytes
            && let Some(temp) = temp_bytes
        {
            let temp_str = unsafe { std::str::from_utf8_unchecked(temp) };
            let temp = temp_str.parse::<f32>().unwrap();

            let station = unsafe { std::str::from_utf8_unchecked(station) };

            match stations.get_mut(station) {
                Some(stats) => {
                    stats.0 = stats.0.min(temp);
                    stats.1 = stats.1 + temp / 2.0;
                    stats.2 = stats.0.max(temp);
                    stats.3 += 1;
                }
                None => {
                    stations.insert(station, (f32::MAX, temp, f32::MIN, 1));
                }
            }
        }
    }
    for (k, v) in stations {
        println!("{k}={}/{}/{},", v.0, v.1, v.2);
    }
}
