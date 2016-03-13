

fn main() {
    let xs = vec![1, 2, 3];
    std::thread::spawn(|| {
        println!("{:?}", xs);
    });
}
