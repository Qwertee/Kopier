use std::fs::File;
use std::env;
use std::io::Error;
use std::io::BufWriter;
use std::io::prelude::*;

// Simple file copier
// copies file at first commandline argument to location at second commandline argument
fn main() {
    run().unwrap();
}

fn run() -> Result<(), Error> {
    // get the filenames
    let source_string = env::args().nth(1).unwrap();
    let dest_string   = env::args().nth(2).unwrap();

    println!("Source String: {0}\nDestination String: {1}", source_string, dest_string);
    
    // try to open a file at each location
    let mut source_file = try!(File::open(source_string));
    let mut dest_file = try!(File::create(dest_string));

    // data from original location is stored here
    let mut data = String::new();
    // try to read the source file into the string buffer
    try!(source_file.read_to_string(&mut data));

    println!("STRING BUFFER:\n{}", data);

    let mut f = BufWriter::new(dest_file);
    f.write_all(data.as_bytes());

    Ok(())

}
