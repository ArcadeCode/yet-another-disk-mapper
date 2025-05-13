use std::fs;
use std::time::{Duration, Instant};
use std::path::{PathBuf};
use std::ffi::OsStr;

use rmp_serde::{encode};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

fn format_file_size(size: u64, unit: &str) -> String {
    match unit {
        "o" => format!("{} octets", size),
        "Ko" => format!("{:.2} Ko", size as f64 / 1024.0),
        "Mo" => format!("{:.2} Mo", size as f64 / (1024.0 * 1024.0)),
        "Go" => format!("{:.2} Go", size as f64 / (1024.0 * 1024.0 * 1024.0)),
        _ => format!("{} octets", size),
    }
}

fn scan_dir(path: &str) -> Vec<PathBuf> {
    let mut result = Vec::new();
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    // Récursivement scanner les sous-répertoires
                    let mut sub_dir_results = scan_dir(path.to_str().unwrap());
                    result.append(&mut sub_dir_results);
                } else {
                    // Ajouter le fichier à la liste des résultats
                    result.push(path);
                }
            }
        }
    }
    result
}

fn scan_dirs(path: &str) -> Vec<PathBuf> {
    let result = scan_dir(path);
    result
}

fn parsing_to_hashmap(entries: &Vec<PathBuf>) -> Vec<HashMap<String, String>> {
    let mut dictionaries: Vec<HashMap<String, String>> = Vec::new();

    for entry in entries {
        let mut hashmap = HashMap::new();

        // Convertir Option<&OsStr> en String pour "name"
        if let Some(name) = entry.file_name().and_then(OsStr::to_str) {
            hashmap.insert(String::from("name"), name.to_string());
        } else {
            hashmap.insert(String::from("name"), String::from(""));
        }

        // Convertir Option<&OsStr> en String pour "ext"
        if let Some(ext) = entry.extension().and_then(OsStr::to_str) {
            hashmap.insert(String::from("ext"), ext.to_string());
        } else {
            hashmap.insert(String::from("ext"), String::from(""));
        }

        // Convertir Option<&str> en String pour "path"
        if let Some(path) = entry.to_str() {
            hashmap.insert(String::from("path"), path.to_string());
        } else {
            hashmap.insert(String::from("path"), String::from(""));
        }

        dictionaries.push(hashmap);
    }

    dictionaries
}


fn main() {
    println!("1. Scanning folders...");
    let start_time: Instant = Instant::now();

    let paths: Vec<PathBuf> = scan_dirs("C:/Users/ederv/");

    let end_time: Duration = start_time.elapsed();

    println!("Time taken: {:?}", end_time);
    println!("Elements found: {:?}", paths.len());

    if paths.len() > 0 {
        let medium_time_per_element = end_time.as_nanos() / paths.len() as u128;
        println!("Medium time by element: {:?} ns", medium_time_per_element);
    } else {
        println!("No elements found, cannot calculate medium time by element.");
    }

    //println!("2. Parsing to a list of dicts...");
    let hashmap = parsing_to_hashmap(&paths);

    // Sérialiser le HashMap en MessagePack
    let encoded = encode::to_vec(&hashmap).unwrap();

    // Écrire le MessagePack dans un fichier
    let mut file = File::create("output.msgpack").expect("Impossible de créer le fichier");
    file.write_all(&encoded).expect("Impossible d'écrire dans le fichier");

    // Obtenir la taille du fichier
    let metadata = file.metadata().expect("Impossible d'obtenir les métadonnées du fichier");
    let file_size = metadata.len();

    //println!("MessagePack sauvegardé dans output.msgpack");
    
    // Afficher la taille du fichier dans différentes unités
    println!("Taille du fichier : {}", format_file_size(file_size, "o"));
    println!("Taille du fichier : {}", format_file_size(file_size, "Ko"));
    println!("Taille du fichier : {}", format_file_size(file_size, "Mo"));
    println!("Taille du fichier : {}", format_file_size(file_size, "Go"));

}