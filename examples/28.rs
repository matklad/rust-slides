

fn main() {
    let xs = vec![1, 2, 3];
    std::thread::spawn(move || {
        println!("{:?}", xs);
    });
    // No `xs` here
}
