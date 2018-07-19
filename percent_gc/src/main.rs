extern crate seq_io;
use seq_io::fasta::{Reader,Record};
use std::io;
use std::env;
use std::str;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut reader = Reader::from_path(&filename).unwrap();
    let mut highest_gc = 0.0;
    let mut highest_name = String::new();

    for bio_rec in reader.records() {
        let rec = bio_rec.unwrap();

        let mut num_gc = 0;
        let mut tot = 0;

        for c in str::from_utf8(rec.seq()).unwrap().chars() {
            match c {
                'C' => num_gc += 1,
                'G' => num_gc += 1,
                 _  => (),
            }
        }
        tot += rec.seq().len();
        let frac_gc = (num_gc as f64) / (tot as f64);
        if frac_gc > highest_gc {
            highest_gc = frac_gc;
            highest_name = rec.id().unwrap().to_string();
        }
    }

    println!("{} {:.6}", highest_name, 100.0 * highest_gc);
}
