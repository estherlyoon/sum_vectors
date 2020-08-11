use std::env;
use std::fs::{read_to_string, File};
use std::io::{BufRead, Write};
use std::convert::TryInto;
use std::time::Instant;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};

extern crate crossbeam;

fn main() -> std::io::Result<()> {

	let time0 = Instant::now();
	
	let args: Vec<String> = env::args().collect();
	let numlines = &args[1].parse::<i32>().unwrap();
	let dimension = &args[2].parse::<i32>().unwrap();
	let input_file = &args[3];
	let output_file = &args[4];
	let parseargs = time0.elapsed().as_nanos() as f64;
	let cpus: i32 = num_cpus::get() as i32;
	
	let time1 = Instant::now();

	let mut sums = Arc::new(Mutex::new(vec![0.0; (*dimension).try_into().unwrap()]));
	let mut outfile = File::create(output_file)?;
	let contents = read_to_string(input_file).expect("error reading file");
	let spl = contents.split("\n");
	let mut lines = Arc::new(Mutex::new(vec![]));
	
	let lines_clone = Arc::clone(&lines);

	for s in spl {
		let str = Arc::new(Mutex::new(String::from(s)));
		let str_clone = Arc::clone(&str);
		let mut str_shared = str_clone.lock().unwrap();
		let mut lines_shared = lines_clone.lock().unwrap();
		lines_shared.push(s);
	}

	let obj_creation = time1.elapsed().as_nanos() as f64;

	let time2 = Instant::now();
	crossbeam::scope(|scope| {
		for cpu in 0..cpus {			
			let sums_clone = Arc::clone(&sums);
			let lines_clone = Arc::clone(&lines);
			
			scope.spawn(move |_| {		
				let start = cpu * (dimension / cpus);
				let end = if cpu == cpus-1 {*dimension} else {start + (*dimension / cpus)}; 
				let lines_sh = lines_clone.lock().unwrap();
				let mut sums_sh = sums_clone.lock().unwrap();
				
				for i in start..end {
					let split = lines_sh[i as usize].split(",");
					let mut index = 0;
					for s in split {
						sums_sh[index] += s.parse::<f32>().unwrap();
						index += 1;
					}
				}
			});
		}
	});	

	let total_line = time2.elapsed().as_nanos() as f64;
		
	let time4 = Instant::now();

	let sums1 = sums.lock().unwrap();
	for n in 0..*dimension {
		let str = sums1[n as usize].to_string() + " ";
		outfile.write_all(str.as_bytes()).expect("can't write to output file");
	}

	let write_out = time4.elapsed().as_nanos() as f64;
	let total = time0.elapsed().as_nanos() as f64;

	println!("time to parse args: {} ({:.2}%)", parseargs, parseargs / total * 100.0);
	println!("time to create vector / files: {} ({:.2}%)", obj_creation, obj_creation / total * 100.0);
	println!("total time for summing: {} ({:.2}%)", total_line, total_line / total * 100.0);	
	println!("time to write to output file: {} ({:.2}%)", write_out, write_out / total * 100.0);
	println!("total time: {}", total);
	
	Ok(())
}

