extern crate crossbeam;

fn main() {
    let mut xs = [0, 0, 0, 0];

    crossbeam::scope(|scope| {
        for i in &mut xs {
            scope.spawn(move || {
                *i += 1; // Stack of another thread!
            });
        }
    });

    println!("{:?}", xs);
}
