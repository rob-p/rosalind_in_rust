use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("could not read from file");

    let mut cnt_a = 0;
    let mut cnt_c = 0;
    let mut cnt_g = 0;
    let mut cnt_t = 0;
    let mut cnt_other = 0;

    for c in contents.chars() {
        match c {
            'A' => cnt_a += 1,
            'C' => cnt_c += 1,
            'G' => cnt_g += 1,
            'T' => cnt_t += 1,
            _   => cnt_other += 1,
        }
    }

    println!("{} {} {} {}", cnt_a, cnt_c, cnt_g, cnt_t)
}
