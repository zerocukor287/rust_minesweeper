use std::fs::{self, create_dir_all, File};
use std::io::Write;

use directories::ProjectDirs;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Stats {
    version: u32,
    defused: usize,
    revealed: usize,
    exploded: usize,
}

const STATS_VERSION: u32 = 1;

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
    if let Some(proj_dirs) = ProjectDirs::from("com", "ChromaticCarrot",  "Minesweeper") {
        let mut file = File::create(proj_dirs.data_local_dir()
                .join("stats.json"))
                .expect("I just created this file.. How can it be not accessible?");
        let _ = file.write_all(serde_json::to_string(&data).unwrap().as_bytes());
    }

}

pub fn get_stats() -> Stats {
    if let Some(proj_dirs) = ProjectDirs::from("com", "ChromaticCarrot",  "Minesweeper") {
        if !proj_dirs.data_local_dir().exists() {
            let result = create_dir_all(proj_dirs.data_local_dir());
            match result {
                Ok(_) => (),
                Err(err) => println!("Error on creating the folder. {}", err),
            }
        }

        if proj_dirs.data_local_dir()
                .join("stats.json").exists() {
            let data = fs::read_to_string(proj_dirs.data_local_dir()
                                    .join("stats.json"))
                                    .expect("Failed to read an existing file");
            
            if let Ok(stats) = serde_json::from_str(&data) {
                stats
            } else {
                // remove the corrupted file, and create a new one
                Stats {
                    version: 0,
                    defused: 0,
                    revealed: 0,
                    exploded: 0,
                }
            }
        } else {
            Stats {
                version: 0,
                defused: 0,
                revealed: 0,
                exploded: 0,
            }
        }
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