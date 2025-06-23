use std::path::Path;
mod yadm;

fn main() {
    yadm::serialize(Path::new("."))
}
