use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};
type DocumentId = String;
type Term = String;
type IndexType = HashMap<Term, HashSet<DocumentId>>;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct file_data {
    name: String,
    files: Vec<String>,
}

fn load_data(data_filename: &str) -> Result<IndexType, Box<dyn std::error::Error>> {
    let file = File::open(data_filename)?;
    let reader = BufReader::new(file);

    let mut data: Vec<file_data> = Vec::new();
    for line in reader.lines().take(2) {
        let line = line?;
        let line = line.trim();
        data.push(serde_json::from_str(line)?);
    }

    let mut indexes = IndexType::new();
    for file_data in data.as_mut_slice() {
        for file in file_data.files.as_mut_slice() {
            let local_terms = file.split("/").map(String::from).collect::<Vec<_>>();
            for term in local_terms {
                indexes
                    .entry(term.to_string())
                    .and_modify(|t| {
                        (*t).insert(file_data.name.to_string());
                    })
                    .or_insert(HashSet::from([file_data.name.to_string()]));
            }
        }
    }
    println! {"{:?}",indexes.len()}
    Ok(indexes)
}
fn run_search(data: &IndexType, search: &Vec<&str>) {
    let mut counter = HashMap::<DocumentId, u32>::new();
    for term in search {
        let app = data.get(*term);
        for name in app.unwrap() {
            counter
                .entry(name.to_string())
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
    }
    let maximum = search.len();
    for file in counter {
        println!("In file :{} found {}/{}", file.0, file.1, maximum)
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    let data_filename = &args[1];
    let start = Instant::now();
    let data = load_data(&data_filename)?;
    let search = Vec::from(["AndroidManifest.xml", "DebugProbesKt.bin"]);
    run_search(&data, &search);
    println!("Time elapsed {:?}", start.elapsed());

    Ok(())
}
