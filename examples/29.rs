use std::sync::mpsc::channel;

fn main() {
    let (tx, rx) = channel();

    std::thread::spawn(move || {
        let xs = rx.recv().unwrap();
        println!("{:?}", xs);
    });

    let xs = vec![1, 2, 3];
    tx.send(xs).unwrap(); // No copy here!
}
