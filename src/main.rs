use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};
type DocumentId = String;
type Term = String;
type IndexType = HashMap<Term, HashMap<DocumentId, u32>>;
type IDFType = HashMap<Term, f64>;
type DocLength = HashMap<DocumentId, u32>;
use serde::{Deserialize, Serialize};
#[derive(Debug)]
struct IndexData {
    terms_to_docs: IndexType,
    idf: IDFType,
    doc_lengths: DocLength,
    avg: f64,
}
#[derive(Debug, Serialize, Deserialize)]
struct FileData {
    name: String,
    files: Vec<String>,
}

fn load_data(data_filename: &str) -> Result<IndexData, Box<dyn std::error::Error>> {
    let file = File::open(data_filename)?;
    let reader = BufReader::new(file);
    let mut data: Vec<FileData> = Vec::new();
    for line in reader.lines().take(100) {
        let line = line?;
        let line = line.trim();
        data.push(serde_json::from_str(line)?);
    }
    let n = data.len() as f64;
    let mut indexes = IndexType::new();
    let mut doc_lengths = DocLength::new();
    for file_data in data.as_mut_slice() {
        let mut l = 0;
        for file in file_data.files.as_mut_slice() {
            let local_terms = file.split("/").map(String::from).collect::<Vec<_>>();
            l += local_terms.len();
            for term in local_terms {
                indexes
                    .entry(term.to_string())
                    .and_modify(|t| {
                        (*t).entry(file_data.name.to_string())
                            .and_modify(|e| *e += 1)
                            .or_insert(1);
                    })
                    .or_insert(HashMap::from([(file_data.name.to_string(), 1)]));
            }
        }
        doc_lengths.insert(file_data.name.to_string(), l as u32);
    }

    let mut idf = IDFType::new();
    for (term, documents) in indexes.clone().into_iter() {
        let nqi = documents.len() as f64;
        let value = ((n - nqi + 0.5) / (nqi + 0.5) + 1.).ln();
        idf.insert(term, value);
    }

    println! {"{:?}",indexes.len()}
    Ok(IndexData {
        terms_to_docs: indexes,
        idf: idf,
        doc_lengths: doc_lengths.clone(),
        avg: doc_lengths.clone().into_values().sum::<u32>() as f64 / doc_lengths.len() as f64,
    })
}
fn run_search(data: &IndexData, search: &Vec<&str>) -> Vec<(std::string::String, f64)> {
    let mut counter = HashMap::<DocumentId, u32>::new();
    let n = search.len();
    for term in search {
        let app = data.terms_to_docs.get(*term);
        for doc in app.unwrap() {
            counter
                .entry(doc.1.to_string())
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
    }
    let mut scores: Vec<(DocumentId, f64)> = Vec::new();
    for (doc, _count) in counter {
        let mut score = 0.;
        for term in search {
            let f = *data.terms_to_docs.get(*term).unwrap().get(&doc).unwrap() as f64;
            let d = *data.doc_lengths.get(&doc).unwrap() as f64;
            let avgd = data.avg;
            let k = 1.5;
            let b = 0.75;
            score += data.idf.get(*term).unwrap() * (f * (k + 1.))
                / (f + k * (1. - b + b * (d.abs() / avgd)));
        }
        scores.push((doc.to_string(), score))
    }

    scores.sort_by(|a, b| b.1.total_cmp(&a.1));
    return scores;
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    let data_filename = &args[1];
    let start = Instant::now();
    let data = load_data(&data_filename)?;
    let search = Vec::from([
        "AndroidManifest.xml",
        "DebugProbesKt.bin",
        "PullLocationStoreData.sql",
    ]);
    let matches = run_search(&data, &search);
    for file in matches {
        println!("In file :{} score: {}", file.0, file.1)
    }
    println!("Time elapsed {:?}", start.elapsed());

    Ok(())
}
