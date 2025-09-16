//! Search for a kanji by its strokes, from the list of Joyo Kanji.
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use serde_wasm_bindgen::to_value;

// Enable wasm-bindgen test support when testing
#[cfg(test)]
use wasm_bindgen_test::wasm_bindgen_test_configure;

// Configure it to run in a browser
#[cfg(test)]
wasm_bindgen_test_configure!(run_in_browser);


#[derive(Serialize, Deserialize, Debug)]
pub struct Kanji {
    pub kanji: String,
    pub strokes: u8,
    pub on_readings: Vec<String>,
    pub kun_readings: Vec<String>,
    pub on_readings_romaji: Vec<String>,
    pub kun_readings_romaji: Vec<String>,
}

#[wasm_bindgen]
pub fn search_by_strokes(strokes: u8) -> JsValue {
    let data = include_bytes!("../kanji_data.bin");
    let kanji_list: Vec<Kanji> = bincode::deserialize(data).unwrap();

    let results: Vec<&Kanji> = kanji_list
        .iter()
        .filter(|k| k.strokes == strokes)
        .collect();

    to_value(&results).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_wasm_bindgen::from_value;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn test_search() {
        let val = search_by_strokes(1);
        let vec: Vec<Kanji> = from_value(val).unwrap();
        assert!(vec.iter().all(|k| k.strokes == 1));
    }
}

