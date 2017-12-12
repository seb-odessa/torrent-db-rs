extern crate torrent_db;

use std::io::Read;
use std::io;

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buffer = Vec::new();
    match handle.read_to_end(&mut buffer) {
        Ok(bytes) => println!("Read {} bytes", &bytes),
        Err(e) => println!("ERROR: {:?}", e),
    }
}
