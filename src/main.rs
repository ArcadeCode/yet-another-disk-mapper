use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use std::collections::HashMap;
use rmp_serde::encode;
use walkdir::WalkDir;

mod yadm;
use yadm::report::report::Report;

fn scan_dir(path: &Path) -> Vec<PathBuf> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_owned())
        .collect()
}

fn parsing_to_hashmap(entries: &[PathBuf]) -> Vec<HashMap<String, String>> {
    entries.iter().map(|entry: &PathBuf| {
        let mut hashmap: HashMap<String, String> = HashMap::new();
        hashmap.insert("name".to_string(), entry.file_name().unwrap_or_default().to_string_lossy().into_owned());
        hashmap.insert("ext".to_string(), entry.extension().unwrap_or_default().to_string_lossy().into_owned());
        hashmap.insert("path".to_string(), entry.to_string_lossy().into_owned());
        hashmap
    }).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <directory_path>", args[0]);
        std::process::exit(1);
    }

    let target_dir: &String = &args[1];

    println!("1. Scanning folders at: {}", target_dir);
    let scan_start: Instant = Instant::now();
    let paths: Vec<PathBuf> = scan_dir(Path::new(target_dir));
    let scan_duration: Duration = scan_start.elapsed();

    let total_files: u64 =
    if paths.is_empty() {
        eprintln!("No files found. Exiting.");
        return;
    } else {
        let count = paths.len() as u64;
        println!("Found {} files", count);
        count
    };


    println!("2. Parsing to hashmap...");
    let hashmap_parsing_start: Instant = Instant::now();
    let hashmap: Vec<HashMap<String, String>> = parsing_to_hashmap(&paths);
    let hashmap_parsing_duration: Duration = hashmap_parsing_start.elapsed();


    println!("3. Encoding + writing MessagePack...");
    let msgpack_parsing_start = Instant::now();

    let encoded: Vec<u8> = encode::to_vec(&hashmap).unwrap();
    let mut file: File = File::create("output.msgpack").expect("Impossible de créer le fichier");
    file.write_all(&encoded).expect("Impossible d'écrire dans le fichier");

    let msgpack_parsing_duration: Duration = msgpack_parsing_start.elapsed();

    let metadata: std::fs::Metadata = file.metadata().expect("Impossible d'obtenir les métadonnées du fichier");
    let file_size: u64 = metadata.len();

    let report: Report = Report {
        target: args[0].clone(),
        scan_start_at: scan_start,
        scan_duration: scan_duration,
        elements_found: file_size,
        hashmap_parsing_start_at: hashmap_parsing_start,
        hashmap_parsing_duration: hashmap_parsing_duration,
        msgpack_parsing_start_at: msgpack_parsing_start,
        msgpack_parsing_duration: msgpack_parsing_duration,
        output_file_size: file_size,
        full_duration: scan_duration+hashmap_parsing_duration+msgpack_parsing_duration,
        average_duration_by_file: file_size.checked_div(scan_duration.as_secs()+hashmap_parsing_duration.as_secs()+msgpack_parsing_duration.as_secs()).unwrap_or(0),
        average_size_by_file: file_size.checked_div(total_files).unwrap_or(0),
    };

    println!("{}", report)
}