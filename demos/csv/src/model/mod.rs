use {
    csv::Reader,
    serde::Deserialize,
    std::{collections::HashMap, fs},
};

#[derive(Debug, Deserialize, Clone)]
pub struct Price {
    #[serde(rename = "Date")]
    _date: String,
    #[serde(rename = "Open")]
    pub open: f64,
    #[serde(rename = "High")]
    pub high: f64,
    #[serde(rename = "Low")]
    pub low: f64,
    #[serde(rename = "Close")]
    pub close: f64,
    #[serde(rename = "Volume")]
    _volume: usize,
}

const DIR: &str = "../assets/historical_data";

#[derive(Debug, Default, Clone)]
pub struct Model {
    pub cash: HashMap<String, Vec<Price>>,
    pub list: Vec<String>,
    pub curr: usize,
}

impl Model {
    pub fn init(&mut self) {
        for file in std::fs::read_dir(DIR).unwrap() {
            let entry = file.unwrap().file_name().into_string().unwrap();
            if entry.ends_with(".csv") {
                self.list
                    .push(entry.strip_suffix(".csv").unwrap().to_string());
            }
            self.choice(self.curr);
        }
    }
    pub fn choice(&mut self, curr: usize) -> bool {
        if self.cash.contains_key(&self.list[curr]) {
            self.curr = curr;
        } else if let Ok(data) = fs::read(format!("{DIR}/{}.csv", self.list[curr])) {
            let mut prices: Vec<Price> = Vec::new();
            for result in Reader::from_reader(data.as_slice()).deserialize() {
                prices.push(result.unwrap());
            }
            self.cash.insert(self.list[curr].clone(), prices);
            self.curr = curr;
        }
        true
    }
}
