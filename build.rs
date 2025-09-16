use std::fs::File;
use std::io::Write;

fn kana_to_romaji(kana: &str) -> String {
    // Minimal example mapping; for real use you might want a library
    match kana {
        "ジン" => "jin",
        "ニン" => "nin",
        "ひと" => "hito",
        _ => kana, // fallback: leave as-is
    }.to_string()
}


fn main() {
    println!("cargo:rerun-if-changed=joyo.csv");

    let mut rdr = csv::Reader::from_path("joyo.csv").expect("CSV file missing");
    let mut kanji_list = Vec::new();

#[derive(serde::Serialize)]
pub struct Kanji {
    pub kanji: String,
    pub strokes: u8,
    pub on_readings: Vec<String>,   // kana
    pub kun_readings: Vec<String>,  // kana
    pub on_readings_romaji: Vec<String>,
    pub kun_readings_romaji: Vec<String>,
}

    for result in rdr.records() {
        let record = result.unwrap();

        // Adjust indices depending on CSV structure
        let kanji = record[1].to_string();
        let strokes: u8 = record[4].parse().unwrap_or(0);
        let on_readings: Vec<String> = record[8].split('|').map(|s| s.trim().to_string()).collect();
        let kun_readings: Vec<String> = record[9].split('|').map(|s| s.trim().to_string()).collect();
        let on_readings_romaji: Vec<String> = on_readings.iter().map(|r| kana_to_romaji(r)).collect();
        let kun_readings_romaji: Vec<String> = kun_readings.iter().map(|r| kana_to_romaji(r)).collect();

        kanji_list.push(Kanji {
            kanji,
            strokes,
            on_readings,
            kun_readings,
            on_readings_romaji,
            kun_readings_romaji,
        });
    }

    let encoded = bincode::serialize(&kanji_list).unwrap();
    let mut out = File::create("kanji_data.bin").unwrap();
    out.write_all(&encoded).unwrap();
}
