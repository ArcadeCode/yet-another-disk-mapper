use std::fmt;
use std::path::PathBuf;
use std::time::{Duration, Instant};

pub struct Report {
    pub target_path: PathBuf,
    pub scan_start_at: Instant,
    pub scan_duration: Duration,
    pub scan_started: bool,
    pub elements_found: usize,

    pub hashmap_parsing_start_at: Instant,
    pub hashmap_parsing_duration: Duration,
    pub hashmap_parsing_started: bool,

    pub msgpack_parsing_start_at: Instant,
    pub msgpack_parsing_duration: Duration,
    pub msgpack_parsing: bool,

    pub zstd_compression_start_at: Instant,
    pub zstd_compression_duration: Duration,
    pub zstd_compression: bool,

    pub output_file_size: usize,
    pub full_duration: Duration,
    pub average_duration_by_file: usize,
    pub average_size_by_file: usize,
}

impl Report {
    pub fn time_scan(&mut self) {
        if self.scan_started {
            self.scan_duration = Instant::now() - self.scan_start_at;
        } else {
            self.scan_started = true;
        }
    }
    pub fn time_hashmap_parsing(&mut self) {
        if self.hashmap_parsing_started {
            self.hashmap_parsing_duration = Instant::now() - self.hashmap_parsing_start_at;
        } else {
            self.hashmap_parsing_started = true;
        }
    }
    pub fn time_msgpack_parsing(&mut self) {
        if self.msgpack_parsing {
            self.msgpack_parsing_duration = Instant::now() - self.msgpack_parsing_start_at;
        } else {
            self.msgpack_parsing = true;
        }
    }
    pub fn time_zstd_compression(&mut self) {
        if self.zstd_compression {
            self.zstd_compression_duration = Instant::now() - self.zstd_compression_start_at;
        } else {
            self.zstd_compression = true;
        }
    }

    pub fn new(target_path: PathBuf) -> Self {
        // We use `Instant::now()` as a neutral initialization value for all start timestamps.
        // Although these fields will be overwritten later when their respective steps actually begin,
        // `Instant` does not implement `Default`, and there is no way to create an "empty" Instant.
        // Using a common `now` reference ensures the struct is fully initialized with valid values
        // and avoids the need for Option<Instant>, which would add complexity and require unwrapping.
        let now = Instant::now();
        Report {
            target_path: target_path.canonicalize().unwrap_or(target_path),
            scan_start_at: now,
            scan_duration: Duration::ZERO,
            scan_started: false,
            elements_found: usize::default(),

            hashmap_parsing_start_at: now,
            hashmap_parsing_duration: Duration::ZERO,
            hashmap_parsing_started: false,

            msgpack_parsing_start_at: now,
            msgpack_parsing_duration: Duration::ZERO,
            msgpack_parsing: false,

            zstd_compression_start_at: now,
            zstd_compression_duration: Duration::ZERO,
            zstd_compression: false,

            output_file_size: usize::default(),
            full_duration: Duration::ZERO,
            average_duration_by_file: usize::default(),
            average_size_by_file: usize::default(),
        }
    }

    pub fn close_report(&mut self) {
        self.full_duration = self.scan_duration
            + self.hashmap_parsing_duration
            + self.msgpack_parsing_duration
            + self.zstd_compression_duration;

        if self.elements_found > 0 {
            let nanos = self.full_duration.as_nanos() / self.elements_found as u128;
            self.average_duration_by_file = (nanos / 1_000_000) as usize; // en millisecondes
            self.average_size_by_file = self.output_file_size / self.elements_found;
        } else {
            self.average_duration_by_file = 0;
            self.average_size_by_file = 0;
        }
    }
}

impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Scan of {} found {} elements in {:?}",
            self.target_path.to_str().unwrap(),
            self.elements_found,
            self.full_duration
        )?;

        write!(
            f,
            "
=== YADM report ===
Target : {}
Scan duration : {:.2}s
Elements found : {}
Hashmap parsing duration : {:.2}s
MsgPack parsing duration : {:.2}s
-----
Full duration : {:.2}s
Average duration by file : {:.2}s
Average size by file : {} o
Output file size : {} o
Output file size : {:.2} Ko
Output file size : {:.2} Mo
Output file size : {:.2} Go
===================
",
            self.target_path.to_str().unwrap(),
            self.scan_duration.as_secs_f64(),
            self.elements_found,
            self.hashmap_parsing_duration.as_secs_f64(),
            self.msgpack_parsing_duration.as_secs_f64(),
            self.full_duration.as_secs_f64(),
            self.average_duration_by_file,
            self.average_size_by_file,
            self.output_file_size,
            self.output_file_size as f64 / 1024.0,
            self.output_file_size as f64 / (1024.0 * 1024.0),
            self.output_file_size as f64 / (1024.0 * 1024.0 * 1024.0),
        )
    }
}
