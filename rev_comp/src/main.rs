use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("could not read from file");

    let mut rc = String::new();
    rc.reserve(contents.len());

    for c in contents.trim().chars().rev() {
        match c {
            'T' => rc.push('A'),
            'C' => rc.push('G'),
            'G' => rc.push('C'),
            'A' => rc.push('T'),
             _  => rc.push('N'),
        }
    }

    println!("{}", rc)
}
