extern crate histogram;
extern crate rand;

use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter};
use std::io::prelude::*;
use histogram::Histogram;
use rand::Rng;

fn get_histogram() -> (Histogram, Vec<u64>) {
    let f = File::open("in.hevc").expect("couldn't open file");
    let mut reader = BufReader::new(f);
    let mut histogram = Histogram::new();

    let mut offset = 0;
    let mut last_nal = 0;
    let mut magic = [0u8; 4];
	let mut offsets = Vec::new();

    loop {
        magic[0] = magic[1];
        magic[1] = magic[2];
        magic[2] = magic[3];
        if let Ok(_) = reader.read_exact(&mut magic[3..]) {
            offset += 1;
            if &magic[1..] == &[0, 0, 1] {
                histogram.increment(offset - last_nal);
                last_nal = offset;
				offsets.push(offset);
            }
        } else {
            break;
        }
    }
    
	println!("Percentiles: p50: {} bytes, p90: {} bytes, p99: {} bytes, p999: {} bytes",
		histogram.percentile(50.0).unwrap(),
		histogram.percentile(90.0).unwrap(),
		histogram.percentile(99.0).unwrap(),
		histogram.percentile(99.9).unwrap(),
	);

	println!("Min: {}b Avg: {}b Max: {}b StdDev: {}b",
		histogram.minimum().unwrap(),
		histogram.mean().unwrap(),
		histogram.maximum().unwrap(),
		histogram.stddev().unwrap(),
	);

	(histogram, offsets)
}

fn prune(histogram: Histogram, offsets: Vec<u64>) {
    let f = File::open("in.hevc").expect("couldn't open file");
    let mut reader = BufReader::new(f);

	let out_file = OpenOptions::new().create(true).write(true).truncate(true).open("out.hevc").expect("couldn't create output file");
	let mut writer = BufWriter::new(out_file);
	let mut rng = rand::thread_rng();

	for i in 10..offsets.len()-1 {
		let len = offsets[i+1] - offsets[i];
		if len < histogram.percentile(90.0).unwrap() {
			let mut frame_buf = vec![0u8; len as usize];
			reader.read_exact(&mut frame_buf[..]).expect("failed to read frame when copying");

			if rng.gen_weighted_bool(10) && len > 10 {
				rng.shuffle(&mut frame_buf[10..]);
			}
			
			writer.write_all(&frame_buf[..]).expect("failed to write frame to output hevc");
		}
	}
}

fn main() {
	println!("collecting histogram...");
	let (histogram, offsets) = get_histogram();

	println!("writing output");
	prune(histogram, offsets);
}
