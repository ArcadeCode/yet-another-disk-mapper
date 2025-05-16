use std::fmt;
use std::time::{Duration, Instant};

pub mod report {
    use super::*;

    pub struct Report {
        pub target: String,
        pub scan_start_at: Instant,
        pub scan_duration: Duration,
        pub elements_found: u64,

        pub hashmap_parsing_start_at: Instant,
        pub hashmap_parsing_duration: Duration,

        pub msgpack_parsing_start_at: Instant,
        pub msgpack_parsing_duration: Duration,

        pub output_file_size: u64,
        pub full_duration: Duration,
        pub average_duration_by_file: u64,
        pub average_size_by_file: u64,
    }

    impl fmt::Display for Report {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Scan of {} found {} elements in {:?}",
                self.target,
                self.elements_found,
                self.full_duration
            )?;

            write!(
                f, "
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
                self.target,
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
}
