use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use zstd;

use super::report::Report;

/// TODO: Faire la doc
pub fn serialize(path: &Path, use_zstd: bool, verbose: bool) {
    // Step 1 : Getting all entities into the Path
    fn scan_dir(path: &Path) -> Vec<PathBuf> {
        WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .map(|e| e.path().to_owned())
            .collect()
    }

    // Step 2 : Creating a list of dictionary with all related and useful information.
    fn parsing_to_hashmap(entries: &[PathBuf]) -> Vec<HashMap<String, String>> {
        entries
            .iter()
            .map(|entry: &PathBuf| {
                let mut hashmap: HashMap<String, String> = HashMap::new();
                hashmap.insert(
                    "name".to_string(),
                    entry
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .into_owned(),
                );
                hashmap.insert(
                    "ext".to_string(),
                    entry
                        .extension()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .into_owned(),
                );
                hashmap.insert("path".to_string(), entry.to_string_lossy().into_owned());
                hashmap
            })
            .collect()
    }

    // Step 3 : We serializing the Vector to a message pack.
    fn serialize_to_msgpack(hashmap: &Vec<HashMap<String, String>>) -> File {
        let encoded: Vec<u8> = rmp_serde::encode::to_vec(&hashmap).unwrap();
        let mut file: File =
            File::create("map.msgpack.temp").expect("Creation of the file impossible");
        file.write_all(&encoded)
            .expect("Writing of the file impossible");
        // We will rewriting the file using the file content but this technique isn't good for production
        // In the futur commit, TODO: replacing this with a stream flux and output into a file AFTER.
        return file;
    }

    // Report give some information to know how good or bad this version of YADM was.
    let mut report: Report = Report::new(path.to_path_buf());

    // We check if the path go to only one File to skip the WalkDir call if not needed.
    let hashmap: Vec<HashMap<String, String>> = if path.is_file() {
        // Because we didn't scan a directory, we intialized
        report.elements_found = 1;
        report.time_hashmap_parsing();
        parsing_to_hashmap(&[path.to_path_buf()])
    } else {
        report.time_scan();
        let entries: Vec<PathBuf> = scan_dir(path);
        report.time_scan();
        report.elements_found = entries.len();

        report.time_hashmap_parsing();
        parsing_to_hashmap(&entries)
    };

    report.time_hashmap_parsing(); // Here we stop the timer

    // Serialization into msgpack format
    report.time_msgpack_parsing();
    serialize_to_msgpack(&hashmap);
    report.time_msgpack_parsing();

    if use_zstd {
        // Ouvrir le fichier sérialisé en lecture
        let mut uncompressed_file =
            File::open("map.msgpack.temp").expect("Failed to open serialized file");

        // Ouvrir un fichier de sortie pour recevoir la version compressée
        let output_file =
            File::create("map.msgpack.zst").expect("Failed to create output compressed file");

        report.time_zstd_compression();
        // Compresser du fichier sérialisé vers le fichier compressé avec un niveau de compression à 22 (max)
        zstd::stream::copy_encode(&mut uncompressed_file, output_file, 22).expect("Compression failed");
        report.time_zstd_compression();

        match std::fs::remove_file(Path::new("map.msgpack.temp")) {
            Ok(_) => println!("Fichier supprimé avec succès !"),
            Err(e) => eprintln!("Erreur lors de la suppression du fichier : {}", e),
        }
    } else {
        match std::fs::rename(Path::new("map.msgpack.temp"), Path::new("map.msgpack")) {
            Ok(_) => println!("Fichier supprimé avec succès !"),
            Err(e) => eprintln!("Erreur lors de la suppression du fichier : {}", e),
        }
    }
    

    report.close_report();
    if verbose {
        // FIXME: In the futur, we need to implement a dummy report struct to skip all timers when "verbose" is set to false
        println!("{}", report);
    }
}
