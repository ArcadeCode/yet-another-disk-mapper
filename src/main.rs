use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Instant;
use std::collections::HashMap;
use rmp_serde::encode;
use walkdir::WalkDir;

fn format_file_size(size: u64, unit: &str) -> String {
    match unit {
        "o" => format!("{} octets", size),
        "Ko" => format!("{:.2} Ko", size as f64 / 1024.0),
        "Mo" => format!("{:.2} Mo", size as f64 / (1024.0 * 1024.0)),
        "Go" => format!("{:.2} Go", size as f64 / (1024.0 * 1024.0 * 1024.0)),
        _ => format!("{} octets", size),
    }
}

fn scan_dir(path: &Path) -> Vec<PathBuf> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_owned())
        .collect()
}

fn parsing_to_hashmap(entries: &[PathBuf]) -> Vec<HashMap<String, String>> {
    entries.iter().map(|entry| {
        let mut hashmap = HashMap::new();
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

    let target_dir = &args[1];
    println!("1. Scanning folders at: {}", target_dir);

    let start_time = Instant::now();
    let paths = scan_dir(Path::new(target_dir));
    let end_time = start_time.elapsed();

    println!("Time taken: {:?}", end_time);
    println!("Elements found: {:?}", paths.len());

    if !paths.is_empty() {
        let medium_time_per_element = end_time.as_nanos() / paths.len() as u128;
        println!("Medium time by element: {:?} ns", medium_time_per_element);
    } else {
        println!("No elements found, cannot calculate medium time by element.");
    }

    let hashmap = parsing_to_hashmap(&paths);
    let encoded = encode::to_vec(&hashmap).unwrap();

    let mut file = File::create("output.msgpack").expect("Impossible de créer le fichier");
    file.write_all(&encoded).expect("Impossible d'écrire dans le fichier");

    let metadata = file.metadata().expect("Impossible d'obtenir les métadonnées du fichier");
    let file_size = metadata.len();

    println!("Taille du fichier : {}", format_file_size(file_size, "o"));
    println!("Taille du fichier : {}", format_file_size(file_size, "Ko"));
    println!("Taille du fichier : {}", format_file_size(file_size, "Mo"));
    println!("Taille du fichier : {}", format_file_size(file_size, "Go"));
}
