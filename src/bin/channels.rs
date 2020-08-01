use std::time::Duration;
use std::thread;
use std::sync::mpsc;
//use crossbeam::atomic::AtomicCell;
//use crossbeam::thread;

fn main() {

	// creates channel with transmitting and receiving ends
	let (tx, rx) = mpsc::channel();

	let tx1 = mpsc::Sender::clone(&tx);

	thread::spawn(move || {
		let val = String::from("yo");
		tx1.send(val).unwrap();
		thread::sleep(Duration::from_secs(1));
	});	

	// move gives ownership of objects inside to this thread
	let handle = thread::spawn(move || {
		let val = String::from("hi");
		tx.send(val).unwrap(); // unwrap panics incase there's an error (eg receiver closed)
	});

	/*for i in 1..5 {
		println!("main thread {}", i);
		thread::sleep(Duration::from_millis(1));
	}
	*/

	// recv blocks main channel until something is sent.. try_recv doesn't block (good if other
	// work needs to be done)
	//let received = rx.recv().unwrap();
	//println!("Got {}", received);
	
	for received in rx {
		println!("Got {}", received);
	}

	// blocks the thread currently running until the handle thread finishes
	handle.join().unwrap();
	
}
