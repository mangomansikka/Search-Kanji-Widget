use std::fs::File;
use std::io::Write;

fn main() {
    println!("cargo:rerun-if-changed=joyo.csv");

    let mut rdr = csv::Reader::from_path("joyo.csv").expect("CSV file missing");
    let mut kanji_list = Vec::new();

#[derive(serde::Serialize)]
pub struct Kanji {
    pub kanji: String,
    pub strokes: u8,
    pub on_readings: Vec<String>,
    pub kun_readings: Vec<String>,
}

    for result in rdr.records() {
        let record = result.unwrap();

        // Adjust indices depending on CSV structure
        let kanji = record[1].to_string();
        let strokes: u8 = record[4].parse().unwrap_or(0);
        let on_readings = record[8]
        .split(|c| c == ',' || c == '|')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
        let kun_readings = record[9]
        .split(|c| c == ',' || c == '|')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();


        kanji_list.push(Kanji {
            kanji,
            strokes,
            on_readings,
            kun_readings,
        });
    }

    let encoded = bincode::serialize(&kanji_list).unwrap();
    let mut out = File::create("kanji_data.bin").unwrap();
    out.write_all(&encoded).unwrap();
}
