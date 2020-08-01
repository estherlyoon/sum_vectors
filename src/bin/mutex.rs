use std::sync::{Arc, Mutex};
use std::thread;

fn main() {

	let v0 = Arc::new(Mutex::new(vec![1,2,3]));
//	let v1 = vec![10,20,30];
//	let mut vectors = vec![];
//	vectors.push(v0);
//	vectors.push(v1);
	let mut results = Arc::new(Mutex::new(vec![0,0,0]));

	// Arc stands for atomic reference count; primitive type that is shareable across threads
	// Mutex = mutual exclusion, implements locking when accessing data
    let counter = Arc::new(Mutex::new(0)); 
	let mut handles = vec![];

//	for v in vectors {
		for ind in 0..3 {
			// clone results to allow sharing
			let result = Arc::clone(&results);
			let v0_copy = Arc::clone(&v0);
			let handle = thread::spawn(move || {
				// must lock mutex before you can access data
				let mut res = result.lock().unwrap();
				let copyv = v0_copy.lock().unwrap();
				res[ind] += copyv[ind];
			});
			handles.push(handle);
//		}
	}	
	// mutexes automatically unlocked when it goes out of scope

	// make sure all threads have finished
	for handle in handles {
		handle.join().unwrap();
	}

	let end_res = &*results.lock().unwrap();
	for n in 0..3 {
		println!("{}", end_res[n]);
	}	

}

