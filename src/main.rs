use std::{
    collections::HashMap,
    io::{self, BufRead},
};

mod model;

struct Measurement(String, u64, u64);

struct Table {
    prefix: String,
    measurements: Vec<Measurement>,
}

impl Table {
    fn new(prefix: String) -> Self {
        Self {
            prefix,
            measurements: Vec::new(),
        }
    }

    fn add(&mut self, name: String, median: u64, deviation: u64) {
        self.measurements.push(Measurement(name, median, deviation));
    }

    fn print(&self) {
        println!("## {}", self.prefix);
        println!("**Note:** All measurements are in ns/iter.");
        println!("| Name | Median | Deviation |");
        println!("|------|--------|-----------|");
        for Measurement(name, median, deviation) in &self.measurements {
            println!("| {} | {} | {} |", name, median, deviation);
        }
        println!();
    }
}

fn split_bench_name(name: String) -> (String, String) {
    let mut parts = name.rsplitn(2, "::");
    let name = parts.next().unwrap();
    let prefix = parts.next().unwrap();
    (prefix.to_string(), name.to_string())
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut tables = HashMap::new();
    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let message = serde_json::from_str::<model::Message>(&line).unwrap();
        if let model::Message::Bench(outcome) = message {
            let (prefix, name) = split_bench_name(outcome.name);
            let table = tables
                .entry(prefix.clone())
                .or_insert_with(|| Table::new(prefix));
            table.add(name, outcome.median, outcome.deviation);
        }
    }
    for table in tables.values() {
        table.print();
    }
}
