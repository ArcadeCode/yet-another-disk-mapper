use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

/// TODO: Faire la doc
pub fn serialize(path: &Path) {

    // Step 1 : Getting all entities into the Path
    fn scan_dir(path: &Path) -> Vec<PathBuf> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_owned())
        .collect()
    }

    // Step 2 : Creating a list of dictionary with all related and useful informations.
    fn parsing_to_hashmap(entries: &[PathBuf]) -> Vec<HashMap<String, String>> {
        entries.iter().map(|entry: &PathBuf| {
            let mut hashmap: HashMap<String, String> = HashMap::new();
            hashmap.insert("name".to_string(), entry.file_name().unwrap_or_default().to_string_lossy().into_owned());
            hashmap.insert("ext".to_string(), entry.extension().unwrap_or_default().to_string_lossy().into_owned());
            hashmap.insert("path".to_string(), entry.to_string_lossy().into_owned());
            hashmap
        }).collect()
    }

    // We check if the path go to only one File to skip the WalkDir call if not needed.
    let hashmap: Vec<HashMap<String, String>> = if path.is_file() {
        parsing_to_hashmap(&[path.to_path_buf()])
    } else {
        parsing_to_hashmap(&scan_dir(path))
    };

    let encoded: Vec<u8> = rmp_serde::encode::to_vec(&hashmap).unwrap();
    let mut file: File = File::create("output.msgpack").expect("Creation of the file impossible");
    file.write_all(&encoded).expect("Writing of the file impossible");
}