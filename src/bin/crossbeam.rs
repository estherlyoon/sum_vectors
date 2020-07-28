use std::env;
use crossbeam::atomic::AtomicCell;
use crossbeam::thread;

fn main() {
	let mut a = AtomicCell::new(5);

	thread::scope(|s| {
		s.spawn(|_| {
			a.store(6);
			println!("{}", a.into_inner());
		});
	}).unwrap();

	println!("{}", a.into_inner());
}
