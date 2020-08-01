use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::convert::TryInto;
use std::time::Instant;

//use std::thread;
//use std::sync::{Arc, Mutex};

extern crate crossbeam;
use crossbeam::atomic::AtomicCell;

fn main() -> std::io::Result<()> {

	// for atomic cell make vector of atomics
	let mut obj_creation = 0.0;
	let mut avg_line = 0.0;
	let mut total_line = 0.0;

	let time0 = Instant::now();
	let args: Vec<String> = env::args().collect();
	
	// parse args
	let _lines = &args[1].parse::<i32>().unwrap();
	let dimension = &args[2].parse::<i32>().unwrap();
	let input_file = &args[3];
	let output_file = &args[4];
    let	parseargs = time0.elapsed().as_nanos() as f64;

	let time1 = Instant::now();
	
	let mut sums = vec![0.0; (*dimension).try_into().unwrap()];

	let mut outfile = File::create(output_file)?;
	
	// read input file line by line
	if let Ok(lines) = read_lines(input_file) {
		obj_creation = time1.elapsed().as_nanos() as f64;
		let time2 = Instant::now();
		let mut timesum = 0.0;
		
		for line in lines {
			let time3 = Instant::now();
			if let Ok(ip) = line {			
				
				let split: Vec<&str> = ip.split(",").collect();
				
				let _ = crossbeam::scope(|s| {
					let mut index: usize = 0;
					for slice in sums.chunks_mut(1) {
						let element = split[index]; 
						s.spawn(move |_| write_slice(slice, element));
						index += 1;
					}
				}).unwrap();

//				println!("{:?}", &sums[..]);
			}
			timesum = timesum + time3.elapsed().as_nanos() as f64;		
		}
		println!("{}", *_lines);
		avg_line = timesum as f64 / *_lines as f64;	
		total_line = time2.elapsed().as_nanos() as f64;
	
	}

	// write sum to output file
	let time4 = Instant::now();
	for n in &sums {
		let str = n.to_string() + " ";
		outfile.write_all(str.as_bytes()).expect("can't write to output file");
	}
	let write_out = time4.elapsed().as_nanos() as f64;
	let total = time0.elapsed().as_nanos() as f64;

	println!("time to parse args: {} ({:.2}%)", parseargs, parseargs / total * 100.0);
	println!("time to create vector / files: {} ({:.2}%)", obj_creation, obj_creation / total * 100.0);
	println!("average time per line: {} ({:.2}%)", avg_line, avg_line / total * 100.0);	
	println!("total time for summing: {} ({:.2}%)", total_line, total_line / total * 100.0);	
	println!("time to write to output file: {} ({:.2}%)", write_out, write_out / total * 100.0);
	println!("total time: {}", total * 100.0);
	
	Ok(())
}

// returns an Iterator to the Reader of the lines of the file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> 
where P: AsRef<Path>, {
	let file = File::open(filename)?;
	Ok(io::BufReader::new(file).lines())
}

fn write_slice(slice: &mut [f64], element: &str) {
	for e in slice.iter_mut() {
		*e += element.parse::<f64>().unwrap() as f64;
	}
}
