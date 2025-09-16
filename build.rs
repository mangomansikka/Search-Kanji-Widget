use std::fs::File;
use std::io::Write;

fn main() {
    println!("cargo:rerun-if-changed=joyo.csv");

    let mut rdr = csv::Reader::from_path("joyo.csv").expect("CSV file missing");
    let mut kanji_list = Vec::new();

    #[derive(serde::Serialize)]
    struct Kanji {
        character: String,
        strokes: u8,
        on_readings: Vec<String>,
        kun_readings: Vec<String>,
    }

    for result in rdr.records() {
        let record = result.unwrap();

        // Adjust indices depending on CSV structure
        let character = record[0].to_string();
        let strokes: u8 = record[1].parse().unwrap_or(0);
        let on_readings = record[2].split(';').map(|s| s.trim().to_string()).collect();
        let kun_readings = record[3].split(';').map(|s| s.trim().to_string()).collect();


        kanji_list.push(Kanji {
            character,
            strokes,
            on_readings,
            kun_readings,
        });
    }

    let encoded = bincode::serialize(&kanji_list).unwrap();
    let mut out = File::create("kanji_data.bin").unwrap();
    out.write_all(&encoded).unwrap();
}
