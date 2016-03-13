fn main() {
    let xs = std::sync::Mutex::new([0, 0, 0, 0]); // Protect the data
    crossbeam::scope(|scope| {
        for _ in 0..10 {
            scope.spawn(|| {
                let mut guard = xs.lock().unwrap();
                let xs: &mut [i32; 4] = &mut guard; // Can't leak `xs`
                for i in xs {
                    *i += 1;
                }
            });
        }
    });
    println!("{:?}", *xs.lock().unwrap());
}

extern crate crossbeam;
