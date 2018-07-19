use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("could not read from file");

    let mut rna = String::new();
    rna.reserve(contents.len());

    for c in contents.chars() {
        match c {
            'T' => rna.push('U'),
             n  => rna.push(n),
        }
    }

    println!("{}", rna)
}
