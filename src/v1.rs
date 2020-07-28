use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::convert::TryInto;
use std::time::Instant;

fn main() -> std::io::Result<()> {

	let time0 = Instant::now();
	let args: Vec<String> = env::args().collect();
	// parse command line args
	let lines = &args[1].parse::<i32>().unwrap();
	let dimension = &args[2].parse::<i32>().unwrap();
	let input_file = &args[3];
	let output_file = &args[4];
	let parseargs = time0.elapsed().as_nanos() as f64;

	let time1 = Instant::now();
	
	// init sum vector and file vars
	let mut sums = vec![0.0 as f32; (*dimension).try_into().unwrap()];

	let mut outfile = File::create(output_file)?;
	
	// read and parse input file 
	let contents = fs::read_to_string(input_file).expect("unable to read file");
	let split: Vec<&str> = contents.split(&[',','\n'][..]).collect();

	let obj_creation = time1.elapsed().as_nanos() as f64;

	let mut index = 0;

	// sequentially add to sum vector
	let time3 = Instant::now();
	for _l in 0..*lines {
		for d in 0..*dimension {
			sums[d as usize] += split[index].parse::<f32>().unwrap();
			index += 1;
		}
	}
	
	let total_line = time3.elapsed().as_nanos() as f64;
	let avg_line = total_line / *lines as f64;	
	
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


