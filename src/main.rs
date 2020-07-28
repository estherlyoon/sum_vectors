use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::convert::TryInto;
use std::time::Instant;
use crossbeam::atomic::AtomicCell;


fn main() -> std::io::Result<()> {

	let mut a = AtomicCell::new(7);
	println!("{}", a.into_inner());

	// times
	let mut parseargs = 0.0;
	let mut obj_creation = 0.0;
	let mut avg_line = 0.0;
	let mut total_line = 0.0;
	let mut write_out = 0.0;
	let mut total = 0.0;

	let time0 = Instant::now();
	let args: Vec<String> = env::args().collect();
	// parse command line args
	let _lines = &args[1].parse::<i32>().unwrap();
	let dimension = &args[2].parse::<i32>().unwrap();
	let input_file = &args[3];
	let output_file = &args[4];
	parseargs = time0.elapsed().as_nanos() as f64;

	let time1 = Instant::now();
	// init sum vector and file vars
	let mut sums = vec![0.0; (*dimension).try_into().unwrap()];

	let mut outfile = File::create(output_file)?;
	
	// read input file line by line
	if let Ok(lines) = read_lines(input_file) {
		obj_creation = time1.elapsed().as_nanos() as f64;
		let time2 = Instant::now();
		let mut timesum = 0.0;
		for line in lines {
			let mut time3 = Instant::now();
			if let Ok(ip) = line {
				let split = ip.split(",");
				let mut index = 0;
				for s in split {
					sums[index] += s.parse::<f32>().unwrap();
					index = index + 1;
				}
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
	write_out = time4.elapsed().as_nanos() as f64;
	total = time0.elapsed().as_nanos() as f64;

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
