extern crate crossbeam;

const CHUNKS: usize = 10;
const CHUNK_SIZE: usize = 10;

fn main() {

	let mut table = [0; CHUNKS * CHUNK_SIZE];

	// scoped threads let the compiler prove that no threads will 
	// outlive table
	let _ = crossbeam::scope(|scope| {
		// chop the table into disjoint sub-slices
		for slice in table.chunks_mut(CHUNK_SIZE) {
			// spawn a thread operating on that slice
			scope.spawn(move |_| write_slice(slice));
		}
		// crossbeam::scope ensures that all spawned threads join
		// before returning control back from this closure
	});

	// all threads have joined, we have exclusive access to table again

	println!("{:?}", &table[..]);

}

fn write_slice(slice: &mut [i32]) {
	// for a slice, element at i is set to the index of slice (0-9)
	for (i, e) in slice.iter_mut().enumerate() {
		*e = i as i32;
	}
}
