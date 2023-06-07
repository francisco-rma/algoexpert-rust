use std::{self, fs::DirEntry};

fn main() {
    let mut path = "";
    for entry in std::fs::read_dir("../json_examples/") {
        path = match entry {
            DirEntry(value) => value,
            Err(e) => return,
        }
    }
}
