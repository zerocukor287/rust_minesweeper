use std::fs::{self, File};
use std::path::Path;
use std::io::Write;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Stats {
    version: u32,
    defused: usize,
    revealed: usize,
    exploded: usize,
}

const STATS_VERSION: u32 = 1;
const STATS_PATH: &str = "data/stats.json";

pub fn save_stats(defused: usize, revealed: usize, exploded: bool) {
    // get previous content, or empty if not existing
    let mut data = get_stats();

    // convert from old
    if data.version != STATS_VERSION {
        data.version = 1;
        data.defused = 0;
        data.revealed = 0;
        data.exploded = 0;
    }
    // increment the numbers
    data.defused += defused;
    data.revealed += revealed;
    if exploded {
        data.exploded += 1;
    }

    // write file
    let mut file = File::create(STATS_PATH).expect("I just created this file.. How can it be not accessible?");
    let _ = file.write_all(serde_json::to_string(&data).unwrap().as_bytes());
}

pub fn get_stats() -> Stats {
    if Path::new(STATS_PATH).exists() {
        let data = fs::read_to_string(STATS_PATH).expect("Failed to read an existing file");
        
        let stats: Stats = serde_json::from_str(&data).expect("Bad formatted json");
        stats
    } else {
        Stats {
            version: 0,
            defused: 0,
            revealed: 0,
            exploded: 0,
        }
    }
}

pub fn print_stats(stat: &Stats) {
    print!(
"Stats:
    You have defused {} mines
    You have revealed {} safe tiles
    You have exploded {} times
Congrats!\n\n", stat.defused, stat.revealed, stat.exploded);
}